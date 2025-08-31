// src/render/geom.rs

//! 3D → 2D math: rotations about cube center, isometric projection,
//! face visibility, depths, and per-cell geometry.

use iced::Point;

use crate::cube::FaceId;
use super::types::{RotZ, RotX, RotY};

/// Classic isometric projection of `(x,y,z)` with a per-view size and origin.
#[inline]
pub fn project(x: f32, y: f32, z: f32, size: f32, origin: (f32, f32)) -> (f32, f32) {
    let ex = (0.8660254, -0.5);
    let ey = (-0.8660254, -0.5);
    let ez = (0.0,       -1.0);
    let px = origin.0 + size * (x * ex.0 + y * ey.0 + z * ez.0);
    let py = origin.1 + size * (x * ex.1 + y * ey.1 + z * ez.1);
    (px, py)
}

// rotate about cube center (1,1,1)
const CEN: (f32, f32, f32) = (1.0, 1.0, 1.0);

#[inline]
fn rot_z_point(p: (f32,f32,f32), deg: f32) -> (f32,f32,f32) {
    let (x,y,z) = p; let (x0,y0,z0) = (x-CEN.0, y-CEN.1, z-CEN.2);
    let r = deg.to_radians(); let (c,s) = (r.cos(), r.sin());
    (c*x0 - s*y0 + CEN.0, s*x0 + c*y0 + CEN.1, z0 + CEN.2)
}
#[inline]
fn rot_y_point(p: (f32,f32,f32), deg: f32) -> (f32,f32,f32) {
    let (x,y,z) = p; let (x0,y0,z0) = (x-CEN.0, y-CEN.1, z-CEN.2);
    let r = deg.to_radians(); let (c,s) = (r.cos(), r.sin());
    (c*x0 + s*z0 + CEN.0, y0 + CEN.1, -s*x0 + c*z0 + CEN.2)
}
#[inline]
fn rot_x_point(p: (f32,f32,f32), deg: f32) -> (f32,f32,f32) {
    let (x,y,z) = p; let (x0,y0,z0) = (x-CEN.0, y-CEN.1, z-CEN.2);
    let r = deg.to_radians(); let (c,s) = (r.cos(), r.sin());
    (x0 + CEN.0, c*y0 - s*z0 + CEN.1, s*y0 + c*z0 + CEN.2)
}

#[inline]
pub fn rotate_pt_all(p: (f32,f32,f32), rz: RotZ, ry: RotY, rx: RotX) -> (f32,f32,f32) {
    let pz = rot_z_point(p, rz.0);
    let py = rot_y_point(pz, ry.0);
    rot_x_point(py, rx.0)
}

// Outer polygon of each face in CCW order w.r.t. OUTWARD normal.
pub fn face_outer(face: FaceId) -> [(f32,f32,f32);4] {
    match face {
        // +Z (Up): u=+X, v=+Y
        FaceId::U => [(0.0,0.0,2.0),(2.0,0.0,2.0),(2.0,2.0,2.0),(0.0,2.0,2.0)],
        // -Z (Down): u=+X, v=-Y
        FaceId::D => [(0.0,2.0,0.0),(2.0,2.0,0.0),(2.0,0.0,0.0),(0.0,0.0,0.0)],
        // -Y (Front): u=+X, v=+Z, plane y=0
        FaceId::F => [(0.0,0.0,0.0),(2.0,0.0,0.0),(2.0,0.0,2.0),(0.0,0.0,2.0)],
        // +Y (Back): u=-X, v=+Z, plane y=2
        FaceId::B => [(2.0,2.0,0.0),(0.0,2.0,0.0),(0.0,2.0,2.0),(2.0,2.0,2.0)],
        // -X (Left): u=+Y, v=-Z, plane x=0
        FaceId::L => [(0.0,0.0,2.0),(0.0,2.0,2.0),(0.0,2.0,0.0),(0.0,0.0,0.0)],
        // +X (Right): u=+Y, v=+Z, plane x=2
        FaceId::R => [(2.0,0.0,0.0),(2.0,2.0,0.0),(2.0,2.0,2.0),(2.0,0.0,2.0)],
    }
}

/// Inset a 2D quad toward its centroid by fraction `k` (0..1).
pub fn inset_polygon(pts: &[(f32,f32);4], k: f32) -> [(f32,f32);4] {
    let cx = (pts[0].0 + pts[1].0 + pts[2].0 + pts[3].0) * 0.25;
    let cy = (pts[0].1 + pts[1].1 + pts[2].1 + pts[3].1) * 0.25;
    [
        (cx + (pts[0].0 - cx) * (1.0 - k), cy + (pts[0].1 - cy) * (1.0 - k)),
        (cx + (pts[1].0 - cx) * (1.0 - k), cy + (pts[1].1 - cy) * (1.0 - k)),
        (cx + (pts[2].0 - cx) * (1.0 - k), cy + (pts[2].1 - cy) * (1.0 - k)),
        (cx + (pts[3].0 - cx) * (1.0 - k), cy + (pts[3].1 - cy) * (1.0 - k)),
    ]
}

/// Simple back-face test using projected signed area (2D).
pub fn face_visible(face: FaceId, rz: RotZ, rx: RotX, ry: RotY) -> bool {
    let q3 = face_outer(face).map(|p| rotate_pt_all(p, rz, ry, rx));
    let pts = q3.map(|(x,y,z)| project(x, y, z, 1.0, (0.0, 0.0)));

    let mut a = 0.0f32;
    for i in 0..4 {
        let j = (i + 1) & 3;
        a += pts[i].0 * pts[j].1 - pts[j].0 * pts[i].1;
    }
    a < 0.0
}

pub fn face_depth(face: FaceId, rz: RotZ, rx: RotX, ry: RotY) -> f32 {
    let c0 = match face {
        FaceId::U => (1.0, 1.0, 2.0),
        FaceId::D => (1.0, 1.0, 0.0),
        FaceId::F => (1.0, 0.0, 1.0),
        FaceId::B => (1.0, 2.0, 1.0),
        FaceId::L => (0.0, 1.0, 1.0),
        FaceId::R => (2.0, 1.0, 1.0),
    };
    // correct order: (rz, ry, rx)
    let c = rotate_pt_all(c0, rz, ry, rx);
    let (_px, py) = project(c.0, c.1, c.2, 1.0, (0.0, 0.0));
    -py
}

/// 8 cube corners in object space (2x2x2 cube)
#[inline]
pub fn cube_corners() -> [(f32,f32,f32); 8] {
    [
        (0.0,0.0,0.0), (2.0,0.0,0.0), (0.0,2.0,0.0), (2.0,2.0,0.0),
        (0.0,0.0,2.0), (2.0,0.0,2.0), (0.0,2.0,2.0), (2.0,2.0,2.0),
    ]
}