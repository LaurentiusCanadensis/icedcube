// src/ui/bottom.rs

//! UI: bottom control strip (seed field + Apply/Scramble/Reset + quick move buttons).
//! Shown under the canvas. Designed to be compact and readable on narrow widths.

use iced::{
    Alignment, Element, Length,
    widget::{row, column, text, text_input, button},
};
use crate::app::Msg;

/// Bottom section: seed + Apply/Scramble/Reset + move buttons.
///
/// Displays:
/// - A **Seed** input and three action buttons.
/// - Two rows of single-move buttons for quick manual turning.
///
/// Returns an `Element<Msg>` you can place directly in your layout.
///
/// # Parameters
/// * `seed` – current seed text
/// * `on_seed_change` – callback to emit `Msg` on seed input changes
/// * `on_apply` / `on_scramble` / `on_reset` – button messages
/// * `on_move` – builder for a `Msg` given a move token like `"U"` or `"R'"`
pub fn bottom_section<'a, FMove>(
    seed: &str,
    on_seed_change: fn(String) -> Msg,
    on_apply: Msg,
    on_scramble: Msg,
    on_reset: Msg,
    on_move: FMove,
) -> Element<'a, Msg>
where
    FMove: Fn(&'static str) -> Msg + Copy + 'a,
{
    // Seed row: label + input + 3 buttons
    let seed_row = row![
        text("Seed"),
        text_input("Enter seed (e.g., 12345)", seed)
            .on_input(on_seed_change)
            .width(Length::Fixed(220.0)),
        button("Apply").on_press(on_apply),
        button("Scramble").on_press(on_scramble),
        button("Reset").on_press(on_reset),
    ]
        .spacing(8)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    // Top row of face turns
    let moves_top = row![
        mv("U", on_move),  mv("U'", on_move), mv("U2", on_move),
        mv("R", on_move),  mv("R'", on_move), mv("R2", on_move),
        mv("F", on_move),  mv("F'", on_move), mv("F2", on_move),
    ]
        .spacing(8)
        .align_items(Alignment::Center);

    // Bottom row of face turns
    let moves_bot = row![
        mv("D", on_move),  mv("D'", on_move), mv("D2", on_move),
        mv("L", on_move),  mv("L'", on_move), mv("L2", on_move),
        mv("B", on_move),  mv("B'", on_move), mv("B2", on_move),
    ]
        .spacing(8)
        .align_items(Alignment::Center);

    column![seed_row, text("Moves"), moves_top, moves_bot]
        .spacing(10)
        .width(Length::Fill)
        .into()
}

#[inline]
fn mv<'a, F>(label: &'static str, on_move: F) -> Element<'a, Msg>
where
    F: Fn(&'static str) -> Msg + Copy + 'a,
{
    // Tiny helper to keep button creation terse.
    button(label).on_press(on_move(label)).into()
}