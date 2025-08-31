// src/render/canvas.rs

//! Iced `Canvas` program that draws two cube views with depth sorting.

use iced::widget::canvas::{self, Frame, Program};
use iced::{Theme, Rectangle};

use super::types::ViewParams;
use super::face::{draw_face};
use super::layout::{layout_origins, fit_vertically};
use crate::cube::{Cube, FaceId};
use crate::render::geom::face_depth;

pub struct CubeCanvas<'a> {
    pub cube: &'a Cube,
    pub left: ViewParams,
    pub right: ViewParams,
}
impl<'a> Program<()> for CubeCanvas<'a> {
    type State = ();

    /// Draw both views into the provided canvas bounds. Auto-places and
    /// vertically fits both views to keep them within margins.
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // 1) Start with your incoming params
        let mut left  = self.left;
        let mut right = self.right;

        // 2) If origins are NaN (our signal to auto-place), give them a first pass
        if left.origin.0.is_nan() || right.origin.0.is_nan() {
            let (ol, or) = layout_origins(bounds, left.size.min(right.size));
            if left.origin.0.is_nan()  { left.origin  = ol; }
            if right.origin.0.is_nan() { right.origin = or; }
        }

        // 3) Nudge both views so the pair is vertically centered *and*
        //    still respects top/bottom margins for the current size.
        fit_vertically(bounds, &mut left, &mut right);

        // 4) Depth-sorted render with the adjusted origins
        let mut render = |vp: ViewParams| {
            let ViewParams { rz, rx, ry, origin, size } = vp;

            let mut faces = [FaceId::U, FaceId::R, FaceId::F, FaceId::D, FaceId::L, FaceId::B];
            faces.sort_by(|a, b| face_depth(*a, rz, rx, ry)
                .partial_cmp(&face_depth(*b, rz, rx, ry)).unwrap());

            for f in faces {
                draw_face(&mut frame, self.cube.face(f), f, origin, size, rz, rx, ry);
            }
        };

        render(left);
        render(right);

        vec![frame.into_geometry()]
    }
}