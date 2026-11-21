#!/usr/bin/env python3
"""Génère crates/cps-cart/src/dips_data.rs : valeurs par défaut des ports DSWA/DSWB/DSWC de chaque bloc INPUT_PORTS de
ref/mame-cps1/cps1.cpp (MAME, BSD-3). Gère PORT_INCLUDE, PORT_START, PORT_MODIFY et les macros CPS1_COINAGE_n / CPS1_DIFFICULTY_n.
Défaut d'un port = OU des défauts de ses champs (bits non définis = 0) ; un champ redéfini (PORT_MODIFY) remplace les champs qui le recouvrent.
Port absent : 0xFF. Usage : python tools/extract_dips.py [--check]"""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "ref/mame-cps1/cps1.cpp"
OUT = ROOT / "crates/cps-cart/src/dips_data.rs"

def load():
    src = SRC.read_text(encoding="utf-8")
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    src = re.sub(r"//[^\n]*", "", src)
    macros = {}
    for m in re.finditer(r"^#define\s+(CPS1_\w+)\(diploc\)\s*\\\n((?:.*\\\n)*.*)", src, flags=re.M):
        macros[m.group(1)] = [l.rstrip("\\").strip() for l in m.group(2).splitlines()]
    blocks = {}
    for m in re.finditer(r"INPUT_PORTS_START\(\s*(\w+)\s*\)(.*?)INPUT_PORTS_END", src, flags=re.S):
        blocks[m.group(1)] = m.group(2).splitlines()
    return macros, blocks

def num(s):
    return int(s.strip(), 0)

def run(name, blocks, macros, ports, depth=0):
    cur = None
    lines = list(blocks[name])
    i = 0
    while i < len(lines):
        line = lines[i].strip(); i += 1
        if not line:
            continue
        mm = re.match(r"(CPS1_\w+)\(", line)
        if mm and mm.group(1) in macros:
            lines[i:i] = macros[mm.group(1)]; continue
        m = re.match(r"PORT_INCLUDE\(\s*(\w+)\s*\)", line)
        if m:
            run(m.group(1), blocks, macros, ports, depth + 1); continue
        m = re.match(r'PORT_START\(\s*"(\w+)"\s*\)', line)
        if m:
            cur = m.group(1); ports[cur] = []; continue
        m = re.match(r'PORT_MODIFY\(\s*"(\w+)"\s*\)', line)
        if m:
            cur = m.group(1); ports.setdefault(cur, []); continue
        field = None
        m = re.match(r"PORT_(?:DIPNAME|DIPUNUSED_DIPLOC|DIPUNUSED|DIPUNKNOWN_DIPLOC|DIPUNKNOWN)\(\s*([^,]+),\s*([^,)]+)", line)
        if m:
            field = (num(m.group(1)), num(m.group(2)))
        m = re.match(r"PORT_(?:SERVICE_DIPLOC|SERVICE_NO_TOGGLE|SERVICE|BIT)\(\s*([^,]+),\s*(IP_ACTIVE_LOW|IP_ACTIVE_HIGH)", line)
        if m:
            mask = num(m.group(1)); field = (mask, mask if m.group(2) == "IP_ACTIVE_LOW" else 0)
        if field and cur:
            ports[cur] = [f for f in ports[cur] if f[0] & field[0] == 0] + [field]

def main():
    macros, blocks = load()
    rows = []
    for name in blocks:
        ports = {}
        run(name, blocks, macros, ports)
        vals = []
        for p in ("DSWA", "DSWB", "DSWC"):
            vals.append(0xFF if p not in ports else (sum(d & m for m, d in ports[p]) & 0xFF))
        rows.append((name, vals))
    out = ["// Fichier GÉNÉRÉ par tools/extract_dips.py depuis ref/mame-cps1/cps1.cpp — NE PAS MODIFIER À LA MAIN.",
           "// license:BSD-3-Clause (données MAME)", "//! DIP switches par défaut (DSWA, DSWB, DSWC) par bloc INPUT_PORTS. spec: docs/spec/01 §DIP", "",
           "/// (nom du bloc INPUT_PORTS = `GameInfo::input`, dswa, dswb, dswc)", "pub static DIPS: &[(&str, u8, u8, u8)] = &["]
    for n, v in rows:
        out.append(f'    ("{n}", 0x{v[0]:02X}, 0x{v[1]:02X}, 0x{v[2]:02X}),')
    out.append("];")
    text = "\n".join(out) + "\n"
    if "--check" in sys.argv:
        sys.exit(0 if OUT.exists() and OUT.read_text(encoding="utf-8") == text else "dips_data.rs n'est pas à jour")
    OUT.write_text(text, encoding="utf-8", newline="\n"); print(f"{OUT.relative_to(ROOT)} : {len(rows)} blocs")

if __name__ == "__main__":
    main()
