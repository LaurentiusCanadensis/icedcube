// src/app/view.rs

//! Top-level view layout: canvas, controls, sliders, toggles, and status.

use iced::{
    Alignment, Element, Length,
    widget::{column, row, text, container, Space, Canvas, slider, checkbox},
};

use super::{App, Msg};
use crate::render::{CubeCanvas, ViewParams, RotZ, RotX, RotY};
use crate::ui::{
    build_angle_block,
    build_presets_row,
    build_seed_panel,
    build_algorithm_panel,
};
use crate::ui::moves::build_moves_scroller;

/// Build the full UI tree for the current `App` state.
pub fn view(app: &App) -> Element<Msg> {
    // Build ViewParams for the renderer. Origins are auto-laid out in render code when NaN.
    let left  = ViewParams {
        rz: RotZ(app.params.left.rz),
        rx: RotX(app.params.left.rx),
        ry: RotY(app.params.left.ry),
        origin: (f32::NAN, f32::NAN),
        size: app.params.size,
    };
    let right = ViewParams {
        rz: RotZ(app.params.right.rz),
        rx: RotX(app.params.right.rx),
        ry: RotY(app.params.right.ry),
        origin: (f32::NAN, f32::NAN),
        size: app.params.size,
    };

    // ── Fixed canvas area so controls never get squeezed ─────────────
    const CANVAS_H: f32 = 320.0; // stable space for both cubes
    let canvas_raw: Element<()> = Canvas::new(CubeCanvas { cube: &app.cube, left, right })
        .width(Length::Fill)
        .height(Length::Fixed(CANVAS_H))
        .into();

    // Give the compiler an explicit type to avoid inference errors (E0283).
    let canvas_el: Element<Msg> = container(canvas_raw.map(|_| Msg::Noop))
        .width(Length::Fill)
        .center_x()
        .into();

    // ── Size slider (16..=40) ───────────────────────────────────────
    let size_row = row![
        text("Size"),
        Space::with_width(8),
        slider(16.0..=40.0, app.params.size, Msg::SizeChanged)
            .step(1.0)
            .width(Length::Fill),
        Space::with_width(12),
        text(format!("{:.0}px", app.params.size)),
    ]
        .spacing(8)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    // ── Angle blocks ─────────────────────────────────────────────────
    let angles_row = row![
        build_angle_block(
            "Left view",
            app.params.left.rz, app.params.left.rx, app.params.left.ry,
            Msg::LeftRzChanged, Msg::LeftRxChanged, Msg::LeftRyChanged
        ),
        Space::with_width(16),
        build_angle_block(
            "Right view",
            app.params.right.rz, app.params.right.rx, app.params.right.ry,
            Msg::RightRzChanged, Msg::RightRxChanged, Msg::RightRyChanged
        ),
    ]
        .spacing(12)
        .width(Length::Fill);

    // ── Presets + “Opposite right” toggle ────────────────────────────
    let presets = build_presets_row(app.snap90);
    let presets_row = row![
        presets,
        Space::with_width(12),
        checkbox("Opposite right", app.link_opposite)
            .on_toggle(Msg::ToggleOpposite),
    ]
        .spacing(12)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    // ── Moves (scrollable for small screens) ─────────────────────────
    let moves_scroller = build_moves_scroller();

    // ── Seed / Algorithm panels ──────────────────────────────────────
    let seed_panel = build_seed_panel(&app.seed_input);
    let alg_panel  = build_algorithm_panel(&app.alg_input);

    // ── Info + status line ───────────────────────────────────────────
    let info = text(format!(
        "Left (Rz,Rx,Ry)=({:.0}°, {:.0}°, {:.0}°)   Right (Rz,Rx,Ry)=({:.0}°, {:.0}°, {:.0}°)   Size={:.0}px",
        app.params.left.rz,  app.params.left.rx,  app.params.left.ry,
        app.params.right.rz, app.params.right.rx, app.params.right.ry,
        app.params.size
    ));

    let status: Element<Msg> = if app.status.is_empty() {
        Space::with_height(0).into()
    } else {
        container(text(&app.status))
            .width(Length::Fill)
            .padding([0, 2, 2, 2])
            .into()
    };

    // ── Title (smaller, centered) ────────────────────────────────────
    let title: Element<Msg> = container(text("2×2 Pocket Cube — Isometric 3D").size(24))
        .width(Length::Fill)
        .center_x()
        .into();

    // ── Layout ───────────────────────────────────────────────────────
    column![
        title,
        canvas_el,
        Space::with_height(8),
        size_row,
        container(
            column![
                angles_row,
                presets_row,
                moves_scroller,
                row![seed_panel, Space::with_width(16), alg_panel].spacing(16),
                info,
                status,
            ]
            .spacing(10)
            .width(Length::Fill)
        )
        .padding(10)
        .width(Length::Fill),
    ]
        .spacing(12)
        .width(Length::Fill)
        .into()
}