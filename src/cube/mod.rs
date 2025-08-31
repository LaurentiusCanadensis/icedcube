// src/cube/mod.rs

//! In-memory 2×2 cube model with face rotations (U, D, F, B, L, R).
//! The representation uses 2×2 faces and exposes move methods and getters.

// src/cube/mod.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceId { U, D, F, B, L, R }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Col { W, Y, G, B, O, R }

pub type Face = [[Col; 2]; 2];

#[derive(Debug, Clone)]
pub struct Cube {
    faces: [Face; 6], // order: U, D, F, B, L, R
}

impl Default for Cube {
    fn default() -> Self {
        use Col::*;
        // Standard color scheme:
        // U=White, D=Yellow, F=Green, B=Blue, L=Orange, R=Red
        let u = [[W, W],[W, W]];
        let d = [[Y, Y],[Y, Y]];
        let f = [[G, G],[G, G]];
        let b = [[B, B],[B, B]];
        let l = [[O, O],[O, O]];
        let r = [[R, R],[R, R]];
        Self { faces: [u, d, f, b, l, r] }
    }
}

// --------- getters used by renderer ---------

impl Cube {
    #[inline] pub fn face(&self, id: FaceId) -> &Face {
        &self.faces[id as usize]
    }
}

// --------- small helpers ---------

#[inline]
fn rot_face_cw(f: &mut Face) {
    // [[a,b],[c,d]] -> CW -> [[c,a],[d,b]]
    let a = f[0][0]; let b = f[0][1];
    let c = f[1][0]; let d = f[1][1];
    f[0][0] = c; f[0][1] = a;
    f[1][0] = d; f[1][1] = b;
}

#[inline]
fn rot_face_ccw(f: &mut Face) {
    // [[a,b],[c,d]] -> CCW -> [[b,d],[a,c]]
    let a = f[0][0]; let b = f[0][1];
    let c = f[1][0]; let d = f[1][1];
    f[0][0] = b; f[0][1] = d;
    f[1][0] = a; f[1][1] = c;
}

#[inline]
fn rot_face_180(f: &mut Face) {
    // [[a,b],[c,d]] -> 180 -> [[d,c],[b,a]]
    let a = f[0][0]; let b = f[0][1];
    let c = f[1][0]; let d = f[1][1];
    f[0][0] = d; f[0][1] = c;
    f[1][0] = b; f[1][1] = a;
}

// --------- move engine (2x2) ---------
//
// Face indexing:
//   faces[U=0], faces[D=1], faces[F=2], faces[B=3], faces[L=4], faces[R=5]
//
// Sticker indexing: [row][col] with row 0 = top, col 0 = left,
// in the orientation of the face when you look straight at it.
//

impl Cube {
    // U, U', U2
    pub fn mv_u(&mut self)          { self.u_cw(); }
    pub fn mv_u_prime(&mut self)    { self.u_ccw(); }
    pub fn mv_u2(&mut self)         { self.u_180(); }

    // D
    pub fn mv_d(&mut self)          { self.d_cw(); }
    pub fn mv_d_prime(&mut self)    { self.d_ccw(); }
    pub fn mv_d2(&mut self)         { self.d_180(); }

    // R
    pub fn mv_r(&mut self)          { self.r_cw(); }
    pub fn mv_r_prime(&mut self)    { self.r_ccw(); }
    pub fn mv_r2(&mut self)         { self.r_180(); }

    // L
    pub fn mv_l(&mut self)          { self.l_cw(); }
    pub fn mv_l_prime(&mut self)    { self.l_ccw(); }
    pub fn mv_l2(&mut self)         { self.l_180(); }

    // F
    pub fn mv_f(&mut self)          { self.f_cw(); }
    pub fn mv_f_prime(&mut self)    { self.f_ccw(); }
    pub fn mv_f2(&mut self)         { self.f_180(); }

    // B
    pub fn mv_b(&mut self)          { self.b_cw(); }
    pub fn mv_b_prime(&mut self)    { self.b_ccw(); }
    pub fn mv_b2(&mut self)         { self.b_180(); }
}

