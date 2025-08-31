// src/ui/panels.rs

//! UI: parameter panels (angles, presets, seed & algorithm).

use iced::{
    Alignment, Element, Length,
    widget::{row, column, text, text_input, button, checkbox, slider},
};
use crate::app::Msg;

/// Builds a labeled trio of angle sliders (Rz, Rx, Ry) with live values.
pub fn build_angle_block<
    Fz: Fn(f32) -> Msg + 'static + Copy,
    Fx: Fn(f32) -> Msg + 'static + Copy,
    Fy: Fn(f32) -> Msg + 'static + Copy,
>(
    title: &str,
    rz: f32, rx: f32, ry: f32,
    on_rz: Fz, on_rx: Fx, on_ry: Fy,
) -> Element<'static, Msg> {
    column![
        text(title),
        row![ text("Rz"), slider(0.0..=360.0, rz, on_rz).step(1.0), text(format!("{:.0}°", rz)) ].spacing(8),
        row![ text("Rx"), slider(0.0..=360.0, rx, on_rx).step(1.0), text(format!("{:.0}°", rx)) ].spacing(8),
        row![ text("Ry"), slider(0.0..=360.0, ry, on_ry).step(1.0), text(format!("{:.0}°", ry)) ].spacing(8),
    ]
        .spacing(6)
        .into()
}

/// Preset camera buttons + Snap-90° toggle row.
pub fn build_presets_row(snap90: bool) -> Element<'static, Msg> {
    row![
        button("Preset Left U/F/R").on_press(Msg::PresetLeft),
        button("Preset Right D/L/B").on_press(Msg::PresetRight),
        button("Reset Cameras").on_press(Msg::ResetCameras),
        checkbox("Snap 90°", snap90).on_toggle(Msg::ToggleSnap90),
    ]
        .spacing(12)
        .align_items(Alignment::Center)
        .into()
}

/// Seed input + Apply/Scramble/Reset buttons (same actions used elsewhere).
pub fn build_seed_panel(seed: &str) -> Element<'static, Msg> {
    row![
        text("Seed"),
        text_input("Enter seed (e.g., 12345)", seed)
            .on_input(Msg::SeedChanged)
            .width(Length::Fixed(220.0)),
        button("Apply").on_press(Msg::ApplySeed),
        button("Scramble").on_press(Msg::Scramble),
        button("Reset").on_press(Msg::Reset),
    ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
}

/// Text field for an algorithm string (e.g., `R U R' U'`).
/// Currently wires to `Msg::SeedChanged` as a placeholder emitter.
pub fn build_algorithm_panel(alg: &str) -> Element<'static, Msg> {
    // If/when you add a dedicated AlgChanged, swap the .on_input handler.
    row![
        text_input("Algorithm (e.g., R U R' U')", alg)
            .on_input(Msg::SeedChanged) // placeholder to keep compiling
            .width(Length::Fill),
    ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
}