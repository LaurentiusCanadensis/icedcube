// src/ui/mod.rs

//! UI module: panels, moves, and bottom control strip.

pub mod moves;
pub mod panels;
pub(crate) mod bottom;

pub use panels::{build_algorithm_panel, build_angle_block, build_presets_row, build_seed_panel};