// Each move is face rotation + a 4-way cycle of edge rows/cols.
// The cycles below are chosen to work with the renderer’s face orientations.
//
// If anything looks mirrored in your specific render, swap the order marked
// with comments (“// may need reverse”); but these should match the earlier
// U/F/R identity and the D/L/B flipped view.

impl Cube {

    fn u_cw(&mut self) {
        const U: usize = FaceId::U as usize;
        const F: usize = FaceId::F as usize;
        const R: usize = FaceId::R as usize;
        const B: usize = FaceId::B as usize;
        const L: usize = FaceId::L as usize;

        rot_face_cw(&mut self.faces[U]);

        // snapshot rows
        let f0 = self.faces[F][0];
        let r0 = self.faces[R][0];
        let b0 = self.faces[B][0];
        let l0 = self.faces[L][0];

        // cycle F -> R -> B -> L -> F (top rows)
        self.faces[R][0] = f0;
        self.faces[B][0] = r0;
        self.faces[L][0] = b0;
        self.faces[F][0] = l0;
    }

    fn d_cw(&mut self) {
        const D: usize = FaceId::D as usize;
        const F: usize = FaceId::F as usize;
        const R: usize = FaceId::R as usize;
        const B: usize = FaceId::B as usize;
        const L: usize = FaceId::L as usize;

        rot_face_cw(&mut self.faces[D]);

        // snapshot rows
        let f1 = self.faces[F][1];
        let r1 = self.faces[R][1];
        let b1 = self.faces[B][1];
        let l1 = self.faces[L][1];

        // cycle F(bottom) -> L(bottom) -> B(bottom) -> R(bottom) -> F(bottom)
        self.faces[L][1] = f1;
        self.faces[B][1] = l1;
        self.faces[R][1] = b1;
        self.faces[F][1] = r1;
    }

    fn r_cw(&mut self) {
        const U: usize = FaceId::U as usize;
        const D: usize = FaceId::D as usize;
        const F: usize = FaceId::F as usize;
        const B: usize = FaceId::B as usize;
        const R: usize = FaceId::R as usize;

        rot_face_cw(&mut self.faces[R]);

        // snapshot columns (right col of U/F/D, left col of B, note reversals)
        let u_col = [self.faces[U][0][1], self.faces[U][1][1]];
        let f_col = [self.faces[F][0][1], self.faces[F][1][1]];
        let d_col = [self.faces[D][0][1], self.faces[D][1][1]];
        let b_col = [self.faces[B][0][0], self.faces[B][1][0]]; // B left

        // U right -> F right
        self.faces[F][0][1] = u_col[0];
        self.faces[F][1][1] = u_col[1];

        // F right -> D right
        self.faces[D][0][1] = f_col[0];
        self.faces[D][1][1] = f_col[1];

        // D right -> B left (reversed)
        self.faces[B][0][0] = d_col[1];
        self.faces[B][1][0] = d_col[0];

        // B left (reversed) -> U right
        self.faces[U][0][1] = b_col[1];
        self.faces[U][1][1] = b_col[0];
    }

    fn l_cw(&mut self) {
        const U: usize = FaceId::U as usize;
        const D: usize = FaceId::D as usize;
        const F: usize = FaceId::F as usize;
        const B: usize = FaceId::B as usize;
        const L: usize = FaceId::L as usize;

        rot_face_cw(&mut self.faces[L]);

        // snapshot columns (left col of U/F/D, right col of B)
        let u_col = [self.faces[U][0][0], self.faces[U][1][0]];
        let f_col = [self.faces[F][0][0], self.faces[F][1][0]];
        let d_col = [self.faces[D][0][0], self.faces[D][1][0]];
        let b_col = [self.faces[B][0][1], self.faces[B][1][1]]; // B right

        // U left -> B right (reversed)
        self.faces[B][0][1] = u_col[1];
        self.faces[B][1][1] = u_col[0];

        // B right (reversed) -> D left
        self.faces[D][0][0] = b_col[1];
        self.faces[D][1][0] = b_col[0];

        // D left -> F left
        self.faces[F][0][0] = d_col[0];
        self.faces[F][1][0] = d_col[1];

        // F left -> U left
        self.faces[U][0][0] = f_col[0];
        self.faces[U][1][0] = f_col[1];
    }

