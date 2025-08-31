// src/render/face.rs

//! Draw a single cube face (plastic base + 2×2 stickers) into a canvas frame.

use iced::Color;
use iced::widget::canvas::{self, Frame};

use crate::cube::{Col, Face, FaceId};
use super::types::{RotZ, RotX, RotY};
use super::geom::{project, face_outer, inset_polygon, face_visible, rotate_pt_all};

fn base_color(c: Col) -> Color {
    match c {
        Col::W => Color::from_rgb(1.0, 1.0, 1.0),
        Col::Y => Color::from_rgb(1.0, 0.90, 0.00),
        Col::R => Color::from_rgb(0.90, 0.00, 0.00),
        Col::O => Color::from_rgb(1.00, 0.50, 0.00),
        Col::B => Color::from_rgb(0.00, 0.35, 0.90),
        Col::G => Color::from_rgb(0.00, 0.60, 0.20),
    }
}

fn path_polygon(points: &[[f32; 2]]) -> canvas::Path {
    canvas::Path::new(|b| {
        if let Some(first) = points.first() {
            b.move_to(iced::Point::new(first[0], first[1]));
            for p in &points[1..] {
                b.line_to(iced::Point::new(p[0], p[1]));
            }
            b.close();
        }
    })
}

/// Local copy of the raw 3D quad for a given face cell (row `r`, col `c`).
/// This avoids importing `face_cell_raw` in case your build/module layout differs.
fn cell_quad_raw(face: FaceId, r: usize, c: usize) -> [(f32, f32, f32); 4] {
    let r = r as f32;
    let c = c as f32;
    let p = |x: f32, y: f32, z: f32| (x, y, z);

    match face {
        FaceId::U => [
            p(c, r, 2.0),
            p(c + 1.0, r, 2.0),
            p(c + 1.0, r + 1.0, 2.0),
            p(c, r + 1.0, 2.0),
        ],
        FaceId::D => [
            p(c, 2.0 - r, 0.0),
            p(c + 1.0, 2.0 - r, 0.0),
            p(c + 1.0, 2.0 - (r + 1.0), 0.0),
            p(c, 2.0 - (r + 1.0), 0.0),
        ],
        FaceId::F => {
            let z0 = 2.0 - r;
            let z1 = 2.0 - (r + 1.0);
            [p(c, 0.0, z0), p(c + 1.0, 0.0, z0), p(c + 1.0, 0.0, z1), p(c, 0.0, z1)]
        }
        FaceId::B => {
            let z0 = 2.0 - r;
            let z1 = 2.0 - (r + 1.0);
            [
                p(2.0 - c, 2.0, z0),
                p(2.0 - (c + 1.0), 2.0, z0),
                p(2.0 - (c + 1.0), 2.0, z1),
                p(2.0 - c, 2.0, z1),
            ]
        }
        FaceId::L => {
            let z0 = 2.0 - r;
            let z1 = 2.0 - (r + 1.0);
            [
                p(0.0, 2.0 - c, z0),
                p(0.0, 2.0 - (c + 1.0), z0),
                p(0.0, 2.0 - (c + 1.0), z1),
                p(0.0, 2.0 - c, z1),
            ]
        }
        FaceId::R => {
            let z0 = 2.0 - r;
            let z1 = 2.0 - (r + 1.0);
            [
                p(2.0, c, z0),
                p(2.0, c + 1.0, z0),
                p(2.0, c + 1.0, z1),
                p(2.0, c, z1),
            ]
        }
    }
}

/// Draw one face of the cube with plastic edges and sticker seams.
/// Skips rendering if the face is back-facing for the given orientation.
pub fn draw_face(
    fr: &mut Frame,
    face: &Face,
    which: FaceId,
    origin: (f32, f32),
    size: f32,
    rz: RotZ,
    rx: RotX,
    ry: RotY,
) {
    if !face_visible(which, rz, rx, ry) {
        return;
    }

    // 1) plastic base
    let outer = face_outer(which).map(|p| rotate_pt_all(p, rz, ry, rx));
    let outer_xy = outer.map(|(x, y, z)| project(x, y, z, size, origin));
    let outer_path = path_polygon(&[
        [outer_xy[0].0, outer_xy[0].1],
        [outer_xy[1].0, outer_xy[1].1],
        [outer_xy[2].0, outer_xy[2].1],
        [outer_xy[3].0, outer_xy[3].1],
    ]);

    let plastic_w = (size * 0.070).clamp(0.9, 2.4);
    let seam_w = (size * 0.030).clamp(0.4, 1.2);
    let inset_k = (0.09 + (size - 24.0) * 0.002).clamp(0.09, 0.14);

    fr.fill(&outer_path, Color::from_rgb(0.05, 0.05, 0.05));
    fr.stroke(
        &outer_path,
        canvas::Stroke {
            width: plastic_w,
            style: canvas::stroke::Style::Solid(Color::from_rgb(0.03, 0.03, 0.03)),
            ..Default::default()
        },
    );

    // 2) stickers
    for r in 0..2 {
        for c in 0..2 {
            let q3 = cell_quad_raw(which, r, c).map(|p| rotate_pt_all(p, rz, ry, rx));
            let pts = q3.map(|(x, y, z)| project(x, y, z, size, origin));

            let raw2d = [
                (pts[0].0, pts[0].1),
                (pts[1].0, pts[1].1),
                (pts[2].0, pts[2].1),
                (pts[3].0, pts[3].1),
            ];
            let inset = inset_polygon(&raw2d, inset_k);

            let poly = path_polygon(&[
                [inset[0].0, inset[0].1],
                [inset[1].0, inset[1].1],
                [inset[2].0, inset[2].1],
                [inset[3].0, inset[3].1],
            ]);

            fr.fill(&poly, base_color(face[r][c]));
            fr.stroke(
                &poly,
                canvas::Stroke {
                    width: seam_w,
                    style: canvas::stroke::Style::Solid(Color::from_rgb(0.04, 0.04, 0.04)),
                    ..Default::default()
                },
            );
        }
    }
}