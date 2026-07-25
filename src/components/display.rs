/// Width of the CHIP-8 display in pixels.
pub const SCREEN_WIDTH: usize = 64;

/// Height of the CHIP-8 display in pixels.
pub const SCREEN_HEIGHT: usize = 32;

/// Monochrome 64×32 framebuffer.
///
/// Each entry in the framebuffer represents a single display row.
/// Every row is stored as a 64-bit integer where:
///
/// - Bit 63 corresponds to the leftmost pixel.
/// - Bit 0 corresponds to the rightmost pixel.
///
/// Pixels are drawn using XOR operations as defined by the
/// CHIP-8 specification.
pub struct Display {
  buffer: [u64; SCREEN_HEIGHT],
}

impl Display {
  /// Creates a cleared framebuffer.
  #[inline(always)]
  pub fn new() -> Self {
    Self { buffer: [0; SCREEN_HEIGHT] }
  }

  /// Clears the display.
  #[inline(always)]
  pub fn clear(&mut self) {
    self.buffer = [0; SCREEN_HEIGHT];
  }

  /// Returns the current framebuffer.
  ///
  /// Each element represents one display row.
  pub fn buffer(&self) -> & [u64; SCREEN_HEIGHT] {
    &self.buffer
  }

  /// Draws a single pixel using XOR semantics.
  ///
  /// Returns `true` if the pixel was previously set,
  /// indicating a collision.
  #[inline(always)]
  pub fn draw_pixel(&mut self, x: u8, y: u8, pixel: bool) -> bool {
    debug_assert!((x as usize) < SCREEN_WIDTH);
    debug_assert!((y as usize) < SCREEN_HEIGHT);

    let mask = 1u64 << (SCREEN_WIDTH - 1 - x as usize);
    let row = &mut self.buffer[y as usize];
    let prev = *row & mask;
    if pixel {
      *row ^= mask;
    }
    prev != 0
  }

  /// Draws one 8-pixel row of a sprite.
  ///
  /// The sprite byte is horizontally aligned according
  /// to `x` and XORed into the specified display row.
  ///
  /// Returns `true` if drawing caused one or more pixels
  /// to be unset due to XOR collision.
  pub fn draw_sprite_row(&mut self, x: u8, y: u8, sprite: u8, ) -> bool {
    let mut collision = false;
    for bit in 0..8 {
      let pixel = (sprite >> (7 - bit)) & 1 != 0;
      if !pixel { continue; }

      let px = x as usize + bit;
      if px >= SCREEN_WIDTH { break; }

      collision |= self.draw_pixel(px as u8, y, true);
    }
    collision
  }
}