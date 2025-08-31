// src/render/layout.rs

//! Compute canvas layout/origins for the two cube views and keep them visible.

use iced::Rectangle;

use super::types::ViewParams;
use super::geom::{project, rotate_pt_all};

/// 8 cube corners in object space (2×2×2 cube)
#[inline]
fn cube_corners() -> [(f32,f32,f32); 8] {
    [
        (0.0,0.0,0.0), (2.0,0.0,0.0), (0.0,2.0,0.0), (2.0,2.0,0.0),
        (0.0,0.0,2.0), (2.0,0.0,2.0), (0.0,2.0,2.0), (2.0,2.0,2.0),
    ]
}

fn min_projected_y(vp: &ViewParams) -> f32 {
    cube_corners()
        .map(|p| rotate_pt_all(p, vp.rz, vp.ry, vp.rx))
        .map(|(x,y,z)| project(x, y, z, vp.size, vp.origin).1)
        .into_iter()
        .fold(f32::INFINITY, f32::min)
}

fn max_projected_y(vp: &ViewParams) -> f32 {
    cube_corners()
        .map(|p| rotate_pt_all(p, vp.rz, vp.ry, vp.rx))
        .map(|(x,y,z)| project(x, y, z, vp.size, vp.origin).1)
        .into_iter()
        .fold(f32::NEG_INFINITY, f32::max)
}

/// Initial horizontal placement + vertical center line.
pub fn layout_origins(bounds: Rectangle, size: f32) -> ((f32,f32),(f32,f32)) {
    let mid_x    = bounds.x + bounds.width * 0.5;
    let center_y = bounds.y + bounds.height * 0.48; // near true vertical center

    // Horizontal spacing that scales with width/size but stays reasonable
    let min_gap = size * 2.6;
    let max_gap = bounds.width * 0.60;
    let gap = ((bounds.width * 0.30) + size * 1.0).clamp(min_gap, max_gap);

    ((mid_x - gap * 0.5, center_y), (mid_x + gap * 0.5, center_y))
}

/// Shift both origins vertically so the pair stays centered and within margins.
pub fn fit_vertically(bounds: Rectangle, left: &mut ViewParams, right: &mut ViewParams) {
    // Combined vertical bounding box (screen Y) for both cubes
    let (min_l, max_l) = (min_projected_y(left),  max_projected_y(left));
    let (min_r, max_r) = (min_projected_y(right), max_projected_y(right));
    let min_all = min_l.min(min_r);
    let max_all = max_l.max(max_r);
    let center_all = 0.5 * (min_all + max_all);

    // Target vertical center (slightly above exact middle so UI has air)
    let desired_center = bounds.y + bounds.height * 0.48;

    // Shift needed to center
    let mut dy = desired_center - center_all;

    // Keep inside top/bottom margins
    let top_margin    = bounds.y + 8.0;
    let bottom_margin = bounds.y + bounds.height - 8.0;
    dy = dy.clamp(top_margin - min_all, bottom_margin - max_all);

    if dy.abs() > 0.01 {
        left.origin.1  += dy;
        right.origin.1 += dy;
    }
}