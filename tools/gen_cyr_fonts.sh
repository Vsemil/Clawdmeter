#!/usr/bin/env bash
# Cyrillic fallback fonts for the RU interface. Styrene, Tiempos and the mono
# font ship ASCII only, so each size gets a companion font holding Cyrillic
# (plus a few typographic marks) that LVGL falls back to glyph by glyph:
#   Styrene -> Golos Text, Tiempos -> PT Serif, mono -> DejaVu Sans Mono.
# The base fonts point at these via `.fallback` (see the base font_*.c files).
# Run from the repository root.
set -euo pipefail

RANGE="0x401,0x410-0x44F,0x451,0xAB,0xBB,0x2014,0x2116"
CONV=(npx --yes lv_font_conv@1.5.3)

gen() {  # font file, size, output symbol
    local out="firmware/src/$3.c"
    "${CONV[@]}" --font "$1" -r "$RANGE" --size "$2" --format lvgl --bpp 4 \
        --no-compress -o "$out" --lv-include "lvgl.h"
    # Path-free header so regenerating on another machine is diff-clean.
    sed -i '' -e "s|--font [^ ]*/assets/|--font assets/|; s| -o [^ ]*/firmware/src/| -o firmware/src/|" "$out"
    python3 tools/lvgl9_font_tail.py "$out"   # LVGL 9 struct layout (docs/fonts.md)
}

for size in 48 28 24 20 16 14 12; do gen assets/GolosText-Regular.ttf "$size" "font_golos_${size}_cyr"; done
for size in 56 34;                do gen assets/PTSerif-Regular.ttf   "$size" "font_ptserif_${size}_cyr"; done
for size in 32 18;                do gen assets/DejaVuSansMono.ttf    "$size" "font_mono_${size}_cyr"; done
