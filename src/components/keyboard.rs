/// A key on the CHIP-8 hexadecimal keypad.
///
/// CHIP-8 provides sixteen keys arranged in a 4×4 layout:
///
/// ```text
/// ┌────┬────┬────┬────┐
/// │ 1  │ 2  │ 3  │ C  │
/// ├────┼────┼────┼────┤
/// │ 4  │ 5  │ 6  │ D  │
/// ├────┼────┼────┼────┤
/// │ 7  │ 8  │ 9  │ E  │
/// ├────┼────┼────┼────┤
/// │ A  │ 0  │ B  │ F  │
/// └────┴────┴────┴────┘
/// ```
///
/// Internally, each key corresponds to its hexadecimal value
/// (`0x0..=0xF`).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
  K0, K1, K2, K3,
  K4, K5, K6, K7,
  K8, K9, KA, KB,
  KC, KD, KE, KF,
}

impl Key {
  /// Converts a hexadecimal key value (`0x0..=0xF`) into a [`Key`].
  ///
  /// Returns `None` if the value is outside the valid CHIP-8 key range.
  pub fn from_nibble(nibble: u8) -> Option<Self> {
    if nibble < 16 {
      Some(unsafe { std::mem::transmute(nibble) })
    } else {
      None
    }
  }

  /// Returns the numeric key index (`0..15`).
  ///
  /// This is primarily used when manipulating the keyboard bitmap.
  #[inline(always)]
  pub const fn index(self) -> usize {
    self as usize
  }
}

/// Represents the CHIP-8 hexadecimal keypad.
///
/// The keypad state is stored as a 16-bit bitmap where each bit
/// corresponds to one key.
///
/// - Bit `0`  → `K0`
/// - Bit `1`  → `K1`
/// - ...
/// - Bit `15` → `KF`
///
/// A set bit indicates that the corresponding key is currently pressed.
pub struct Keyboard {
  keys: u16,
}

impl Keyboard {
  /// Creates a new keyboard with all keys released.
  #[inline(always)]
  pub fn new() -> Self {
    Self { keys: 0 }
  }

  /// Returns `true` if the specified key is currently pressed.
  #[inline(always)]
  pub fn is_pressed(&self, key: Key) -> bool {
    (self.keys & (1 << key.index())) != 0
  }

  /// Returns one currently pressed key, if any.
  ///
  /// If multiple keys are pressed simultaneously, the highest-numbered
  /// pressed key is returned.
  ///
  /// Returns `None` if no keys are currently pressed.
  pub fn any_pressed(&self) -> Option<Key> {
    if self.keys == 0 {
      None
    } else {
      Key::from_nibble(self.keys.ilog2() as u8)
    }
  }

  /// Marks the specified key as pressed.
  #[inline(always)]
  pub fn press(&mut self, key: Key) {
    self.keys |= 1 << key.index();
  }

  /// Marks the specified key as released.
  #[inline(always)]
  pub fn release(&mut self, key: Key) {
    self.keys &= !(1 << key.index());
  }

  /// Releases all currently pressed keys.
  #[inline(always)]
  pub fn release_all(&mut self) {
    self.keys = 0;
  }
}