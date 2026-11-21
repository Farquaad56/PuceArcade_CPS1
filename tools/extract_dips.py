#!/usr/bin/env python3
"""Génère crates/cps-cart/src/dips_data.rs : valeurs par défaut ET description (libellés, réglages) des ports DSWA/DSWB/DSWC de chaque bloc INPUT_PORTS de
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

def label(t):
    t = t.strip()
    m = re.match(r"DEF_STR\(\s*(\w+)\s*\)", t)
    if not m:
        return t.strip('"').replace("\\", "")
    k = m.group(1)
    c = re.match(r"(\d+)C_(\d+)C$", k)
    if c:
        a, b = int(c.group(1)), int(c.group(2))
        return f"{a} Coin{'s' if a > 1 else ''}/{b} Credit{'s' if b > 1 else ''}"
    return k.replace("_", " ")

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
        m = re.match(r"PORT_DIPNAME\(\s*([^,]+),\s*([^,]+),\s*(DEF_STR\(\s*\w+\s*\)|\"[^\"]*\")", line)
        if m:
            field = [num(m.group(1)), num(m.group(2)), label(m.group(3)), []]
        m = None if field else re.match(r"PORT_(DIPUNUSED|DIPUNKNOWN)(?:_DIPLOC)?\(\s*([^,]+),\s*([^,)]+)", line)
        if m:
            mk, d = num(m.group(2)), num(m.group(3))
            field = [mk, d, "Unused" if m.group(1) == "DIPUNUSED" else "Unknown", [("Off", d), ("On", d ^ mk)]]
        m = None if field else re.match(r"PORT_(SERVICE_DIPLOC|SERVICE_NO_TOGGLE|SERVICE|BIT)\(\s*([^,]+),\s*(IP_ACTIVE_LOW|IP_ACTIVE_HIGH)", line)
        if m:
            mask = num(m.group(2)); d = mask if m.group(3) == "IP_ACTIVE_LOW" else 0
            field = [mask, d, "Service Mode" if m.group(1).startswith("SERVICE") else "Unknown", [("Off", d), ("On", d ^ mask)]]
            t = re.search(r"IPT_(\w+)", line); pl = re.search(r"PORT_PLAYER\(\s*(\d)\s*\)", line)
            field.append((t.group(1) if t else "", int(pl.group(1)) if pl else 1))
        m = re.match(r"PORT_DIPSETTING\(\s*([^,]+),\s*(DEF_STR\(\s*\w+\s*\)|\"[^\"]*\")", line)
        if m and cur and ports.get(cur):
            ports[cur][-1][3].append((label(m.group(2)), num(m.group(1))))
            continue
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
            vals.append(0xFF if p not in ports else (sum(f[1] & f[0] for f in ports[p]) & 0xFF))
        fields = [(i, f[:4]) for i, p in enumerate(("DSWA", "DSWB", "DSWC")) for f in ports.get(p, [])]
        metas = [f[4] for p, fl in ports.items() if not p.startswith("DSW") for f in fl if len(f) > 4]
        players = max([pl for t, pl in metas if t.startswith("JOYSTICK")] or [1])
        buttons = max([int(t[6:]) for t, pl in metas if re.fullmatch(r"BUTTON\d+", t) and pl == 1] or [0])
        rows.append((name, vals, fields, players, buttons))
    out = ["// Fichier GÉNÉRÉ par tools/extract_dips.py depuis ref/mame-cps1/cps1.cpp — NE PAS MODIFIER À LA MAIN.",
           "// license:BSD-3-Clause (données MAME)", "//! DIP switches par défaut (DSWA, DSWB, DSWC) par bloc INPUT_PORTS. spec: docs/spec/01 §DIP", "",
           "use crate::dips::DipFieldDef;", "",
           "/// (nom du bloc INPUT_PORTS = `GameInfo::input`, dswa, dswb, dswc)", "pub static DIPS: &[(&str, u8, u8, u8)] = &["]
    for n, v, *_ in rows:
        out.append(f'    ("{n}", 0x{v[0]:02X}, 0x{v[1]:02X}, 0x{v[2]:02X}),')
    out.append("];")
    out.append("")
    out.append("/// Champs DIP par bloc INPUT_PORTS, dans l'ordre du source (port 0 = DSWA, 1 = DSWB, 2 = DSWC).")
    out.append("pub static DIP_FIELDS: &[(&str, &[DipFieldDef])] = &[")
    esc = lambda t: t.replace('"', "'")
    for n, _, fields, *_ in rows:
        items = []
        for port, (mk, d, lab, sets) in fields:
            st = ", ".join(f'("{esc(a)}", 0x{b:02X})' for a, b in sets)
            items.append(f'DipFieldDef {{ port: {port}, mask: 0x{mk:02X}, default: 0x{d & mk:02X}, label: "{esc(lab)}", settings: &[{st}] }}')
        out.append(f'    ("{n}", &[' + ", ".join(items) + "]),")
    out.append("];")
    out.append("")
    out.append("/// (bloc INPUT_PORTS, nombre de joueurs, boutons du joueur 1) — pour BoardInfo du frontal JAMMA.")
    out.append("pub static INPUTS: &[(&str, u8, u8)] = &[")
    for n, _, _, pl, bt in rows:
        out.append(f'    ("{n}", {pl}, {bt}),')
    out.append("];")
    text = "\n".join(out) + "\n"
    if "--check" in sys.argv:
        sys.exit(0 if OUT.exists() and OUT.read_text(encoding="utf-8") == text else "dips_data.rs n'est pas à jour")
    OUT.write_text(text, encoding="utf-8", newline="\n"); print(f"{OUT.relative_to(ROOT)} : {len(rows)} blocs")

if __name__ == "__main__":
    main()
