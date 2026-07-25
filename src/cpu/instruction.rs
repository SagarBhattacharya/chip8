use crate::components::Register;

/// A decoded CHIP-8 instruction.
///
/// The decoder translates a raw 16-bit opcode into one of these semantic
/// instructions. The CPU executor operates on this representation instead
/// of raw opcodes, improving readability and simplifying debugging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
  /// `00E0` - Clear the display.
  ClearScreen,

  /// `00EE` - Return from the current subroutine.
  Return,

  /// `1NNN` - Jump to the specified address.
  Jump {
    addr: u16,
  },

  /// `2NNN` - Call the subroutine at the specified address.
  Call {
    addr: u16,
  },

  /// `3XNN` - Skip the next instruction if `VX == NN`.
  SkipEqImm {
    reg: Register,
    value: u8,
  },

  /// `4XNN` - Skip the next instruction if `VX != NN`.
  SkipNeImm {
    reg: Register,
    value: u8,
  },

  /// `5XY0` - Skip the next instruction if `VX == VY`.
  SkipEqReg {
    lhs: Register,
    rhs: Register,
  },

  /// `6XNN` - Load an immediate value into `VX`.
  LoadImm {
    reg: Register,
    value: u8,
  },

  /// `7XNN` - Add an immediate value to `VX`.
  AddImm {
    reg: Register,
    value: u8,
  },

  /// `8XY0` - Copy `VY` into `VX`.
  Move {
    dst: Register,
    src: Register,
  },

  /// `8XY1` - Compute `VX |= VY`.
  Or {
    dst: Register,
    src: Register,
  },

  /// `8XY2` - Compute `VX &= VY`.
  And {
    dst: Register,
    src: Register,
  },

  /// `8XY3` - Compute `VX ^= VY`.
  Xor {
    dst: Register,
    src: Register,
  },

  /// `8XY4` - Add `VY` to `VX`, storing the carry in `VF`.
  Add {
    dst: Register,
    src: Register,
  },

  /// `8XY5` - Subtract `VY` from `VX`, storing the borrow flag in `VF`.
  Sub {
    dst: Register,
    src: Register,
  },

  /// `8XY6` - Shift `VX` right by one bit.
  ShiftRight {
    reg: Register,
  },

  /// `8XY7` - Compute `VX = VY - VX`, storing the borrow flag in `VF`.
  SubN {
    dst: Register,
    src: Register,
  },

  /// `8XYE` - Shift `VX` left by one bit.
  ShiftLeft {
    reg: Register,
  },

  /// `9XY0` - Skip the next instruction if `VX != VY`.
  SkipNeReg {
    lhs: Register,
    rhs: Register,
  },
  
  /// `ANNN` - Load the index register with the specified address.
  LoadIndex {
    addr: u16,
  },

  /// `BNNN` - Jump to `NNN + V0`.
  JumpV0 {
    addr: u16,
  },

  /// `CXNN` - Generate a random byte and mask it with `NN`.
  Random {
    reg: Register,
    mask: u8,
  },

  /// `DXYN` - Draw an `N`-byte sprite at (`VX`, `VY`).
  Draw {
    x: Register,
    y: Register,
    height: u8,
  },

  /// `EX9E` - Skip the next instruction if the key stored in `VX` is pressed.
  SkipKeyPressed {
    reg: Register,
  },

  /// `EXA1` - Skip the next instruction if the key stored in `VX` is not pressed.
  SkipKeyNotPressed {
    reg: Register,
  },

  /// `FX07` - Load the current delay timer value into `VX`.
  ReadDelay {
    reg: Register,
  },

  /// `FX0A` - Wait for a key press and store it in `VX`.
  WaitKey {
    reg: Register,
  },

  /// `FX15` - Set the delay timer from `VX`.
  WriteDelay {
    reg: Register,
  },

  /// `FX18` - Set the sound timer from `VX`.
  WriteSound {
    reg: Register,
  },
  
  /// `FX1E` - Add `VX` to the index register.
  AddIndex {
    reg: Register,
  },

  /// `FX29` - Load the address of the sprite for `VX` into the index register.
  LoadFont {
    reg: Register,
  },

  /// `FX33` - Store the BCD representation of `VX` in memory at `I`.
  StoreBCD {
    reg: Register,
  },

  /// `FX55` - Store registers `V0..VX` into memory starting at `I`.
  StoreRegisters {
    last: Register,
  },

  /// `FX65` - Load registers `V0..VX` from memory starting at `I`.
  LoadRegisters {
    last: Register,
  },

  /// An instruction that is not recognized by the decoder.
  Unknown(u16),
}