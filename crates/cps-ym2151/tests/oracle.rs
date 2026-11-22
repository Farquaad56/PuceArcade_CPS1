// Oracle YM2151 : comparaison aux vecteurs ymfm (tests-data/ym2151)
// Reproduit les 7 scénarios de tools/ymfm_opm_golden.cpp

use std::fs;
use std::path::Path;

fn repo_root() -> String {
    // Compute repo root from this file's location: crates/cps-ym2151/tests/oracle.rs
    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    let path = Path::new(manifest_dir);
    path.parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

fn read_i16_le(path: &str) -> Vec<i16> {
    let full_path = format!("{}/{}", repo_root(), path);
    let data = fs::read(&full_path).unwrap_or_else(|_| panic!("Cannot read {}", full_path));
    let mut result = Vec::with_capacity(data.len() / 2);
    for i in (0..data.len()).step_by(2) {
        let lo = data[i] as i16;
        let hi = data[i + 1] as i8 as i16;
        result.push((hi << 8) | lo);
    }
    result
}

fn read_u8(path: &str) -> Vec<u8> {
    let full_path = format!("{}/{}", repo_root(), path);
    fs::read(&full_path).unwrap_or_else(|_| panic!("Cannot read {}", full_path))
}

struct WriteOp {
    before: i32,
    offset: u8,
    value: u8,
}

fn r(writes: &mut Vec<WriteOp>, before: i32, reg: u8, val: u8) {
    writes.push(WriteOp {
        before,
        offset: 0x00,
        value: reg,
    });
    writes.push(WriteOp {
        before,
        offset: 0x01,
        value: val,
    });
}

fn voice(writes: &mut Vec<WriteOp>, before: i32, ch: u8, algo: u8, fb: u8, tl: u8, kc: u8) {
    r(writes, before, 0x20 + ch, 0xC0 | (fb << 3) | algo);
    r(writes, before, 0x28 + ch, kc);
    r(writes, before, 0x30 + ch, 0x00);
    r(writes, before, 0x38 + ch, 0x00);
    for op in 0..4 {
        let o = op * 8 + ch;
        r(writes, before, 0x40 + o, 0x01 + op);
        r(writes, before, 0x60 + o, tl + op * 4);
        r(writes, before, 0x80 + o, 0x1F);
        r(writes, before, 0xA0 + o, 0x05);
        r(writes, before, 0xC0 + o, 0x02);
        r(writes, before, 0xE0 + o, 0x24);
    }
}

fn run_scenario(name: &str, writes: Vec<WriteOp>, expect_status: bool) -> (bool, String) {
    let mut clocked = cps_ym2151::clocked::ClockedYm2151::new();

    let golden_path = format!("tests-data/ym2151/{}.bin", name);
    let expected = read_i16_le(&golden_path);

    let mut status_expected: Option<Vec<u8>> = None;
    if expect_status {
        status_expected = Some(read_u8(&format!("tests-data/ym2151/{}_status.bin", name)));
    }

    let mut irq_expected: Option<Vec<u8>> = None;
    if expect_status {
        irq_expected = Some(read_u8(&format!("tests-data/ym2151/{}_irq.bin", name)));
    }

    let mut actual_samples = Vec::with_capacity(1000);
    let mut actual_status = Vec::with_capacity(1000);
    let mut actual_irq = Vec::with_capacity(1000);

    let mut wi = 0;
    for n in 1..=1000 {
        clocked.advance_to(((n - 1) as u64) * 64);

        while wi < writes.len() && writes[wi].before <= n {
            clocked.write(writes[wi].offset, writes[wi].value);
            wi += 1;
        }

        clocked.advance_to((n as u64) * 64);
        let samples = clocked.take_samples();

        if samples.is_empty() {
            return (false, format!("{}: no sample at n={}", name, n));
        }

        for s in &samples {
            actual_samples.push(s[0] as i16);
            actual_samples.push(s[1] as i16);
        }

        if expect_status {
            let status = clocked.read(0x01);
            actual_status.push(status);
            actual_irq.push(if clocked.irq() { 1 } else { 0 });
        }
    }

    // Compare samples
    let mut mismatches = 0;
    for (i, (exp, act)) in expected.iter().zip(actual_samples.iter()).enumerate() {
        if exp != act && mismatches < 5 {
            let ch = i % 2;
            let n = i / 2 + 1;
            eprintln!(
                "DEBUG {}: sample {} ch={} expected {} got {}",
                name, n, ch, exp, act
            );
            mismatches += 1;
        }
    }
    if mismatches > 0 {
        return (false, format!("{}: {} sample mismatches", name, mismatches));
    }

    // Compare status if expected
    if let Some(ref exp_status) = status_expected {
        for (i, (exp, act)) in exp_status.iter().zip(actual_status.iter()).enumerate() {
            if exp != act {
                return (
                    false,
                    format!(
                        "{}: status at n={} expected {:02X} got {:02X}",
                        name,
                        i + 1,
                        exp,
                        act
                    ),
                );
            }
        }
    }

    // Compare IRQ if expected
    if let Some(ref exp_irq) = irq_expected {
        for (i, (exp, act)) in exp_irq.iter().zip(actual_irq.iter()).enumerate() {
            if exp != act {
                return (
                    false,
                    format!("{}: irq at n={} expected {} got {}", name, i + 1, exp, act),
                );
            }
        }
    }

    (true, String::new())
}

#[test]
fn oracle_silence() {
    let result = run_scenario("silence", vec![], false);
    assert!(result.0, "silence failed: {}", result.1);
}

#[test]
fn oracle_fm_algo7() {
    let mut writes = Vec::new();
    voice(&mut writes, 1, 0, 7, 0, 0x10, 0x4A);
    r(&mut writes, 2, 0x08, 0x78);
    let result = run_scenario("fm_algo7", writes, false);
    assert!(result.0, "fm_algo7 failed: {}", result.1);
}

#[test]
fn oracle_fm_algo4_keyoff() {
    let mut writes = Vec::new();
    voice(&mut writes, 1, 3, 4, 5, 0x08, 0x3E);
    r(&mut writes, 2, 0x08, 0x78 | 3);
    r(&mut writes, 600, 0x08, 3);
    let result = run_scenario("fm_algo4_keyoff", writes, false);
    assert!(result.0, "fm_algo4_keyoff failed: {}", result.1);
}

#[test]
fn oracle_lfo() {
    let mut writes = Vec::new();
    voice(&mut writes, 1, 1, 0, 7, 0x14, 0x5C);
    r(&mut writes, 1, 0x18, 0xC0);
    r(&mut writes, 1, 0x19, 0x7F);
    r(&mut writes, 1, 0x19, 0xFF);
    r(&mut writes, 1, 0x1B, 0x02);
    r(&mut writes, 1, 0x38 + 1, 0x73);
    r(&mut writes, 2, 0x08, 0x78 | 1);
    let result = run_scenario("lfo", writes, false);
    assert!(result.0, "lfo failed: {}", result.1);
}

#[test]
fn oracle_noise() {
    let mut writes = Vec::new();
    voice(&mut writes, 1, 7, 7, 0, 0x10, 0x40);
    r(&mut writes, 1, 0x0F, 0x80 | 0x10);
    r(&mut writes, 2, 0x08, 0x78 | 7);
    let result = run_scenario("noise", writes, false);
    assert!(result.0, "noise failed: {}", result.1);
}

#[test]
fn oracle_timers() {
    let mut writes = Vec::new();
    r(&mut writes, 1, 0x10, 0xFF);
    r(&mut writes, 1, 0x11, 0x00);
    r(&mut writes, 1, 0x12, 0xF0);
    r(&mut writes, 1, 0x14, 0x3F);
    r(&mut writes, 400, 0x14, 0x15);
    r(&mut writes, 700, 0x14, 0x30);
    let result = run_scenario("timers", writes, true);
    assert!(result.0, "timers failed: {}", result.1);
}

#[test]
fn oracle_mix() {
    let mut writes = Vec::new();
    for ch in 0..8 {
        voice(&mut writes, 1, ch, ch, ch & 7, 0x18, 0x20 + ch * 9);
        r(&mut writes, 2 + (ch as i32) * 50, 0x08, 0x78 | ch);
    }
    let result = run_scenario("mix", writes, false);
    assert!(result.0, "mix failed: {}", result.1);
}

fn run_all_scenarios() -> (u32, u32) {
    let mut pass = 0;
    let mut fail = 0;

    // silence
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("silence", vec![], false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: silence - {}", result.1);
        }
    }

    // fm_algo7
    let mut w = Vec::new();
    voice(&mut w, 1, 0, 7, 0, 0x10, 0x4A);
    r(&mut w, 2, 0x08, 0x78);
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("fm_algo7", w, false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: fm_algo7 - {}", result.1);
        }
    }

    // fm_algo4_keyoff
    let mut w = Vec::new();
    voice(&mut w, 1, 3, 4, 5, 0x08, 0x3E);
    r(&mut w, 2, 0x08, 0x78 | 3);
    r(&mut w, 600, 0x08, 3);
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("fm_algo4_keyoff", w, false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: fm_algo4_keyoff - {}", result.1);
        }
    }

    // lfo
    let mut w = Vec::new();
    voice(&mut w, 1, 1, 0, 7, 0x14, 0x5C);
    r(&mut w, 1, 0x18, 0xC0);
    r(&mut w, 1, 0x19, 0x7F);
    r(&mut w, 1, 0x19, 0xFF);
    r(&mut w, 1, 0x1B, 0x02);
    r(&mut w, 1, 0x38 + 1, 0x73);
    r(&mut w, 2, 0x08, 0x78 | 1);
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("lfo", w, false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: lfo - {}", result.1);
        }
    }

    // noise
    let mut w = Vec::new();
    voice(&mut w, 1, 7, 7, 0, 0x10, 0x40);
    r(&mut w, 1, 0x0F, 0x80 | 0x10);
    r(&mut w, 2, 0x08, 0x78 | 7);
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("noise", w, false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: noise - {}", result.1);
        }
    }

    // timers
    let mut w = Vec::new();
    r(&mut w, 1, 0x10, 0xFF);
    r(&mut w, 1, 0x11, 0x00);
    r(&mut w, 1, 0x12, 0xF0);
    r(&mut w, 1, 0x14, 0x3F);
    r(&mut w, 400, 0x14, 0x15);
    r(&mut w, 700, 0x14, 0x30);
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("timers", w, true)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: timers - {}", result.1);
        }
    }

    // mix
    let mut w = Vec::new();
    for ch in 0..8 {
        voice(&mut w, 1, ch, ch, ch & 7, 0x18, 0x20 + ch * 9);
        r(&mut w, 2 + (ch as i32) * 50, 0x08, 0x78 | ch);
    }
    if let Ok(result) = std::panic::catch_unwind(|| run_scenario("mix", w, false)) {
        if result.0 {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: mix - {}", result.1);
        }
    }

    (pass, fail)
}

#[test]
fn write_metrics() {
    let (pass, fail) = run_all_scenarios();

    // Write metrics JSON
    let json = format!(r#"{{"pass": {}, "fail": {}, "total": 7}}"#, pass, fail);
    let metrics_path = format!("{}/metrics/ym2151.json", repo_root());
    fs::write(&metrics_path, &json).expect("Cannot write metrics");

    assert_eq!(fail, 0, "{} scenarios failed", fail);
}
