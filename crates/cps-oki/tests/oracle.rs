// Oracle OKI MSM6295 : rejoue les 7 scénarios de tools/oki_golden.cpp et compare
// aux vecteurs MAME dans tests-data/oki/. Format : 2000 échantillons i32 LE +
// <nom>_status.bin (2000 octets). Convention : avant l'échantillon n on applique
// les commandes planifiées "avant n", puis génération.

use cps_oki::okim6295::Okim6295;
use std::fs;
use std::path::PathBuf;

/// ROM oracle : rom(a) = (h ^ (h >> 15)) & 0xFF, h = a * 0x9E3779B1 (u32),
/// SAUF la table des échantillons (a < 0x400).
fn build_rom() -> Vec<u8> {
    let mut rom = vec![0u8; 0x40000];

    // Table d'échantillons : a < 0x400
    for s in 1..=127 {
        let start: u32 = 0x400 + (s as u32) * 0x100;
        let stop: u32 = if s == 5 {
            start
        } else {
            start + 0x7F + (s as u32)
        };

        // Octets 0-2 : start address (big-endian, masked to 18 bits)
        rom[s * 8] = ((start >> 16) & 0xFF) as u8;
        rom[s * 8 + 1] = ((start >> 8) & 0xFF) as u8;
        rom[s * 8 + 2] = (start & 0xFF) as u8;

        // Octets 3-5 : stop address
        rom[s * 8 + 3] = ((stop >> 16) & 0xFF) as u8;
        rom[s * 8 + 4] = ((stop >> 8) & 0xFF) as u8;
        rom[s * 8 + 5] = (stop & 0xFF) as u8;

        // Octets 6-7 : 0
    }

    // Hachage pour le reste de la ROM
    for (a, byte) in rom.iter_mut().enumerate() {
        if a < 0x400 {
            continue;
        }
        let h: u32 = (a as u32).wrapping_mul(0x9E3779B1);
        *byte = ((h ^ (h >> 15)) & 0xFF) as u8;
    }

    rom
}

/// Commande planifiée : avant l'échantillon `before`, écrire `val`.
struct Cmd {
    before: i32,
    val: u8,
}

fn run_scenario(_name: &str, cmds: Vec<Cmd>) -> (Vec<i32>, Vec<u8>) {
    let rom = build_rom();
    let mut chip = Okim6295::new(rom);
    let mut out = Vec::with_capacity(2000);
    let mut st = Vec::with_capacity(2000);

    let mut ci = 0;
    for k in 1..=2000 {
        // Appliquer les commandes planifiées "avant k"
        while ci < cmds.len() && cmds[ci].before <= k {
            chip.write(cmds[ci].val);
            ci += 1;
        }

        out.push(chip.generate());
        st.push(chip.read());
    }

    (out, st)
}

/// Resolve path relative to workspace root (two levels up from crate tests dir).
fn resolve_path(name: &str) -> PathBuf {
    // cargo test runs from crates/cps-oki, so go up two levels
    let mut p = std::env::current_dir().expect("Cannot get cwd");
    p.pop(); // crates/
    p.pop(); // project root
    p.join(name)
}

fn load_i32(path: &str) -> Vec<i32> {
    let full_path = resolve_path(path);
    let data =
        fs::read(&full_path).unwrap_or_else(|_| panic!("Cannot read {}", full_path.display()));
    assert_eq!(data.len(), 8000, "Expected 8000 bytes (2000 i32)");
    let mut out = Vec::with_capacity(2000);
    for i in 0..2000 {
        let offset = i * 4;
        let val = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        out.push(val);
    }
    out
}

fn load_u8(path: &str) -> Vec<u8> {
    let full_path = resolve_path(path);
    let data =
        fs::read(&full_path).unwrap_or_else(|_| panic!("Cannot read {}", full_path.display()));
    assert_eq!(data.len(), 2000, "Expected 2000 bytes");
    data
}

fn compare(name: &str, expected_out: &[i32], actual_out: &[i32]) -> bool {
    if expected_out.len() != actual_out.len() {
        println!(
            "{}: length mismatch {} vs {}",
            name,
            expected_out.len(),
            actual_out.len()
        );
        return false;
    }

    let mut mismatches = 0;
    for (i, (e, a)) in expected_out.iter().zip(actual_out.iter()).enumerate() {
        if e != a {
            if mismatches < 5 {
                println!("{}: sample {} expected {} got {}", name, i, e, a);
            }
            mismatches += 1;
        }
    }

    if mismatches > 0 {
        println!("{}: {} mismatches", name, mismatches);
    }

    mismatches == 0
}

fn compare_status(name: &str, expected: &[u8], actual: &[u8]) -> bool {
    let mut mismatches = 0;
    for (i, (e, a)) in expected.iter().zip(actual.iter()).enumerate() {
        if e != a {
            if mismatches < 5 {
                println!(
                    "{} status: sample {} expected {:02X} got {:02X}",
                    name, i, e, a
                );
            }
            mismatches += 1;
        }
    }

    if mismatches > 0 {
        println!("{} status: {} mismatches", name, mismatches);
    }

    mismatches == 0
}

fn test_scenario(name: &str, cmds: Vec<Cmd>) -> bool {
    let (out, st) = run_scenario(name, cmds);

    let base = PathBuf::from("tests-data/oki");
    let out_path = base.join(format!("{}.bin", name));
    let st_path = base.join(format!("{}_status.bin", name));

    let expected_out = load_i32(&out_path.to_string_lossy());
    let expected_st = load_u8(&st_path.to_string_lossy());

    let out_ok = compare(name, &expected_out, &out);
    let st_ok = compare_status(name, &expected_st, &st);

    if out_ok && st_ok {
        println!("{}: PASS", name);
    } else {
        println!("{}: FAIL", name);
    }

    out_ok && st_ok
}

