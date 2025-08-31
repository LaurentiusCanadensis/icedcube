# 2×2 Pocket Cube — Isometric 3D

An interactive Rust/Iced application that renders a 2×2 Rubik’s Cube in an
isometric view. It includes:

- **Canvas renderer**: draws the cube with stickers and seams.
- **Controls**: change cube size, camera angles, and presets.
- **Moves**: buttons and scrollable move panels (`U`, `R'`, `F2`, etc.).
- **Seed scrambles**: apply deterministic or random scrambles by seed.
- **Reset & Opposite-right link**: quickly restore or mirror camera views.

---

## Installing Rust

This project uses the [Rust programming language](https://www.rust-lang.org/).

### 1. Install Rust via `rustup`

On **macOS** / **Linux** / **WSL**:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh