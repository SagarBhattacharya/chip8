/// Total addressable memory of the CHIP-8 virtual machine.
///
/// CHIP-8 provides 4 KB of byte-addressable memory
/// spanning addresses `0x000..=0xFFF`.
pub const MEMORY_SIZE: usize = 4096;

/// Represents the main memory of the CHIP-8 virtual machine.
///
/// Memory is byte-addressable and is used to store:
///
/// - The interpreter font set
/// - The loaded program (ROM)
/// - Runtime variables
/// - Sprite data
///
/// The memory layout itself is not enforced by this type.
/// Address management is the responsibility of the CPU.
pub struct Memory {
  mem: [u8; MEMORY_SIZE],
}

impl Memory {
  /// Creates a zero-initialized memory.
  #[inline(always)]
  pub fn new() -> Self {
    Self { mem: [0; MEMORY_SIZE] }
  }

  /// Reads a single byte from the given memory address.
  ///
  /// Panics in debug builds if the address is outside
  /// the valid CHIP-8 address space.
  #[inline(always)]
  pub fn read_byte(&self, addr: u16) -> u8 {
    debug_assert!(
      addr < MEMORY_SIZE as u16,
      "Memory read out of bounds: {:04X}",
      addr
    );

    self.mem[addr as usize]
  }

  /// Writes a single byte to the given memory address.
  ///
  /// Panics in debug builds if the address is outside
  /// the valid CHIP-8 address space.
  #[inline(always)]
  pub fn write_byte(&mut self, addr: u16, val: u8) {
    debug_assert!(
      addr < MEMORY_SIZE as u16,
      "Memory write out of bounds: {:04X}",
      addr
    );

    self.mem[addr as usize] = val;
  }

  /// Reads a 16-bit big-endian value starting at `addr`.
  ///
  /// CHIP-8 instructions are stored in big-endian format,
  /// making this the primary method used by the CPU fetch stage.
  ///
  /// Panics in debug builds if the read exceeds memory bounds.
  #[inline(always)]
  pub fn read_16_be(&self, addr: u16) -> u16 {
    debug_assert!(
      (addr as usize) + 1 < MEMORY_SIZE,
      "Memory read out of bounds: {:04X}",
      (addr as usize) + 1
    );

    u16::from_be_bytes([self.mem[addr as usize], self.mem[addr as usize + 1]])
  }

  /// Copies a sequence of bytes into memory beginning at `base`.
  ///
  /// This is primarily used to load:
  ///
  /// - The built-in font set
  /// - Program ROMs
  ///
  /// Panics in debug builds if the copied range exceeds memory bounds.
  #[inline]
  pub fn load_bytes(&mut self, base: u16, bytes: &[u8]) {
    debug_assert!(
      (base as usize + bytes.len()) <= MEMORY_SIZE,
      "Memory read out of bounds: {:04X}",
      base as usize + bytes.len()
    );
    self.mem[base as usize..base as usize + bytes.len()].copy_from_slice(bytes);
  }
}