#[test]
fn oracle_silence() {
    assert!(test_scenario("silence", vec![]));
}

#[test]
fn oracle_voice1() {
    // Échantillon 1, voix 0, volume 0 dB (index 0)
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x81,
        },
        Cmd {
            before: 1,
            val: 0x10,
        },
    ];
    assert!(test_scenario("voice1", cmds));
}

#[test]
fn oracle_two_voices() {
    // Échantillon 2 voix 1 vol 2 ; échantillon 3 voix 2 vol 4
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x82,
        },
        Cmd {
            before: 1,
            val: 0x22,
        },
        Cmd {
            before: 50,
            val: 0x83,
        },
        Cmd {
            before: 50,
            val: 0x44,
        },
    ];
    assert!(test_scenario("two_voices", cmds));
}

#[test]
fn oracle_stop() {
    // Échantillon 4 voix 3 vol 0 ; arrêt voix 0 (0x08 = bit 3 -> voix 0)
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x84,
        },
        Cmd {
            before: 1,
            val: 0x80,
        },
        Cmd {
            before: 100,
            val: 0x08,
        },
    ];
    assert!(test_scenario("stop", cmds));
}

#[test]
fn oracle_busy_ignore() {
    // 2e départ sur voix 0 occupée : ignoré
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x81,
        },
        Cmd {
            before: 1,
            val: 0x10,
        },
        Cmd {
            before: 10,
            val: 0x82,
        },
        Cmd {
            before: 10,
            val: 0x10,
        },
    ];
    assert!(test_scenario("busy_ignore", cmds));
}

#[test]
fn oracle_invalid() {
    // Échantillon 5 : start == stop -> rien
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x85,
        },
        Cmd {
            before: 1,
            val: 0x10,
        },
    ];
    assert!(test_scenario("invalid", cmds));
}

#[test]
fn oracle_all4() {
    // Toutes les voix, puis arrêt global à l'échantillon 300
    let cmds = vec![
        Cmd {
            before: 1,
            val: 0x81,
        },
        Cmd {
            before: 1,
            val: 0x10,
        },
        Cmd {
            before: 1,
            val: 0x86,
        },
        Cmd {
            before: 1,
            val: 0x21,
        },
        Cmd {
            before: 1,
            val: 0x87,
        },
        Cmd {
            before: 1,
            val: 0x43,
        },
        Cmd {
            before: 1,
            val: 0x88,
        },
        Cmd {
            before: 1,
            val: 0x88,
        },
        Cmd {
            before: 300,
            val: 0x78,
        },
    ];
    assert!(test_scenario("all4", cmds));
}

#[test]
fn oracle_all() {
    // Exécute tous les scénarios et écrit metrics/oki.json
    let scenarios = vec![
        ("silence", vec![]),
        (
            "voice1",
            vec![
                Cmd {
                    before: 1,
                    val: 0x81,
                },
                Cmd {
                    before: 1,
                    val: 0x10,
                },
            ],
        ),
        (
            "two_voices",
            vec![
                Cmd {
                    before: 1,
                    val: 0x82,
                },
                Cmd {
                    before: 1,
                    val: 0x22,
                },
                Cmd {
                    before: 50,
                    val: 0x83,
                },
                Cmd {
                    before: 50,
                    val: 0x44,
                },
            ],
        ),
        (
            "stop",
            vec![
                Cmd {
                    before: 1,
                    val: 0x84,
                },
                Cmd {
                    before: 1,
                    val: 0x80,
                },
                Cmd {
                    before: 100,
                    val: 0x08,
                },
            ],
        ),
        (
            "busy_ignore",
            vec![
                Cmd {
                    before: 1,
                    val: 0x81,
                },
                Cmd {
                    before: 1,
                    val: 0x10,
                },
                Cmd {
                    before: 10,
                    val: 0x82,
                },
                Cmd {
                    before: 10,
                    val: 0x10,
                },
            ],
        ),
        (
            "invalid",
            vec![
                Cmd {
                    before: 1,
                    val: 0x85,
                },
                Cmd {
                    before: 1,
                    val: 0x10,
                },
            ],
        ),
        (
            "all4",
            vec![
                Cmd {
                    before: 1,
                    val: 0x81,
                },
                Cmd {
                    before: 1,
                    val: 0x10,
                },
                Cmd {
                    before: 1,
                    val: 0x86,
                },
                Cmd {
                    before: 1,
                    val: 0x21,
                },
                Cmd {
                    before: 1,
                    val: 0x87,
                },
                Cmd {
                    before: 1,
                    val: 0x43,
                },
                Cmd {
                    before: 1,
                    val: 0x88,
                },
                Cmd {
                    before: 1,
                    val: 0x88,
                },
                Cmd {
                    before: 300,
                    val: 0x78,
                },
            ],
        ),
    ];

    let mut pass = 0;
    let mut fail = 0;

    for (name, cmds) in scenarios {
        if test_scenario(name, cmds) {
            pass += 1;
        } else {
            fail += 1;
        }
    }

    // Écrire metrics/oki.json
    let json = format!(r#"{{"pass": {}, "fail": {}, "total": 7}}"#, pass, fail);
    let metrics_path = resolve_path("metrics/oki.json");
    fs::write(&metrics_path, &json).expect("Cannot write metrics/oki.json");
    println!("Metrics written: {}", json);

    assert_eq!(fail, 0, "{} scenarios failed", fail);
}
