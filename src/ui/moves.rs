// src/ui/moves.rs

//! UI: compact, horizontally scrollable move buttons (two rows).

use iced::{
    Element, Length,
    widget::{button, row, column, text, scrollable},
};
use crate::app::Msg;

/// Create a compact button for a single move token (e.g., `"R'"`).
fn btn(tok: &'static str) -> iced::widget::Button<'static, Msg> {
    button(text(tok).size(14))      // smaller label
        .padding([4, 8])            // tighter padding
        .on_press(Msg::Move(tok.to_string()))
        .width(Length::Shrink)
}

/// Two compact horizontal scrollers so they don’t explode at small widths.
///
/// The rows are split (U/R/F and D/L/B) to avoid overly wide single rows.
pub fn build_moves_scroller() -> Element<'static, Msg> {
    let row1 = ["U", "U'", "U2", "R", "R'", "R2", "F", "F'", "F2"]
        .into_iter().map(btn).fold(row![], |r, b| r.push(b))
        .spacing(6);

    let row2 = ["D", "D'", "D2", "L", "L'", "L2", "B", "B'", "B2"]
        .into_iter().map(btn).fold(row![], |r, b| r.push(b))
        .spacing(6);

    let props = scrollable::Properties::default();

    let sc1 = scrollable(row1)
        .direction(scrollable::Direction::Horizontal(props))
        .height(Length::Shrink);

    let sc2 = scrollable(row2)
        .direction(scrollable::Direction::Horizontal(props))
        .height(Length::Shrink);

    column![text("Moves").size(16), sc1, sc2]
        .spacing(6)
        .into()
}