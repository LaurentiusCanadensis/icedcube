// src/render/types.rs

//! Basic render types: typed angle wrappers and per-view parameters.

#[derive(Copy, Clone, Debug)]
pub struct RotZ(pub f32);
#[derive(Copy, Clone, Debug)]
pub struct RotX(pub f32);
#[derive(Copy, Clone, Debug)]
pub struct RotY(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct ViewParams {
    pub rz: RotZ,
    pub rx: RotX,
    pub ry: RotY,
    pub origin: (f32, f32),
    pub size: f32,
}