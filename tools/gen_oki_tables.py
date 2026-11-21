#!/usr/bin/env python3
"""Génère crates/cps-oki/src/tables.rs : table DIFF_LOOKUP[49*16] d'oki_adpcm_state::compute_tables (ref/mame-sound/okiadpcm.cpp l.111-145).
stepval = floor(16.0 * pow(11/10, step)) en double IEEE, comme le C++. Usage : python tools/gen_oki_tables.py [--check]"""
import math, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "crates/cps-oki/src/tables.rs"
NBL2BIT = [[1,0,0,0],[1,0,0,1],[1,0,1,0],[1,0,1,1],[1,1,0,0],[1,1,0,1],[1,1,1,0],[1,1,1,1],
           [-1,0,0,0],[-1,0,0,1],[-1,0,1,0],[-1,0,1,1],[-1,1,0,0],[-1,1,0,1],[-1,1,1,0],[-1,1,1,1]]
def c_div(a, b):  # division entière C (troncature vers 0) ; ici a >= 0
    return a // b
vals = []
for step in range(49):
    stepval = math.floor(16.0 * math.pow(11.0 / 10.0, float(step)))
    for nib in range(16):
        n = NBL2BIT[nib]
        vals.append(n[0] * (stepval * n[1] + c_div(stepval, 2) * n[2] + c_div(stepval, 4) * n[3] + c_div(stepval, 8)))
lines = ["// Fichier GÉNÉRÉ par tools/gen_oki_tables.py — NE PAS MODIFIER À LA MAIN.",
         "// license:BSD-3-Clause (algorithme MAME okiadpcm.cpp, copyright-holders: Andrew Gardner, Aaron Giles)",
         "//! Table des différences ADPCM OKI (49 pas × 16 nibbles). spec: docs/spec/03 §OKI", "",
         "/// Décalage d'index de pas par nibble (bits 0-2) : okiadpcm.cpp `s_index_shift`.",
         "pub const INDEX_SHIFT: [i32; 8] = [-1, -1, -1, -1, 2, 4, 6, 8];", "",
         "/// `s_diff_lookup[step * 16 + nibble]`.", "pub const DIFF_LOOKUP: [i32; 784] = ["]
for i in range(0, 784, 16):
    lines.append("    " + ", ".join(str(v) for v in vals[i:i+16]) + ",")
lines.append("];")
text = "\n".join(lines) + "\n"
if "--check" in sys.argv:
    sys.exit(0 if OUT.exists() and OUT.read_text(encoding="utf-8") == text else "tables.rs n'est pas à jour")
OUT.write_text(text, encoding="utf-8", newline="\n"); print(OUT.relative_to(ROOT), "écrit ; DIFF_LOOKUP[16*48+7] =", vals[48*16+7])
