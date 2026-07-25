/// General-purpose CHIP-8 register.
///
/// The CHIP-8 architecture provides sixteen 8-bit registers named
/// `V0` through `VF`.
///
/// Most registers are available for general use by programs.
/// Register `VF` has a special meaning and is implicitly modified by
/// several instructions to indicate:
///
/// - Carry after arithmetic (`8XY4`)
/// - Borrow after subtraction (`8XY5`, `8XY7`)
/// - Sprite collision after drawing (`DXYN`)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
  V0, V1, V2, V3,
  V4, V5, V6, V7,
  V8, V9, VA, VB,
  VC, VD, VE, VF,
}

impl Register {
  /// Converts a 4-bit register index (`0x0..=0xF`) into a [`Register`].
  ///
  /// Returns `None` if the value is outside the valid CHIP-8 register range.
  pub fn from_nibble(nibble: u8) -> Option<Self> {
    if nibble < 16 {
      Some(unsafe { std::mem::transmute(nibble) })
    } else {
      None
    }
  }

  /// Returns the numeric register index (`0..15`).
  ///
  /// This is primarily used for indexing into the register file.
  #[inline(always)]
  pub const fn index(self) -> usize {
    self as usize
  }
}

/// The CHIP-8 register file.
///
/// Stores the sixteen 8-bit general-purpose registers (`V0..VF`).
///
/// Access is performed through the strongly-typed [`Register`] enum
/// instead of raw integer indices to improve readability and prevent
/// accidental misuse.
pub struct RegisterFile {
  regs: [u8; 16],
}

impl RegisterFile {
  /// Creates a new register file with all registers initialized to zero.
  #[inline(always)]
  pub fn new() -> Self {
    Self { regs: [0; 16] }
  }

  /// Returns the current value stored in the given register.
  #[inline(always)]
  pub fn get(&self, reg: Register) -> u8 {
    self.regs[reg.index()]
  }

  /// Writes a value into the given register.
  #[inline(always)]
  pub fn set(&mut self, reg: Register, value: u8) {
    self.regs[reg.index()] = value;
  }

  /// Sets the value of the `VF` flag register.
  ///
  /// `true` writes `1`, while `false` writes `0`.
  #[inline(always)]
  pub fn set_flag(&mut self, value: bool) {
    self.regs[Register::VF.index()] = value as u8;
  }
}