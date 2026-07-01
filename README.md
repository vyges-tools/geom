# vyges-geom

**The shared layout-geometry substrate for the Vyges toolchain** — axis-aligned
rectangle primitives and a uniform-grid **spatial index**, std-only.

Factored out of [`vyges-layout`](https://github.com/vyges-tools/layout) so the
geometry has one home and the spatial index is reusable on its own. `vyges-layout`
depends on this crate and re-exports it, so the layout-side engines (DRC, LVS, the
chip viewer) get the same `geom` / `index` they always had — now with a real crate
boundary underneath.

## What's here

| Module | What it provides |
| --- | --- |
| `geom` | `Rect` (axis-aligned, integer DBU) — area, boundary conversion, rectangle detection, bbox, polygon area, `intersects`, `inflate` (spacing halo) |
| `index` | `RegionIndex` — a uniform-grid spatial index over rectangles: `query` / `overlaps` / `any_overlap` / `within` (spacing halo). Turns "what's near this shape?" from O(N) per query into a handful of grid-cell lookups |

## Use

```rust
use vyges_geom::geom::Rect;
use vyges_geom::index::RegionIndex;

let shapes = vec![Rect::new(0, 0, 100, 100), Rect::new(120, 0, 220, 100)];
let idx = RegionIndex::build(&shapes);

// shapes overlapping a region
let hits = idx.overlaps(&Rect::new(50, 50, 130, 60));

// neighbours within a spacing halo (excluding the query shape) — the DRC pattern
let near = idx.within(&shapes[0], 30, Some(0));
```

The index returns a candidate set; callers recheck with the exact predicate
(`spacing`, `keeps_gap`, …) — near-linear with results identical to an all-pairs scan.

## Design

A **uniform grid** (not an R-tree) is deliberate: layout shapes are numerous, small,
and roughly uniform, which is exactly where a grid wins — O(1) insert, cache-friendly,
no rebalancing. The index is **static** (build once, query many); the grid is capped so
pathological inputs cannot allocate unbounded buckets. Incremental update and an R-tree
variant are reserved depth.

## License

Apache-2.0. © 2026 Vyges. <https://vyges.com>
