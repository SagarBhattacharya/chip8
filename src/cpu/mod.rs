pub mod instruction;
pub mod decoder;
pub mod executor;

use rand::rngs::ThreadRng;
use rand::rng;
use crate::assets::FONTS;
use crate::components::*;
use crate::cpu::decoder::decode;
use crate::cpu::executor::execute;

pub const MEMORY_START: usize = 0x0200;
pub const FONT_START: usize = 0x0050;

/// The central processing unit of the CHIP-8 virtual machine.
///
/// The CPU owns and coordinates all emulator state, including:
///
/// - General-purpose registers
/// - Main memory
/// - Display framebuffer
/// - Keyboard state
/// - Timers
/// - Call stack
///
/// Instruction execution follows the classic fetch-decode-execute cycle:
///
/// ```text
/// Memory
///    │
///    ▼
/// Fetch
///    │
///    ▼
/// Decode
///    │
///    ▼
/// Execute
/// ```
///
/// Decoding and execution are delegated to dedicated modules,
/// keeping the CPU focused on orchestration rather than instruction logic.
pub struct CPU {
  registers: RegisterFile,
  memory: Memory,
  index: u16,
  program_counter: u16,
  stack: Vec<u16>,
  display: Display,
  pub delay_timer: u8,
  pub sound_timer: u8,
  pub keypad: Keyboard,
  rand_engine: ThreadRng,
}

impl CPU {
  /// Creates a new CHIP-8 CPU in its reset state.
  ///
  /// The built-in font set is automatically loaded into memory
  /// beginning at [`FONT_START`].
  pub fn new() -> CPU {
    let mut cpu = Self {
      registers: RegisterFile::new(),
      memory: Memory::new(),
      index: 0,
      program_counter: MEMORY_START as u16,
      stack: Vec::new(),
      display: Display::new(),
      delay_timer: 0,
      sound_timer: 0,
      keypad: Keyboard::new(),
      rand_engine: rng()
    };

    // load fonts
    cpu.memory.load_bytes(FONT_START as u16, &FONTS);
    cpu
  }

  /// Loads a CHIP-8 program into memory.
  ///
  /// Programs are copied beginning at [`MEMORY_START`], leaving
  /// the lower memory reserved for the interpreter and font data.
  pub fn load_rom(&mut self, rom: &[u8]) {
    self.memory.load_bytes(MEMORY_START as u16, rom);
  }

  /// Returns the current display framebuffer.
  ///
  /// Each element represents one display row encoded as a `u64`.
  pub fn framebuffer(&self) -> &[u64; 32] {
    self.display.buffer()
  }

  /// Fetches the next 16-bit instruction from memory.
  ///
  /// CHIP-8 instructions are stored in big-endian format and occupy
  /// two bytes. The program counter is advanced to the following
  /// instruction after the fetch.
  fn fetch(&mut self) -> u16 {
    let opcode = self.memory.read_16_be(self.program_counter);
    self.program_counter += 2;
    opcode
  }

  /// Executes one complete CPU cycle.
  ///
  /// A cycle consists of:
  ///
  /// 1. Fetching the next opcode from memory.
  /// 2. Decoding the opcode into a semantic instruction.
  /// 3. Executing the decoded instruction.
  pub fn cycle(&mut self) {
    let opcode = self.fetch();
    let instruction = decode(opcode);
    execute(self, instruction);
  }

  pub fn press_key(&mut self, key: Key) {
    self.keypad.press(key);
  }

  pub fn release_key(&mut self, key: Key) {
    self.keypad.release(key);
  }

  pub fn tick_timers(&mut self) {
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }

    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }
  }
}