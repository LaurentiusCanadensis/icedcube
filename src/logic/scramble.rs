// src/logic/scramble.rs

//! Scramble utilities: random and seeded deterministic sequences.

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

const MOVES: &[&str] = &[
    "U","U'","U2","D","D'","D2","R","R'","R2","L","L'","L2","F","F'","F2","B","B'","B2",
];

/// Generate a random scramble of `len` tokens using thread RNG.
pub fn random_scramble(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| *MOVES.choose(&mut rng).unwrap()).collect::<Vec<_>>().join(" ")
}

/// Generate a deterministic scramble of `len` tokens from a `seed`.
pub fn scramble_with_seed(len: usize, seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..len).map(|_| *MOVES.choose(&mut rng).unwrap()).collect::<Vec<_>>().join(" ")
}