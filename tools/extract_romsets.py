#!/usr/bin/env python3
"""Génère crates/cps-cart/src/romsets_data.rs depuis ref/mame-cps1/cps1.cpp (MAME, BSD-3) : tous les ROM_START et les lignes GAME.
Régions conservées : maincpu, gfx, audiocpu, oki, stars (les autres : PLD, qsound, etc. sont ignorées et listées en commentaire).
Usage : python tools/extract_romsets.py [--check]"""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "ref/mame-cps1/cps1.cpp"
OUT = ROOT / "crates/cps-cart/src/romsets_data.rs"
KEEP = {"maincpu", "gfx", "audiocpu", "oki", "stars", "qsound", "user1"}
LOADS = {"ROM_LOAD": "Load::Byte", "ROM_LOAD16_BYTE": "Load::Byte16", "ROM_LOAD16_WORD": "Load::Word16",
         "ROM_LOAD16_WORD_SWAP": "Load::Word16Swap", "ROM_LOAD64_WORD": "Load::Word64", "ROM_LOAD64_BYTE": "Load::Byte64",
         "ROM_LOAD32_BYTE": "Load::Byte32"}
NUM = r"(0x[0-9a-fA-F]+|\d+)"

def num(s):
    s = s.strip().replace("CODE_SIZE", "0x400000")
    return int(eval(s.replace("'", ""), {}, {}))

def parse():
    src = SRC.read_text(encoding="utf-8")
    sets = {}
    for m in re.finditer(r"^ROM_START\(\s*(\w+)\s*\)(.*?)^ROM_END", src, flags=re.S | re.M):
        name, body = m.group(1), m.group(2)
        body = re.sub(r"/\*.*?\*/", "", body, flags=re.S)
        regions, cur, skipped = [], None, set()
        for line in body.splitlines():
            line = re.sub(r"//.*", "", line).strip()
            if not line:
                continue
            r = re.match(r'ROM_REGION\s*\(\s*([^,]+),\s*"(\w+)"\s*,\s*([^)]*)\)', line)
            if r:
                rn = r.group(2)
                if rn in KEEP:
                    fill = 0xFF if "ERASEFF" in r.group(3) else 0
                    cur = {"name": rn, "size": num(r.group(1)), "fill": fill, "ops": []}
                    regions.append(cur)
                else:
                    cur = None; skipped.add(rn)
                continue
            if cur is None:
                continue
            if "NO_DUMP" in line:
                cur["ops"].append("Op::NoDump"); continue
            l = re.match(r'(ROM_LOAD\w*)\s*\(\s*"([^"]+)"\s*,\s*([^,]+),\s*([^,]+),\s*(?:BAD_DUMP\s*)?CRC\((\w+)\)', line)
            if l:
                cur["ops"].append(f'Op::Load {{ kind: {LOADS[l.group(1)]}, file: "{l.group(2)}", offset: {hex(num(l.group(3)))}, size: {hex(num(l.group(4)))}, crc: 0x{l.group(5).upper()} }}')
                continue
            l = re.match(r"ROM_(CONTINUE|RELOAD)\s*\(\s*([^,]+),\s*([^)]+)\)", line)
            if l:
                kind = "Continue" if l.group(1) == "CONTINUE" else "Reload"
                cur["ops"].append(f"Op::{kind} {{ offset: {hex(num(l.group(2)))}, size: {hex(num(l.group(3)))} }}")
                continue
            l = re.match(r"ROM_IGNORE\(\s*([^)]+)\)", line)
            if l:
                cur["ops"].append(f"Op::Ignore {{ size: {hex(num(l.group(1)))} }}"); continue
            l = re.match(r'ROM_COPY\(\s*"(\w+)"\s*,\s*([^,]+),\s*([^,]+),\s*([^)]+)\)', line)
            if l:
                cur["ops"].append(f'Op::Copy {{ src: "{l.group(1)}", src_offset: {hex(num(l.group(2)))}, offset: {hex(num(l.group(3)))}, size: {hex(num(l.group(4)))} }}'); continue
            l = re.match(r"ROM_FILL\(\s*([^,]+),\s*([^,]+),\s*([^)]+)\)", line)
            if l:
                cur["ops"].append(f"Op::Fill {{ offset: {hex(num(l.group(1)))}, size: {hex(num(l.group(2)))}, value: {hex(num(l.group(3)))} }}"); continue
            sys.exit(f"{name} : ligne non reconnue : {line}")
        sets[name] = (regions, sorted(skipped))
    games = []
    for g in re.finditer(r"^GAME\(\s*[^,]+,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*(\w+)\s*,\s*\w+\s*,\s*(\w+)\s*,\s*(ROT\d+)\s*,\s*\"([^\"]*)\"\s*,\s*\"((?:[^\"\\\\]|\\\\.)*)\"\s*,\s*([^)]*)\)", src, flags=re.M):
        games.append(g.groups())
    return sets, games

def main():
    sets, games = parse()
    out = ["// Fichier GÉNÉRÉ par tools/extract_romsets.py depuis ref/mame-cps1/cps1.cpp — NE PAS MODIFIER À LA MAIN.",
           "// license:BSD-3-Clause (données MAME, copyright-holders: Paul Leaman)",
           "//! Romsets CPS1 (régions maincpu, gfx, audiocpu, oki, stars) et table des jeux. spec: docs/spec/04", "",
           "use crate::romset::{GameInfo, Load, Op, RegionDef, RomSetDef};", ""]
    for name, (regions, skipped) in sets.items():
        regs = []
        for r in regions:
            ops = "".join(f"\n            {o}," for o in r["ops"])
            regs.append(f'        RegionDef {{ name: "{r["name"]}", size: {hex(r["size"])}, fill: {hex(r["fill"])}, ops: &[{ops}\n        ] }},')
        out.append(f"/// Régions ignorées : {', '.join(skipped) or '(aucune)'}")
        out.append(f"pub const RS_{name.upper()}: RomSetDef = RomSetDef {{ name: \"{name}\", regions: &[\n" + "\n".join(regs) + "\n] };")
    out.append("")
    out.append("/// Lignes GAME de cps1.cpp, dans l'ordre : (nom, parent, config machine, ports d'entrée, init, rotation, constructeur, titre, drapeaux).")
    out.append("pub static GAMES: &[GameInfo] = &[")
    for name, parent, machine, inp, init, rot, maker, title, flags in games:
        if name not in sets:
            continue
        title = title.replace('\\"', "'")
        out.append(f'    GameInfo {{ name: "{name}", parent: "{"" if parent == "0" else parent}", machine: "{machine}", input: "{inp}", init: "{init}", '
                   f'rotation: {rot[3:]}, maker: "{maker}", title: "{title}", not_working: {"true" if "NOT_WORKING" in flags else "false"}, romset: &RS_{name.upper()} }},')
    out.append("];")
    text = "\n".join(out) + "\n"
    if "--check" in sys.argv:
        sys.exit(0 if OUT.exists() and OUT.read_text(encoding="utf-8") == text else "romsets_data.rs n'est pas à jour")
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print(f"{OUT.relative_to(ROOT)} : {len(sets)} romsets, {len(games)} lignes GAME")

if __name__ == "__main__":
    main()