    fn f_cw(&mut self) {
        const U: usize = FaceId::U as usize;
        const D: usize = FaceId::D as usize;
        const F: usize = FaceId::F as usize;
        const L: usize = FaceId::L as usize;
        const R: usize = FaceId::R as usize;

        rot_face_cw(&mut self.faces[F]);

        // snapshot strips
        let u_bot = [self.faces[U][1][0], self.faces[U][1][1]]; // U bottom
        let r_lft = [self.faces[R][0][0], self.faces[R][1][0]]; // R left (top->bottom)
        let d_top = [self.faces[D][0][0], self.faces[D][0][1]]; // D top
        let l_rgt = [self.faces[L][0][1], self.faces[L][1][1]]; // L right (top->bottom)

        // U bottom -> R left (reversed)
        self.faces[R][0][0] = u_bot[1];
        self.faces[R][1][0] = u_bot[0];

        // R left -> D top
        self.faces[D][0][0] = r_lft[0];
        self.faces[D][0][1] = r_lft[1];

        // D top -> L right (reversed)
        self.faces[L][0][1] = d_top[1];
        self.faces[L][1][1] = d_top[0];

        // L right -> U bottom
        self.faces[U][1][0] = l_rgt[0];
        self.faces[U][1][1] = l_rgt[1];
    }

    fn b_cw(&mut self) {
        const U: usize = FaceId::U as usize;
        const D: usize = FaceId::D as usize;
        const B: usize = FaceId::B as usize;
        const L: usize = FaceId::L as usize;
        const R: usize = FaceId::R as usize;

        rot_face_cw(&mut self.faces[FaceId::B as usize]);

        // snapshot strips
        let u_top = [self.faces[U][0][0], self.faces[U][0][1]]; // U top
        let l_lft = [self.faces[L][0][0], self.faces[L][1][0]]; // L left
        let d_bot = [self.faces[D][1][0], self.faces[D][1][1]]; // D bottom
        let r_rgt = [self.faces[R][0][1], self.faces[R][1][1]]; // R right

        // U top -> L left (reversed)
        self.faces[L][0][0] = u_top[1];
        self.faces[L][1][0] = u_top[0];

        // L left -> D bottom
        self.faces[D][1][0] = l_lft[0];
        self.faces[D][1][1] = l_lft[1];

        // D bottom -> R right (reversed)
        self.faces[R][0][1] = d_bot[1];
        self.faces[R][1][1] = d_bot[0];

        // R right -> U top
        self.faces[U][0][0] = r_rgt[0];
        self.faces[U][0][1] = r_rgt[1];
    }

}


impl Cube {
    // ── U helpers ──────────────────────────────────────────────────────────────
    #[inline] fn u_ccw(&mut self) { self.u_cw(); self.u_cw(); self.u_cw(); }
    #[inline] fn u_180(&mut self) { self.u_cw(); self.u_cw(); }

    // ── D helpers ──────────────────────────────────────────────────────────────
    #[inline] fn d_ccw(&mut self) { self.d_cw(); self.d_cw(); self.d_cw(); }
    #[inline] fn d_180(&mut self) { self.d_cw(); self.d_cw(); }

    // ── R helpers ──────────────────────────────────────────────────────────────
    #[inline] fn r_ccw(&mut self) { self.r_cw(); self.r_cw(); self.r_cw(); }
    #[inline] fn r_180(&mut self) { self.r_cw(); self.r_cw(); }

    // ── L helpers ──────────────────────────────────────────────────────────────
    #[inline] fn l_ccw(&mut self) { self.l_cw(); self.l_cw(); self.l_cw(); }
    #[inline] fn l_180(&mut self) { self.l_cw(); self.l_cw(); }

    // ── F helpers ──────────────────────────────────────────────────────────────
    #[inline] fn f_ccw(&mut self) { self.f_cw(); self.f_cw(); self.f_cw(); }
    #[inline] fn f_180(&mut self) { self.f_cw(); self.f_cw(); }

    // ── B helpers ──────────────────────────────────────────────────────────────
    #[inline] fn b_ccw(&mut self) { self.b_cw(); self.b_cw(); self.b_cw(); }
    #[inline] fn b_180(&mut self) { self.b_cw(); self.b_cw(); }
}