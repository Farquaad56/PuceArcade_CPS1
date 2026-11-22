//! Ghouls'n Ghosts integration test: 3000 frames with golden MAME comparison.
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
fn ghouls_golden_comparison() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../cps1.toml");
    let config = Config::load(&config_path).expect("Failed to load cps1.toml");
    let game = load_game(&config, "ghouls").expect("Failed to load ghouls game");
    let mut machine = Machine::new(game);

    println!("Running Ghouls'n Ghosts for 3000 frames...");

    // Run 3000 frames, comparing at golden points
    for frame in 1..=3000 {
        machine.run_frame();

        // Check for double fault (CPU halted)
        if machine.cpu.is_halted() {
            panic!("Double fault detected at frame {}", frame);
        }

        // Golden comparison at specific frames
        if GOLDEN_FRAMES.contains(&frame) {
            let golden_path = golden_dir("ghouls").join(format!("f{:06}.png", frame));
            println!(
                "Frame {}: comparing against golden {}",
                frame,
                golden_path.display()
            );

            // Write visible frame for inspection (always, success or failure)
            let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let output_path =
                manifest_dir.join(format!("../../target/frames/ghouls_f{:06}.png", frame));
            common::write_png(machine.framebuffer(), &output_path);

            // Load golden and compare
            let (w, h, golden_pixels) = load_png_rgb(&golden_path).expect("Failed to load golden");
            let diff = diff_frame(machine.framebuffer(), &golden_pixels, w, h)
                .expect("Frame size mismatch with golden");

            if diff.ratio() > MAX_DIFF_RATIO {
                // Write diff image for debugging
                let diff_path =
                    manifest_dir.join(format!("../../target/diff/ghouls_f{:06}.ppm", frame));
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
fn ghouls_determinism() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../cps1.toml");
    let config = Config::load(&config_path).expect("Failed to load cps1.toml");

    // Run two fresh machines for 600 frames each
    let game1 = load_game(&config, "ghouls").expect("Failed to load ghouls (run 1)");
    let mut machine1 = Machine::new(game1);

    let game2 = load_game(&config, "ghouls").expect("Failed to load ghouls (run 2)");
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
