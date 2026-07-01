//! vyges-geom — the shared layout geometry substrate.
//!
//! Axis-aligned rectangle primitives ([`geom`]) and a uniform-grid **spatial index**
//! ([`index::RegionIndex`]) for region / overlap / spacing-halo queries — the
//! geometry the Vyges layout-side engines (`vyges-layout`, and through it DRC / LVS /
//! the viewer) share. Factored out of `vyges-layout` so the geometry has one home and
//! the spatial index is reusable on its own.
//!
//! Std-only — no external dependencies, builds and tests offline.
//!
//! ```
//! use vyges_geom::geom::Rect;
//! use vyges_geom::index::RegionIndex;
//! let idx = RegionIndex::build(&[Rect::new(0, 0, 10, 10), Rect::new(100, 100, 110, 110)]);
//! assert_eq!(idx.overlaps(&Rect::new(5, 5, 50, 50)), vec![0]);
//! ```

pub mod geom;
pub mod index;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "© 2026 Vyges. All Rights Reserved.  https://vyges.com";
