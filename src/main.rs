// src/main.rs

//! # 2×2 Pocket Cube — Isometric 3D
//!
//! An interactive Rust/Iced app that renders a 2×2 Rubik’s Cube in a
//! software-drawn isometric canvas, with move buttons, seed scrambles, and
//! camera controls (angles, presets, snap-to-90°, opposite-right link).

mod render;
mod ui;
mod logic;
pub mod cube;
mod app;

use iced::widget::{
    column, row, text, slider, text_input, button, container, Space, Canvas, checkbox,
};
use iced::{Alignment, Length, Element, Application, Settings, Command, Theme};

use render::{CubeCanvas, ViewParams, RotZ, RotX, RotY};

use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::app::{App, Msg};

/// Launch the app.
fn main() -> iced::Result {
    App::run(Settings::default())
}