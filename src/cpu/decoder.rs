use crate::components::Register;
use crate::cpu::instruction::Instruction;

/// Decodes a raw 16-bit CHIP-8 opcode into its semantic representation.
///
/// This function performs no execution and has no side effects.
/// Unknown or unsupported opcodes are represented as
/// `Instruction::Unknown`.
pub(super) fn decode(opcode: u16) -> Instruction {
  let n = (opcode & 0x000F) as u8;
  let nn = (opcode & 0x00FF) as u8;
  let nnn = opcode & 0x0FFF;
  let x = Register::from_nibble(((opcode >> 8) & 0x0F) as u8).unwrap();
  let y = Register::from_nibble(((opcode >> 4) & 0x0F) as u8).unwrap();

  match opcode {
    0x00E0 => Instruction::ClearScreen,
    0x00EE => Instruction::Return,

    _ => match opcode & 0xF000 {
      0x1000 => Instruction::Jump { addr: nnn },
      0x2000 => Instruction::Call { addr: nnn },
      0x3000 => Instruction::SkipEqImm { reg: x, value: nn },
      0x4000 => Instruction::SkipNeImm { reg: x, value: nn },
      0x5000 if n == 0x0 => Instruction::SkipEqReg { lhs: x, rhs: y },
      0x6000 => Instruction::LoadImm { reg: x, value: nn },
      0x7000 => Instruction::AddImm { reg: x, value: nn },
      0x8000 => match n {
        0x0 => Instruction::Move { dst: x, src: y },
        0x1 => Instruction::Or { dst: x, src: y },
        0x2 => Instruction::And { dst: x, src: y },
        0x3 => Instruction::Xor { dst: x, src: y },
        0x4 => Instruction::Add { dst: x, src: y },
        0x5 => Instruction::Sub { dst: x, src: y },
        0x6 => Instruction::ShiftRight { reg: x },
        0x7 => Instruction::SubN { dst: x, src: y },
        0xE => Instruction::ShiftLeft { reg: x },
        _ => Instruction::Unknown(opcode)
      },
      0x9000 if n == 0x0 => Instruction::SkipNeReg { lhs: x, rhs: y },
      0xA000 => Instruction::LoadIndex { addr: nnn },
      0xB000 => Instruction::JumpV0 { addr: nnn },
      0xC000 => Instruction::Random { reg: x, mask: nn },
      0xD000 => Instruction::Draw { x, y, height: n },
      0xE000 => match nn {
        0x9E => Instruction::SkipKeyPressed { reg: x },
        0xA1 => Instruction::SkipKeyNotPressed { reg: x },
        _ => Instruction::Unknown(opcode)
      },
      0xF000 => match nn {
        0x07 => Instruction::ReadDelay { reg: x },
        0x0A => Instruction::WaitKey { reg: x },
        0x15 => Instruction::WriteDelay { reg: x },
        0x18 => Instruction::WriteSound { reg: x },
        0x1E => Instruction::AddIndex { reg: x },
        0x29 => Instruction::LoadFont { reg: x },
        0x33 => Instruction::StoreBCD { reg: x },
        0x55 => Instruction::StoreRegisters { last: x },
        0x65 => Instruction::LoadRegisters { last: x },
        _ => Instruction::Unknown(opcode)
      },
      _ => Instruction::Unknown(opcode)
    }
  }
}