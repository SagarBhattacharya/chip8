# CHIP-8 Compatibility

Historically, multiple CHIP-8 interpreters introduced small behavioral differences known as *quirks*.

This emulator currently targets a single compatibility profile.

| Quirk              | Behavior |
|--------------------|----------|
| VF Reset           | Disabled |
| Memory Increment   | Disabled |
| Display Wait       | Enabled  |
| Sprite Clipping    | Enabled  |
| Shift Instructions | VX       |
| Jump Instruction   | Uses V0  |

---

## Sprite Clipping

Sprites are clipped at the display boundary.

Pixels outside the display are discarded.

---

## Shift Instructions

The shift instructions operate on VX.

```
8XY6
8XYE
```

VY is ignored.

---

## Memory Instructions

```
FX55
FX65
```

The index register remains unchanged after execution.

---

## Jump

```
BNNN
```

uses

```
PC = NNN + V0
```

---

Future versions may expose configurable compatibility profiles for different CHIP-8 interpreters.