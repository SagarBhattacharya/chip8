# Architecture

The emulator is organized into three independent layers.

```
Frontend
    │
   CPU
    │
Hardware Components
```

---

## Hardware Components

The emulator models each hardware resource independently.

```
Memory
Display
Keyboard
Register File
```

Each component is responsible only for its own behavior.

For example, the display knows how to draw pixels and sprites but knows nothing about instruction execution.

---

## CPU

The CPU owns every hardware component and coordinates instruction execution.

```
Fetch
↓
Decode
↓
Execute
```

### Fetch

Reads the next 16-bit opcode from memory.

### Decode

Transforms the raw opcode into a semantic `Instruction`.

Example

```
0x6A42

↓

Instruction::LoadImm {
    reg: Register::VA,
    value: 42,
}
```

The decoder performs all opcode parsing.

---

### Execute

Applies the semantic instruction to the virtual machine.

The executor never performs bit manipulation on raw opcodes.

Instead it operates exclusively on decoded instructions.

---

## Frontend

The frontend is responsible for

- Window creation
- Keyboard input
- Display rendering
- Timer scheduling

It contains no CHIP-8 emulation logic.

---

## Design Decisions

### Semantic Instructions

Instead of decoding and executing simultaneously,

```
opcode

↓

bit manipulation

↓

behavior
```

the emulator separates decoding from execution.

```
opcode

↓

Instruction

↓

behavior
```

This significantly improves readability and simplifies testing.

---

### Strong Typing

Registers and keypad values are represented using enums instead of raw integers.

```
Register::VA

Key::KF
```

This prevents accidental misuse while making the code self-documenting.

---

### Minimal Abstraction

Abstractions are introduced only when they improve readability.

The project intentionally avoids unnecessary traits or inheritance hierarchies.