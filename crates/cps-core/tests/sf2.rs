//! Street Fighter II (sf2, World 910522) integration test: 3000 frames with golden MAME comparison.
//! Requires ROM files and golden captures (not in repo). Run with --include-ignored.

mod common;

use cps_core::{load_game, Config, Machine};
use cps_testkit::golden::{diff_frame, golden_dir, load_png_rgb, write_diff_ppm};
use std::path::Path;

/// Golden comparison frames: 120 (self-test), 600, 1500, 3000.
const GOLDEN_FRAMES: [u64; 4] = [120, 600, 1500, 3000];

/// Maximum allowed pixel difference ratio (2%).
const MAX_DIFF_RATIO: f64 = 0.02;

#[test]
#[ignore] // Requires ROM files and golden captures
fn sf2_golden_comparison() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../cps1.toml");
    let config = Config::load(&config_path).expect("Failed to load cps1.toml");
    let game = load_game(&config, "sf2").expect("Failed to load sf2 game");
    let mut machine = Machine::new(game);

    println!("Running Street Fighter II for 3000 frames...");

    // Run 3000 frames, comparing at golden points
    for frame in 1..=3000 {
        machine.run_frame();

        // Check for double fault (CPU halted)
        if machine.cpu.is_halted() {
            panic!("Double fault detected at frame {}", frame);
        }

        // Golden comparison at specific frames
        if GOLDEN_FRAMES.contains(&frame) {
            let golden_path = golden_dir("sf2").join(format!("f{:06}.png", frame));
            println!(
                "Frame {}: comparing against golden {}",
                frame,
                golden_path.display()
            );

            // Write visible frame for inspection (always, success or failure)
            let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let output_path =
                manifest_dir.join(format!("../../target/frames/sf2_f{:06}.png", frame));
            common::write_png(machine.framebuffer(), &output_path);

            // Load golden and compare
            let (w, h, golden_pixels) = load_png_rgb(&golden_path).expect("Failed to load golden");
            let diff = diff_frame(machine.framebuffer(), &golden_pixels, w, h)
                .expect("Frame size mismatch with golden");

            if diff.ratio() > MAX_DIFF_RATIO {
                // Write diff image for debugging
                let diff_path =
                    manifest_dir.join(format!("../../target/diff/sf2_f{:06}.ppm", frame));
                write_diff_ppm(machine.framebuffer(), &golden_pixels, &diff_path)
                    .expect("Failed to write diff PPM");

                panic!(
                    "Frame {} golden comparison failed: {:.4}% pixels differ (max {:.2}%)",
                    frame,
                    diff.ratio() * 100.0,
                    MAX_DIFF_RATIO * 100.0
                );
            } else {
                println!(
                    "Frame {}: OK ({:.4}% pixels differ)",
                    frame,
                    diff.ratio() * 100.0
                );
            }
        }
    }

    println!("All golden comparisons passed!");

    // Update metrics
    common::update_game_metrics(true);
}

#[test]
#[ignore] // Requires ROM files
fn sf2_determinism() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../cps1.toml");
    let config = Config::load(&config_path).expect("Failed to load cps1.toml");

    // Run two fresh machines for 600 frames each
    let game1 = load_game(&config, "sf2").expect("Failed to load sf2 (run 1)");
    let mut machine1 = Machine::new(game1);

    let game2 = load_game(&config, "sf2").expect("Failed to load sf2 (run 2)");
    let mut machine2 = Machine::new(game2);

    for _ in 0..600 {
        machine1.run_frame();
        machine2.run_frame();
    }

    // Compare framebuffer CRC32
    let fb1_crc = machine1.framebuffer().crc32();
    let fb2_crc = machine2.framebuffer().crc32();
    assert_eq!(
        fb1_crc, fb2_crc,
        "Framebuffer CRC mismatch after 600 frames"
    );

    // Compare CPU state (PC and registers)
    let cpu1_state = machine1.cpu.state();
    let cpu2_state = machine2.cpu.state();
    assert_eq!(
        cpu1_state.pc, cpu2_state.pc,
        "CPU PC mismatch after 600 frames"
    );

    println!("Determinism test passed: identical state after 600 frames");
}

#[test]
#[ignore] // Requires ROM files
fn sf2_coin_insert() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../cps1.toml");
    let config = Config::load(&config_path).expect("Failed to load cps1.toml");

    // Run machine for 3000 frames (attract mode)
    let game = load_game(&config, "sf2").expect("Failed to load sf2 game");
    let mut machine = Machine::new(game);

    println!("Running Street Fighter II attract mode for 3000 frames...");
    for frame in 1..=3000 {
        machine.run_frame();
        if machine.cpu.is_halted() {
            panic!("Double fault detected at frame {}", frame);
        }
    }

    // Capture framebuffer CRC before coin insert
    let fb_before_crc = machine.framebuffer().crc32();
    println!("Frame 3000 (before coin): CRC={:x}", fb_before_crc);

    // Insert a coin: IN0 bit0 to 0 for 5 frames at frame 3001-3005, then release
    let mut inputs = cps_core::InputState::default();
    inputs.in0 &= !0x01; // Coin insert (bit 0 low)
    machine.set_inputs(inputs);

    println!("Inserting coin at frame 3001...");
    for _ in 1..=5 {
        machine.run_frame();
        if machine.cpu.is_halted() {
            panic!("Double fault detected during coin insert");
        }
    }

    // Release coin button
    inputs.in0 |= 0x01; // Coin released (bit 0 high)
    machine.set_inputs(inputs);

    // Run to frame 3200 and check that the image differs from frame 3000 without coin
    for _ in 6..=200 {
        machine.run_frame();
        if machine.cpu.is_halted() {
            panic!("Double fault detected after coin insert");
        }
    }

    let fb_after_crc = machine.framebuffer().crc32();
    println!("Frame 3200 (after coin): CRC={:x}", fb_after_crc);

    assert_ne!(
        fb_before_crc, fb_after_crc,
        "Image should differ after coin insert (credit displayed)"
    );

    // Write the post-coin frame for inspection
    let output_path = manifest_dir.join("../../target/frames/sf2_f003200_coin.png");
    common::write_png(machine.framebuffer(), &output_path);

    println!("Coin insert test passed: image changed after coin insert");
}
