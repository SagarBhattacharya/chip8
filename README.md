# CHIP-8 Emulator

A modular CHIP-8 interpreter written in Rust.

This project explores emulator architecture through a clean separation of hardware components, instruction decoding, and execution. Rather than pursuing maximum performance, it emphasizes readability, maintainability, and explicit design decisions while remaining compatible with common CHIP-8 software.

<p align="center">
  <img src="docs/timendus_welcome.png" alt="Timendus welcome logo on emulator">
  Timendus' Chip8 Logo
</p>

<p align="center">
  <img src="docs/corax_opcodes.png" alt="Corax Opcodes running on the emulator">
  Corax opcode tests
</p>

---

## Highlights

- Modular hardware component design
- Fetch → Decode → Execute execution pipeline
- Semantic instruction representation
- Strongly typed registers and keypad
- Documented architecture and compatibility decisions
- Successfully validated against common CHIP-8 test ROMs

---

## Motivation

This project began as one of my earlier Rust experiments and was later revisited with the goal of turning it into a polished, maintainable codebase.

Rather than simply making the emulator "work", I wanted to explore how to organize a moderately sized systems project with clear module boundaries, strong typing, and explicit control flow.

The result is an emulator that serves both as a functional CHIP-8 interpreter and as a demonstration of software engineering practices in Rust.

---

## Repository History

This repository represents a refined snapshot of the project rather than its original development history.

The original implementation evolved over multiple local experiments before being refactored into its current architecture. During cleanup, the project was reorganized, documented, and simplified before being published.

As a result, the Git history should not be interpreted as a chronological development log of the emulator.

---

## Architecture

```
              Frontend (SDL2)
                    │
        Input ──────┼────── Render
                    │
                CPU Cycle
                    │
      Fetch → Decode → Execute
                    │
    ┌────────┬────────┬────────┬────────┐
    │        │        │        │        │
 Memory  Registers  Display  Keyboard Timers
```

The emulator intentionally separates opcode decoding from execution.

Instead of interpreting raw opcodes directly, every opcode is first decoded into a semantic `Instruction`, allowing the executor to focus solely on implementing CHIP-8 behavior.

---

## Project Structure

```text
src
├── assets
│   └── Built-in CHIP-8 font
│
├── components
│   ├── display.rs
│   ├── keyboard.rs
│   ├── memory.rs
│   └── register.rs
│
├── cpu
│   ├── decoder.rs
│   ├── executor.rs
│   ├── instruction.rs
│   └── mod.rs
│
├── frontend
│   └── sdl.rs
│
└── main.rs
```

---

## Building

```bash
cargo build --release
```

Run a ROM

```bash
cargo run -- roms/2-ibm-logo.ch8
```

---

## Controls

```
Keyboard          CHIP-8

1 2 3 4      ->   1 2 3 C
Q W E R      ->   4 5 6 D
A S D F      ->   7 8 9 E
Z X C V      ->   A 0 B F
```

Press **Esc** to quit.

---

## Compatibility

Successfully tested with

- IBM Logo
- Timendus CHIP-8 Test Suite
- Corax Opcode Test

See [QUIRKS.md](docs/QUIRKS.md) for supported compatibility behavior.

---

## Design Goals

This project intentionally favors:

- Readability
- Small focused modules
- Strong typing
- Minimal abstractions
- Explicit control flow

over premature optimization.

---

## Scope

The objective of this project is to implement a clean and readable CHIP-8 interpreter.

It intentionally prioritizes:

- Readability
- Clear architecture
- Maintainability
- Correctness on common CHIP-8 software

over supporting every historical interpreter variant or extension.

Features such as configurable quirk profiles, Super-CHIP, XO-CHIP, debugging tools, and audio output are intentionally left as future extensions.

---

## Acknowledgements

Thanks to the CHIP-8 community for preserving documentation, test ROMs, and compatibility resources that made this project possible.

Special thanks to the authors of the Timendus and Corax test suites for providing comprehensive validation tools.

---

## License

MIT