#!/usr/bin/env python3
"""Génère crates/cps-gfx/src/config_data.rs à partir de ref/mame-cps1/cps1_v.cpp (MAME, BSD-3).
Contenu : macros CPS_B_xx / HACK_B_x, tables mapper_xxx (bank_sizes + plages), table cps1_config_table.
Aucune valeur n'est écrite à la main : tout vient du source. Usage : python tools/gen_cps_tables.py [--check]
--check : régénère en mémoire et échoue (code 1) si le fichier versionné diffère."""
import re, sys, pathlib
ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "ref/mame-cps1/cps1_v.cpp"
OUT = ROOT / "crates/cps-gfx/src/config_data.rs"
TYPES = {"GFXTYPE_SPRITES": 1, "GFXTYPE_SCROLL1": 2, "GFXTYPE_SCROLL2": 4, "GFXTYPE_SCROLL3": 8, "GFXTYPE_STARS": 16}

def strip_comments(s):
    s = re.sub(r"/\*.*?\*/", "", s, flags=re.S)
    return re.sub(r"//[^\n]*", "", s)

def ints(txt):
    return [int(x, 0) for x in re.findall(r"-?0x[0-9a-fA-F]+|-?\d+", txt)]

def main():
    src = strip_comments(SRC.read_text(encoding="utf-8"))
    na = "-1,-1,-1,-1,-1,-1,-1"
    cpsb = {}
    for m in re.finditer(r"^#define\s+((?:CPS_B|HACK_B)_\w+)\s+(.*)$", src, flags=re.M):
        body = m.group(2).replace("__not_applicable__", na).replace("{", " ").replace("}", " ")
        v = ints(body)
        if len(v) != 20:
            sys.exit(f"{m.group(1)} : {len(v)} champs (20 attendus)")
        cpsb[m.group(1)] = v
    mappers = {}
    for m in re.finditer(r"^#define\s+(mapper_\w+)\s+\{([^}]*)\}\s*,\s*(mapper_\w+_table)", src, flags=re.M):
        sizes = ints(m.group(2))
        t = re.search(r"static const struct gfx_range " + m.group(3) + r"\[\]\s*=\s*\{(.*?)\n\};", src, flags=re.S)
        if not t:
            sys.exit(f"table {m.group(3)} introuvable")
        rows = []
        for e in re.finditer(r"\{\s*([A-Z_0-9| ]+?)\s*,\s*(0x[0-9a-fA-F]+|\d+)\s*,\s*(0x[0-9a-fA-F]+|\d+)\s*,\s*(\d+)\s*\}", t.group(1)):
            ty = 0
            for part in e.group(1).split("|"):
                ty |= TYPES[part.strip()]
            rows.append((ty, int(e.group(2), 0), int(e.group(3), 0), int(e.group(4))))
        mappers[m.group(1)] = (sizes, rows)
    table = re.search(r"static const struct CPS1config cps1_config_table\[\]\s*=\s*\{(.*?)\{nullptr\}", src, flags=re.S).group(1)
    games = []
    for e in re.finditer(r'\{\s*"(\w+)"\s*,\s*((?:CPS_B|HACK_B)_\w+)\s*,\s*(mapper_\w+)\s*((?:,\s*-?(?:0x[0-9a-fA-F]+|\d+)\s*)*)\}', table):
        extra = ints(e.group(4)) + [0, 0, 0, 0]
        games.append((e.group(1), e.group(2), e.group(3), extra[:4]))
    out = ["// Fichier GÉNÉRÉ par tools/gen_cps_tables.py depuis ref/mame-cps1/cps1_v.cpp — NE PAS MODIFIER À LA MAIN.",
           "// license:BSD-3-Clause (données MAME, copyright-holders: Paul Leaman)",
           "//! Configurations CPS-B, mappers de banques ROM graphiques et table des jeux (spec: docs/spec/02 §Config).",
           "#![allow(clippy::unreadable_literal)]", "",
           "use crate::config::{CpsBConfig, GameConfig, GfxRange, Mapper};", ""]
    for name, v in cpsb.items():
        out.append(f"pub const {name}: CpsBConfig = CpsBConfig {{ cpsb_addr: {v[0]}, cpsb_value: {v[1]}, mult_factor1: {v[2]}, mult_factor2: {v[3]}, "
                   f"mult_result_lo: {v[4]}, mult_result_hi: {v[5]}, unknown1: {v[6]}, unknown2: {v[7]}, unknown3: {v[8]}, layer_control: {v[9]}, "
                   f"priority: [{v[10]}, {v[11]}, {v[12]}, {v[13]}], palette_control: {v[14]}, layer_enable_mask: [{v[15]}, {v[16]}, {v[17]}, {v[18]}, {v[19]}] }};")
    out.append("")
    for name, (sizes, rows) in mappers.items():
        r = ", ".join(f"GfxRange {{ kind: {t}, start: {hex(s)}, end: {hex(e)}, bank: {b} }}" for t, s, e, b in rows)
        out.append(f"pub const {name.upper()}: Mapper = Mapper {{ bank_sizes: [{', '.join(hex(x) for x in sizes)}], ranges: &[{r}] }};")
    out.append("")
    out.append("/// Table des jeux, dans l'ordre de cps1_config_table (la recherche prend la première entrée de ce nom).")
    out.append("pub static GAMES: &[GameConfig] = &[")
    for g, b, m, x in games:
        out.append(f'    GameConfig {{ name: "{g}", cpsb: {b}, mapper: {m.upper()}, in2_addr: {x[0]}, in3_addr: {x[1]}, out2_addr: {x[2]}, bootleg_kludge: {x[3]} }},')
    out.append("];")
    text = "\n".join(out) + "\n"
    if "--check" in sys.argv:
        if not OUT.exists() or OUT.read_text(encoding="utf-8") != text:
            sys.exit("config_data.rs n'est pas à jour : relancer python tools/gen_cps_tables.py")
        print("OK"); return
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print(f"{OUT.relative_to(ROOT)} : {len(cpsb)} CPS-B, {len(mappers)} mappers, {len(games)} jeux")

if __name__ == "__main__":
    main()
