// src/app/support.rs

//! Stateless helpers used by `app::update`: angle clamping and move parsing.

use crate::cube::Cube;

/// Clamp/snap an angle in degrees to `[0, 360)`; optionally snap to 90°.
pub fn set_deg(v: f32, snap90: bool) -> f32 {
    let mut d = if snap90 {
        let r = (v / 90.0).round() * 90.0;
        if r < 0.0 { r + 360.0 } else { r % 360.0 }
    } else {
        v % 360.0
    };
    if d < 0.0 { d += 360.0; }
    d
}

/// Apply a space-separated algorithm to the cube, e.g. `"R U R' U'"`.
///
/// Returns `Err` if any token is unknown.
pub fn apply_alg(cube: &mut Cube, alg: &str) -> Result<(), String> {
    for tok in alg.split_whitespace() {
        apply_token(cube, tok)?;
    }
    Ok(())
}

/// Apply a single token like `"R"`, `"R'"`, or `"R2"`.
pub fn apply_token(cube: &mut Cube, tok: &str) -> Result<(), String> {
    match tok {
        "U"  => { cube.mv_u(); Ok(()) }
        "U'" => { cube.mv_u_prime(); Ok(()) }
        "U2" => { cube.mv_u2(); Ok(()) }

        "D"  => { cube.mv_d(); Ok(()) }
        "D'" => { cube.mv_d_prime(); Ok(()) }
        "D2" => { cube.mv_d2(); Ok(()) }

        "R"  => { cube.mv_r(); Ok(()) }
        "R'" => { cube.mv_r_prime(); Ok(()) }
        "R2" => { cube.mv_r2(); Ok(()) }

        "L"  => { cube.mv_l(); Ok(()) }
        "L'" => { cube.mv_l_prime(); Ok(()) }
        "L2" => { cube.mv_l2(); Ok(()) }

        "F"  => { cube.mv_f(); Ok(()) }
        "F'" => { cube.mv_f_prime(); Ok(()) }
        "F2" => { cube.mv_f2(); Ok(()) }

        "B"  => { cube.mv_b(); Ok(()) }
        "B'" => { cube.mv_b_prime(); Ok(()) }
        "B2" => { cube.mv_b2(); Ok(()) }

        other => Err(format!("Unknown move: {other}")),
    }
}