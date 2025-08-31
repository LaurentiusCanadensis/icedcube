// src/app/mod.rs

//! Application root: message enum, app state, and `iced::Application` impl.

pub mod view;
pub mod update;
pub mod support;

use iced::{Application, Command, Element, Theme, Settings};
use crate::cube::Cube;

/// Run the interactive Iced application with default settings.
pub fn run() -> iced::Result {
    App::run(Settings::default())
}

/* ---------------- Messages ----------------
   Keep both “seed” naming schemes so update/view stay in sync
   even if one side still emits the old variants.
*/
#[derive(Debug, Clone)]
pub enum Msg {
    // camera params
    LeftRzChanged(f32), LeftRxChanged(f32), LeftRyChanged(f32),
    RightRzChanged(f32), RightRxChanged(f32), RightRyChanged(f32),
    SizeChanged(f32),

    // camera helpers
    PresetLeft,
    PresetRight,
    ResetCameras,
    ToggleSnap90(bool),

    // algorithm panel
    AlgChanged(String),
    ApplyAlg,
    ResetCube,

    // seed / scramble panel
    SeedChanged(String),

    // New names (used by your UI now)
    ApplySeed,
    Scramble,
    Reset,

    // Legacy names (still referenced by some code paths)
    SeedGenerate,
    SeedScramble,
    SeedClear,
    ToggleOpposite(bool),

    // individual move buttons
    Move(String),

    Noop,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewUI { pub rz: f32, pub rx: f32, pub ry: f32 }

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub size: f32,
    pub left: ViewUI,
    pub right: ViewUI,
}
impl Default for Params {
    fn default() -> Self {
        Self {
            // Smaller default cube size
            size: 22.0,
            // Start both cameras at (0,0,0)
            left:  ViewUI { rz: 0.0, rx: 0.0, ry: 0.0 },
            right: ViewUI { rz: 90.0, rx: 180.0, ry: 0.0 },
        }
    }
}

#[derive(Default)]
pub struct App {
    pub cube: Cube,
    pub alg_input: String,
    pub seed_input: String,
    pub status: String,
    pub params: Params,
    pub snap90: bool,

    pub link_opposite: bool,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Msg;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "2×2 Pocket Cube — Isometric 3D".into()
    }

    fn update(&mut self, msg: Msg) -> Command<Msg> {
        update::update(self, msg)
    }

    fn view(&self) -> Element<Msg> {
        view::view(self)
    }
}