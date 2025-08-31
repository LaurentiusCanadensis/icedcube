// src/app/update.rs

//! Central update loop: handles all `Msg` variants and mutates `App` state.

use iced::Command;
use rand::{thread_rng, RngCore};

use crate::cube::Cube;
use crate::logic::scramble::scramble_with_seed;

use super::{App, Msg};
use super::support::{set_deg, apply_alg, apply_token};

/// Default length for generated scrambles.
const SCRAMBLE_LEN: usize = 15;

// --------- helpers ----------------------------------------------------------

/// Normalize degrees into `[0, 360)`.
#[inline]
fn wrap_deg(mut v: f32) -> f32 {
    v = v % 360.0;
    if v < 0.0 { v += 360.0; }
    v
}

// When the link is ON, compute the opposite-facing right view from left,
// matching the D/L/B “opposite” feel (rotate ~180° about X).
fn sync_right_from_left(app: &mut App) {
    if app.link_opposite {
        app.params.right.rz = app.params.left.rz;
        app.params.right.ry = app.params.left.ry;
        app.params.right.rx = wrap_deg(app.params.left.rx + 180.0);
    }
}

// Apply a text algorithm to the cube; update status accordingly.
fn try_apply_alg(app: &mut App, alg: &str) {
    if alg.trim().is_empty() {
        app.status = "Nothing to apply. Enter an algorithm or a seed.".into();
        return;
    }
    match apply_alg(&mut app.cube, alg) {
        Ok(()) => app.status = "Applied algorithm.".into(),
        Err(e) => app.status = format!("Algorithm error: {e}"),
    }
}

// Produce a deterministic scramble from a seed and apply it.
// Also stores the textual sequence into `alg_input` for visibility.
fn apply_seeded_scramble(app: &mut App, seed: u64) {
    let seq = scramble_with_seed(SCRAMBLE_LEN, seed);
    app.cube = Cube::default();
    match apply_alg(&mut app.cube, &seq) {
        Ok(()) => {
            app.alg_input = seq.clone();
            app.status = format!("Applied seed = {seed}: {seq}");
        }
        Err(e) => app.status = format!("Seeded scramble error: {e}"),
    }
}

// --------- main update ------------------------------------------------------

/// Handle one `Msg` and update `app` state. Returns any follow-up command.
pub fn update(app: &mut App, msg: Msg) -> Command<Msg> {
    match msg {
        // ----- cameras (left, drives right when linked) ----------
        Msg::LeftRzChanged(v) => { app.params.left.rz = set_deg(v, app.snap90); sync_right_from_left(app); }
        Msg::LeftRxChanged(v) => { app.params.left.rx = set_deg(v, app.snap90); sync_right_from_left(app); }
        Msg::LeftRyChanged(v) => { app.params.left.ry = set_deg(v, app.snap90); sync_right_from_left(app); }

        // Right edits break the link (one-way sync to avoid ping-pong loops)
        Msg::RightRzChanged(v) => { app.link_opposite = false; app.params.right.rz = set_deg(v, app.snap90); }
        Msg::RightRxChanged(v) => { app.link_opposite = false; app.params.right.rx = set_deg(v, app.snap90); }
        Msg::RightRyChanged(v) => { app.link_opposite = false; app.params.right.ry = set_deg(v, app.snap90); }

        Msg::SizeChanged(v) => { app.params.size = v; }

        // Presets / reset cameras
        Msg::PresetLeft => {
            // Left (cube 1) at 0,0,0
            app.params.left  = super::ViewUI { rz: 0.0,  rx: 0.0,   ry: 0.0 };
            // If linked, drive right from left
            sync_right_from_left(app);
        }
        Msg::PresetRight => {
            // Right (cube 2) at 90,180,0 (your requested pose)
            app.params.right = super::ViewUI { rz: 90.0, rx: 180.0, ry: 0.0 };
            // Manual edit → unlink (so this doesn't bounce back)
            app.link_opposite = false;
        }
        Msg::ResetCameras => {
            app.params.left  = super::ViewUI { rz: 0.0,  rx: 0.0,   ry: 0.0 };
            app.params.right = super::ViewUI { rz: 90.0, rx: 180.0, ry: 0.0 };
            // Keep current link flag as-is
            if app.link_opposite { sync_right_from_left(app); }
        }

        // Snap 90°
        Msg::ToggleSnap90(on) => { app.snap90 = on; }

        // Opposite-right link toggle
        Msg::ToggleOpposite(on) => {
            app.link_opposite = on;
            if on { sync_right_from_left(app); }
        }

        // ----- algorithm / seed actions ------------------------------------

        Msg::AlgChanged(s) => { app.alg_input = s; }

        Msg::ApplyAlg => {
            // Own the text so we can mutably borrow `app` below
            let alg_text = app.alg_input.trim().to_owned();

            if !alg_text.is_empty() {
                // Now safe: `alg_text` doesn't borrow from `app`
                try_apply_alg(app, &alg_text);
            } else if let Ok(seed) = app.seed_input.trim().parse::<u64>() {
                apply_seeded_scramble(app, seed);
            } else {
                app.status = "Nothing to apply. Enter an algorithm or a seed.".into();
            }
        }

        Msg::ResetCube => {
            app.cube = Cube::default();
            app.status = "Cube reset.".into();
        }

        Msg::SeedChanged(s) => { app.seed_input = s; }

        Msg::ApplySeed => {
            if let Ok(seed) = app.seed_input.trim().parse::<u64>() {
                apply_seeded_scramble(app, seed);
            } else {
                app.status = "No valid seed entered — nothing applied.".into();
            }
        }

        Msg::Scramble => {
            // Pick a brand-new random seed and apply a deterministic scramble.
            let mut rng = thread_rng();
            let seed = rng.next_u64();                // no recursion, no gen() ambiguity
            app.seed_input = seed.to_string();
            apply_seeded_scramble(app, seed);
        }

        // “Reset” for the seed/algorithm inputs (kept for convenience)
        Msg::Reset => {
            app.alg_input.clear();
            app.seed_input.clear();
            app.status.clear();
        }

        Msg::SeedClear => { app.seed_input.clear(); }

        // ----- single move buttons -----------------------------------------
        Msg::Move(tok) => {
            match apply_token(&mut app.cube, &tok) {
                Ok(()) => app.status = format!("Did {tok}"),
                Err(e)  => app.status = e,
            }
        }

        Msg::Noop => {}
        _ => {}
    }

    Command::none()
}