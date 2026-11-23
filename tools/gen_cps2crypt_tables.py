#!/usr/bin/env python3
"""Génère crates/cps-cart/src/cps2crypt_tables.rs depuis ref/mame-cps2/cps2crypt.cpp (MAME 0.287, BSD-3) :
groupes de bits des deux réseaux de Feistel et les 32 boîtes de substitution (table, entrées, sorties). spec: docs/spec/05 §Chiffrement
Usage : python tools/gen_cps2crypt_tables.py [--check]"""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "ref/mame-cps2/cps2crypt.cpp"
OUT = ROOT / "crates/cps-cart/src/cps2crypt_tables.rs"

def ints(s):
    return [int(x) for x in re.findall(r"-?\d+", s)]

def main():
    src = re.sub(r"//.*", "", SRC.read_text(encoding="utf-8"))
    out = ["// Fichier GÉNÉRÉ par tools/gen_cps2crypt_tables.py depuis ref/mame-cps2/cps2crypt.cpp — NE PAS MODIFIER À LA MAIN.",
           "// license:BSD-3-Clause",
           "// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald",
           "//! Tables du chiffrement CPS2 : groupes de bits des réseaux de Feistel et boîtes de substitution. spec: docs/spec/05 §Chiffrement",
           "",
           "/// Boîte de substitution (struct sbox, l.139-157) : 6 entrées (-1 = clé seule), 2 sorties.",
           "pub struct SBox {",
           "    pub table: [u8; 64],",
           "    pub inputs: [i8; 6],",
           "    pub outputs: [u8; 2],",
           "}",
           ""]
    for name in ["fn1_groupA", "fn1_groupB", "fn2_groupA", "fn2_groupB"]:
        m = re.search(r"const int %s\[8\]\s*=\s*\{([^}]*)\}" % name, src)
        v = ints(m.group(1)); assert len(v) == 8
        const = name.replace("_groupA", "_group_a").replace("_groupB", "_group_b").upper()
        out.append(f"pub const {const}: [u8; 8] = {v};")
    out.append("")
    for net in (1, 2):
        for rnd in (1, 2, 3, 4):
            name = f"fn{net}_r{rnd}_boxes"
            m = re.search(r"const sbox %s\[4\]\s*=\s*\{(.*?)\n\};" % name, src, re.S)
            boxes = re.findall(r"\{\s*\{([^}]*)\},\s*\{([^}]*)\},\s*\{([^}]*)\}\s*\}", m.group(1))
            assert len(boxes) == 4, name
            out.append(f"pub const {name.upper()}: [SBox; 4] = [")
            for t, i, o in boxes:
                t, i, o = ints(t), ints(i), ints(o)
                assert len(t) == 64 and len(i) == 6 and len(o) == 2
                out.append(f"    SBox {{ table: {t}, inputs: {i}, outputs: {o} }},")
            out.append("];")
    text = "\n".join(out) + "\n"
    if "--check" in sys.argv:
        sys.exit(0 if OUT.exists() and OUT.read_text(encoding="utf-8") == text else "cps2crypt_tables.rs n'est pas à jour")
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print(f"{OUT.relative_to(ROOT)} : 4 groupes, 32 boîtes")

if __name__ == "__main__":
    main()
