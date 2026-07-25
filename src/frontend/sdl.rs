use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use crate::components::{Key, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::cpu::CPU;

const WINDOW_SCALE: u32 = 10;

const WINDOW_WIDTH: u32 = SCREEN_WIDTH as u32 * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT as u32 * WINDOW_SCALE;

const CPU_CYCLES_PER_FRAME: u32 = 3;

pub struct Frontend {
  canvas: WindowCanvas,
  event_pump: EventPump,

  timer_period: Duration,
  last_timer_tick: Instant,

  running: bool,
  frame_dirty: bool,
}

impl Frontend {
  pub fn new(title: &str) -> Result<Self, String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
      .window(title, WINDOW_WIDTH, WINDOW_HEIGHT)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let canvas = window
      .into_canvas()
      .present_vsync()
      .build()
      .map_err(|e| e.to_string())?;

    Ok(Self {
      canvas,
      event_pump: sdl.event_pump()?,
      timer_period: Duration::from_millis(1000 / 60),
      last_timer_tick: Instant::now(),
      running: false,
      frame_dirty: false,
    })
  }

  fn handle_input(&mut self, cpu: &mut CPU) {
    for event in self.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => self.running = false,
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,

        Event::KeyDown { keycode: Some(key), .. } => {
          if let Some(key) = map_keycode(key) {
            cpu.press_key(key);
            self.frame_dirty = true;
          }
        }
        Event::KeyUp { keycode: Some(key), .. } => {
          if let Some(key) = map_keycode(key) {
            cpu.release_key(key);
            self.frame_dirty = true;
          }
        }
        _ => {}
      }
    }
  }

  fn update_timers(&mut self, cpu: &mut CPU) {
    if self.last_timer_tick.elapsed() >= self.timer_period {
      cpu.tick_timers();
      self.last_timer_tick = Instant::now();
    }
  }

  fn render(&mut self, cpu: &CPU, texture: &mut Texture) -> Result<(), String> {
    if self.frame_dirty {
      texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for (y, row) in cpu.framebuffer().iter().enumerate() {
          let row = *row;
          for x in 0..SCREEN_WIDTH {
            let offset = y * pitch + x * 3;
            let pixel_on = (row >> (63 - x)) & 1 != 0;
            let color = if pixel_on { 255 } else { 0 };

            buffer[offset]     = color;
            buffer[offset + 1] = color;
            buffer[offset + 2] = color;
          }
        }
      })?;

      self.frame_dirty = false;
    }

    self.canvas.clear();
    self.canvas.copy(&texture, None, Some(Rect::new(
      0, 0,
      WINDOW_WIDTH,
      WINDOW_HEIGHT,
    )))?;
    self.canvas.present();

    Ok(())
  }

  pub fn run(&mut self, cpu: &mut CPU) -> Result<(), String> {
    self.running = true;
    let texture_creator = self.canvas.texture_creator();
    let mut texture = texture_creator
      .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
      .map_err(|e| e.to_string())?;

    while self.running {
      self.handle_input(cpu);
      for _ in 0..CPU_CYCLES_PER_FRAME {
        cpu.cycle();
        self.frame_dirty = true;
      }
      self.update_timers(cpu);
      self.render(cpu, &mut texture)?;
    }

    Ok(())
  }
}

fn map_keycode(key: Keycode) -> Option<Key> {
  match key {
    Keycode::Num1 => Some(Key::K1),
    Keycode::Num2 => Some(Key::K2),
    Keycode::Num3 => Some(Key::K3),
    Keycode::Num4 => Some(Key::KC),

    Keycode::Q    => Some(Key::K4),
    Keycode::W    => Some(Key::K5),
    Keycode::E    => Some(Key::K6),
    Keycode::R    => Some(Key::KD),

    Keycode::A    => Some(Key::K7),
    Keycode::S    => Some(Key::K8),
    Keycode::D    => Some(Key::K9),
    Keycode::F    => Some(Key::KE),

    Keycode::Z    => Some(Key::KA),
    Keycode::X    => Some(Key::K0),
    Keycode::C    => Some(Key::KB),
    Keycode::V    => Some(Key::KF),
    _ => None,
  }
}
