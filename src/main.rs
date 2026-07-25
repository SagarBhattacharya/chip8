//! # CHIP-8 Emulator
//!
//! A modular CHIP-8 interpreter written in Rust.
//!
//! ## Architecture
//!
//! ```text
//! Frontend (SDL2)
//!        │
//!        ▼
//! Fetch → Decode → Execute
//!        │
//!        ▼
//! Memory • Registers • Display • Keyboard
//! ```
//!
//! The project intentionally separates instruction decoding from
//! execution, allowing the CPU to operate on semantic instructions
//! rather than raw opcodes.
//!
//! The emulator favors readability, strong typing, and modularity
//! over aggressive optimization.

use std::{env, fs};

use crate::cpu::CPU;
use crate::frontend::sdl::Frontend;

mod assets;
mod components;
mod cpu;
mod frontend;

fn main() -> Result<(), String> {
  let rom_path = env::args()
    .nth(1)
    .ok_or_else(|| {
      format!(
        "Usage: {} <rom>",
        env::args().next().unwrap_or_else(|| "chip8".into())
      )
    })?;

  let rom = fs::read(&rom_path)
    .map_err(|e| format!("Failed to read '{}': {}", rom_path, e))?;

  let mut cpu = CPU::new();
  cpu.load_rom(&rom);

  let mut frontend = Frontend::new(&format!("CHIP-8 - {}", rom_path))?;
  frontend.run(&mut cpu)
}