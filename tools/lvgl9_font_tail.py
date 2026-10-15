#!/usr/bin/env python3
"""Rewrite an lv_font_conv 1.5.3 output to the LVGL 9 layout the repo uses.

docs/fonts.md lists the manual steps (drop the version guards and the v8
glyph cache, set release_glyph/kerning/static_bitmap/fallback/user_data);
this does them by regenerating everything after the "ALL CUSTOM DATA" banner
from the values the converter wrote. Usage: lvgl9_font_tail.py FILE...
"""
import re
import sys


def val(src, key):
    m = re.search(r"\." + key + r"\s*=\s*([^,\n]+?)\s*,?\s*(?:/\*.*)?\n", src)
    if not m:
        sys.exit(f"missing .{key}")
    return m.group(1).strip()


for path in sys.argv[1:]:
    src = open(path).read()
    banner = src.index("/*--------------------\n *  ALL CUSTOM DATA")
    head, tail = src[:banner], src[banner:]
    name = re.search(r"lv_font_t (\w+) = \{", tail).group(1)
    guard = re.search(r"#endif /\*#if (\w+)\*/", tail).group(1)
    dsc = {k: val(tail, k) for k in
           ("glyph_bitmap", "glyph_dsc", "cmaps", "kern_dsc", "kern_scale",
            "cmap_num", "bpp", "kern_classes", "bitmap_format")}
    pub = {k: val(tail, k) for k in
           ("line_height", "base_line", "underline_position", "underline_thickness")}
    body = ",\n".join(f"    .{k} = {v}" for k, v in dsc.items())
    tail = f"""/*--------------------
 *  ALL CUSTOM DATA
 *--------------------*/

static const lv_font_fmt_txt_dsc_t font_dsc = {{
{body}
}};

/*-----------------
 *  PUBLIC FONT
 *----------------*/

const lv_font_t {name} = {{
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,
    .release_glyph = NULL,
    .line_height = {pub['line_height']},
    .base_line = {pub['base_line']},
    .subpx = LV_FONT_SUBPX_NONE,
    .kerning = 0,
    .static_bitmap = 0,
    .underline_position = {pub['underline_position']},
    .underline_thickness = {pub['underline_thickness']},
    .dsc = &font_dsc,
    .fallback = NULL,
    .user_data = NULL,
}};

#endif /*#if {guard}*/
"""
    open(path, "w").write(head + tail)
