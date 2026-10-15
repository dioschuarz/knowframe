#!/usr/bin/env python3
"""
Generates the llm-wiki-icons.woff icon font from canonical SVG geometry.
Ensures 100% geometric parity with media/sidebar-icon.svg and media/tab-icon-dark.svg:
- 5-node graph: center hub r=3 at (12,12), 4 satellite nodes r=2 at (5,6), (19,6), (5,18), (19,18).
- 4 connecting stems stroke-width 1.5.
- Single unified polygon via shapely.ops.unary_union with ZERO overlaps, ZERO holes, and clockwise TrueType winding.
"""

from pathlib import Path
import shapely
from shapely.geometry import Point, LineString
from shapely.ops import unary_union
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.ttLib import TTFont

SCALE = 1000.0 / 24.0

def build_unified_polygon():
    c_center = Point(12, 12).buffer(3.0, quad_segs=32)
    c_tl = Point(5, 6).buffer(2.0, quad_segs=32)
    c_tr = Point(19, 6).buffer(2.0, quad_segs=32)
    c_bl = Point(5, 18).buffer(2.0, quad_segs=32)
    c_br = Point(19, 18).buffer(2.0, quad_segs=32)

    l_tl = LineString([(6.5, 7.5), (10, 10.5)]).buffer(0.75, cap_style='round', quad_segs=32)
    l_tr = LineString([(17.5, 7.5), (14, 10.5)]).buffer(0.75, cap_style='round', quad_segs=32)
    l_bl = LineString([(6.5, 16.5), (10, 13.5)]).buffer(0.75, cap_style='round', quad_segs=32)
    l_br = LineString([(17.5, 16.5), (14, 13.5)]).buffer(0.75, cap_style='round', quad_segs=32)

    merged = unary_union([c_center, c_tl, c_tr, c_bl, c_br, l_tl, l_tr, l_bl, l_br])
    simplified = merged.simplify(0.015, preserve_topology=True)
    return simplified

def main():
    media_dir = Path(__file__).resolve().parent
    woff_path = media_dir / 'llm-wiki-icons.woff'

    poly = build_unified_polygon()

    coords = []
    for x, y in poly.exterior.coords:
        fx = round(x * SCALE)
        fy = round((24.0 - y) * SCALE)
        coords.append((fx, fy))

    def signed_area(pts):
        return sum(pts[i][0]*pts[(i+1)%len(pts)][1] - pts[(i+1)%len(pts)][0]*pts[i][1] for i in range(len(pts))) / 2

    if signed_area(coords) > 0:
        coords.reverse()

    font = TTFont(woff_path)

    pen = TTGlyphPen()
    pen.moveTo(coords[0])
    for pt in coords[1:]:
        pen.lineTo(pt)
    pen.closePath()

    font['glyf']['llm-wiki-graph'] = pen.glyph()
    glyph = font['glyf']['llm-wiki-graph']
    glyph.recalcBounds(font['glyf'])

    font['head'].xMin = min(pt[0] for pt in coords)
    font['head'].yMin = min(pt[1] for pt in coords)
    font['head'].xMax = max(pt[0] for pt in coords)
    font['head'].yMax = max(pt[1] for pt in coords)

    font.flavor = 'woff'
    font.save(woff_path)
    print(f'Successfully generated {woff_path} ({len(coords)} vertices)')

if __name__ == '__main__':
    main()
