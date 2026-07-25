use log::debug;
use rand::Rng;
use crate::components::{Key, Register, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::cpu::{CPU, FONT_START};
use crate::cpu::instruction::Instruction;

/// Executes a decoded CHIP-8 instruction.
///
/// The executor mutates the CPU state according to the semantics of the
/// provided instruction. It assumes the instruction has already been
/// validated by the decoder.
pub(super) fn execute(cpu: &mut CPU, ins: Instruction) {
  match ins {
    Instruction::ClearScreen => cpu.display.clear(),

    Instruction::Return => cpu.program_counter = cpu.stack.pop().unwrap(),

    Instruction::Jump { addr } => cpu.program_counter = addr,

    Instruction::LoadImm { reg, value } => cpu.registers.set(reg, value),

    Instruction::Move { dst, src } => cpu.registers.set(dst, cpu.registers.get(src)),

    Instruction::LoadIndex { addr } => cpu.index = addr,

    Instruction::JumpV0 { addr } => cpu.program_counter = addr + cpu.registers.get(Register::V0) as u16,

    Instruction::Call { addr } => {
      cpu.stack.push(cpu.program_counter);
      cpu.program_counter = addr;
    }

    Instruction::SkipEqImm { reg, value } => {
      if cpu.registers.get(reg) == value {
        cpu.program_counter += 2;
      }
    }

    Instruction::SkipNeImm { reg, value } => {
      if cpu.registers.get(reg) != value {
        cpu.program_counter += 2;
      }
    }

    Instruction::SkipEqReg { lhs, rhs } => {
      if cpu.registers.get(lhs) == cpu.registers.get(rhs) {
        cpu.program_counter += 2;
      }
    }

    Instruction::AddImm { reg, value } => {
      let old_value = cpu.registers.get(reg);
      cpu.registers.set(reg, old_value.wrapping_add(value));
    }

    Instruction::Or { dst, src } => {
      let value = cpu.registers.get(dst) | cpu.registers.get(src);
      cpu.registers.set(dst, value);
    }

    Instruction::And { dst, src } => {
      let value = cpu.registers.get(dst) & cpu.registers.get(src);
      cpu.registers.set(dst, value);
    }

    Instruction::Xor { dst, src } => {
      let value = cpu.registers.get(dst) ^ cpu.registers.get(src);
      cpu.registers.set(dst, value);
    }

    Instruction::Add { dst, src } => {
      let (result, ov) = cpu.registers.get(dst)
        .overflowing_add(cpu.registers.get(src));

      cpu.registers.set_flag(ov);
      cpu.registers.set(dst, result);
    }

    Instruction::Sub { dst, src } => {
      let (result, ov) = cpu.registers.get(dst)
        .overflowing_sub(cpu.registers.get(src));

      cpu.registers.set_flag(!ov);
      cpu.registers.set(dst, result);
    }

    Instruction::ShiftRight { reg } => {
      let value = cpu.registers.get(reg);
      cpu.registers.set_flag((value & 0x01) == 1);
      cpu.registers.set(reg, value >> 1);
    }

    Instruction::SubN { dst, src } => {
      let (result, ov) = cpu.registers.get(src)
        .overflowing_sub(cpu.registers.get(dst));

      cpu.registers.set_flag(!ov);
      cpu.registers.set(dst, result);
    }

    Instruction::ShiftLeft { reg } => {
      let value = cpu.registers.get(reg);
      cpu.registers.set_flag(((value & 0x80) >> 7) == 1);
      cpu.registers.set(reg, value << 1);
    }

    Instruction::SkipNeReg { lhs, rhs } => {
      if cpu.registers.get(lhs) != cpu.registers.get(rhs) {
        cpu.program_counter += 2;
      }
    }

    Instruction::Random { reg, mask } => {
      let rand = cpu.rand_engine.random::<u8>() & mask;
      cpu.registers.set(reg, rand);
    }

    Instruction::Draw { x, y, height } => {
      let x = cpu.registers.get(x) % SCREEN_WIDTH as u8;
      let y = cpu.registers.get(y) % SCREEN_HEIGHT as u8;

      let mut has_collision = false;
      for row in 0..height {
        let py = y + row;
        if py >= SCREEN_HEIGHT as u8 {
          break;
        }
        let sprite = cpu.memory.read_byte(cpu.index + row as u16);
        has_collision |= cpu.display.draw_sprite_row(x, py, sprite);
      }
      cpu.registers.set_flag(has_collision);
    }

    Instruction::SkipKeyPressed { reg } => {
      let key = Key::from_nibble(cpu.registers.get(reg)).unwrap();
      if cpu.keypad.is_pressed(key) {
        cpu.program_counter += 2;
      }
    }

    Instruction::SkipKeyNotPressed { reg } => {
      let key = Key::from_nibble(cpu.registers.get(reg)).unwrap();
      if !cpu.keypad.is_pressed(key) {
        cpu.program_counter += 2;
      }
    }

    Instruction::ReadDelay { reg } => cpu.registers.set(reg, cpu.delay_timer),

    Instruction::WriteDelay { reg } => cpu.delay_timer = cpu.registers.get(reg),

    Instruction::WriteSound { reg } => cpu.sound_timer = cpu.registers.get(reg),

    Instruction::WaitKey { reg } => {
      if let Some(key) = cpu.keypad.any_pressed() {
        cpu.registers.set(reg, key.index() as u8);
      } else {
        cpu.program_counter -= 2;
      }
    }

    Instruction::AddIndex { reg } => cpu.index += cpu.registers.get(reg) as u16,

    Instruction::LoadFont { reg } => cpu.index = (FONT_START + cpu.registers.get(reg) as usize * 5) as u16,

    Instruction::StoreBCD { reg } => {
      let mut value = cpu.registers.get(reg);

      cpu.memory.write_byte(cpu.index + 2, value % 10);
      value /= 10;
      cpu.memory.write_byte(cpu.index + 1, value % 10);
      value /= 10;
      cpu.memory.write_byte(cpu.index, value % 10);
    }

    Instruction::StoreRegisters { last } => {
      for i in 0..=last.index() {
        let value = cpu.registers.get(Register::from_nibble(i as u8).unwrap());
        cpu.memory.write_byte(cpu.index + i as u16, value);
      }
    }

    Instruction::LoadRegisters { last } => {
      for i in 0..=last.index() {
        let value = cpu.memory.read_byte(cpu.index + i as u16);
        cpu.registers.set(Register::from_nibble(i as u8).unwrap(), value);
      }
    }

    Instruction::Unknown(code) => {
      debug!("Unknown opcode: 0x{:04X}", code);
    }
  }
}