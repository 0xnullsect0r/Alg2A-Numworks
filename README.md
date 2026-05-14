# Alg2A Tools — Numworks Calculator App

A native app for the **Numworks N110/N120** calculator packed with algebraic tools and cheat-sheet reference cards for an Algebra 2A course. Built in Rust using the [`here-template/Numworks-App`](https://github.com/here-template/Numworks-App) template.

> **What it is:** Quick-access calculators for form conversions, equation derivations, quadratic analysis, system solving, and complex number arithmetic — things the built-in Numworks OS *doesn't* do for you.

---

## Tools

### 📐 Linear Functions
| Tool | What it does |
|------|-------------|
| **2 Points → Equation** | Given (x₁,y₁) and (x₂,y₂), outputs slope, slope-intercept form, point-slope form, and standard form |
| **Form Converter** | Given slope `m` and y-intercept `b`, converts between all three linear forms |
| **Slope Calculator** | Computes rise, run, slope fraction, decimal, and classifies as positive/negative/zero |
| **Intercepts Finder** | Finds x-intercept and y-intercept from `m` and `b` |
| **Parallel/Perpendicular** | Given a slope, outputs the parallel slope and the perpendicular (negative reciprocal) slope |

### 📊 Quadratic Functions
| Tool | What it does |
|------|-------------|
| **Vertex Form Converter** | Converts `ax²+bx+c` → `a(x-h)²+k` showing each step of completing the square |
| **Quadratic Analyzer** | Full analysis: discriminant, root count/type, roots (real or complex), vertex, axis of symmetry, opens up/down |
| **Vertex → Standard Form** | Given vertex (h,k) and one point on the parabola, solves for `a` and outputs both vertex and standard form |

### ⚖️ Systems & Equations
| Tool | What it does |
|------|-------------|
| **2×2 System Solver** | Solves `a₁x+b₁y=c₁` and `a₂x+b₂y=c₂` by Cramer's rule; detects dependent/inconsistent systems |
| **Absolute Value Solver** | Solves `|ax+b|=c` by splitting into the two cases and labeling each |
| **Inequality Solver** | Solves `ax+b OP c`; automatically flips the inequality sign when dividing by a negative, shows interval notation |

### 🔢 Complex Numbers
| Tool | What it does |
|------|-------------|
| **Complex Arithmetic** | Add, subtract, multiply, or divide two complex numbers; division shows the conjugate multiplication steps |
| **Powers of i** | Computes `iⁿ` using the mod-4 cycle; shows `n mod 4` and result |
| **Conjugate & Modulus** | Given `a+bi`, outputs conjugate `a-bi`, `\|z\|²`, and `\|z\|` |

### 📋 Reference Cards (scrollable)
- **Field Axioms** — Closure, Commutativity, Associativity, Identity, Inverses, Distributivity
- **Key Formulas** — Slope, all linear forms, quadratic formula, discriminant rules, vertex formulas, completing the square, imaginary unit cycle
- **Algebraic Properties** — Equality properties, order properties (including sign-flip rule), zero/one props, fraction props
- **Quadratic Quick Ref** — Standard/vertex forms, completing the square steps, quadratic formula, complex root form, conjugate multiply trick, complex division

---

## Controls

| Key | Action |
|-----|--------|
| **Up / Down** | Navigate menu items; move between input fields in tools |
| **EXE / OK** | Select menu item; run calculation |
| **Back** | Return to previous menu |
| **Left / Right** | Cycle operation (Complex Arithmetic: +/−/×/÷; Inequality: </>/≤/≥) |
| **0–9** | Enter digits into the active input field |
| **− (Minus)** | Toggle sign of the active input (enter negative numbers) |
| **. (Dot)** | Enter decimal point |
| **Backspace** | Delete last character in active field |

---

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| [Rust (nightly)](https://rustup.rs/) | nightly | Compiler — `no_std` features required |
| `thumbv7em-none-eabihf` target | — | ARM cross-compilation target |
| [just](https://github.com/casey/just) | any | Task runner (like `make`) |
| [Node.js](https://nodejs.org/) + `bun` | any | Runs `nwlink` (icon conversion + USB upload) |
| Numworks calculator | N110 or N120 | Hardware target (optional — can build without it) |

> **Note:** `arm-none-eabi-gcc` is **not** required. Storage bindings have been removed since this app does not use flash storage.

---

## Installation

### 1 — Install Rust nightly

```bash
# Install rustup if you don't have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The rust-toolchain.toml in this repo sets the channel to nightly automatically.
# To add it manually:
rustup toolchain install nightly
```

### 2 — Add the ARM cross-compilation target

```bash
rustup target add thumbv7em-none-eabihf
```

### 3 — Install `just`

```bash
# Arch / CachyOS
sudo pacman -S just

# macOS
brew install just

# Any platform via cargo
cargo install just
```

### 4 — Install Bun (runs nwlink for icon conversion and USB upload)

```bash
curl -fsSL https://bun.sh/install | bash
```

> `nwlink` itself is fetched automatically by `bunx` — no manual install needed.

### 5 — Clone and enter the repo

```bash
git clone https://github.com/0xnullsect0r/Alg2A-Numworks.git
cd Alg2A-Numworks
```

---

## Building & Installing

### Build for the calculator

```bash
just build
```

Compiles a release binary at `target/thumbv7em-none-eabihf/release/alg2a-tools`.

### Upload to your Numworks via USB

Connect your calculator, press **Home → Settings → About** and enable developer mode if prompted, then:

```bash
just send
```

### Export as a `.nwa` file

```bash
just export-nwa
```

Produces `alg2a-tools.nwa` in the project root. You can transfer this file manually via the [Numworks online workshop](https://my.numworks.com/apps).

### Verify without the calculator

```bash
cargo check --target thumbv7em-none-eabihf
```

### Run in the Epsilon simulator (Linux / macOS)

```bash
# First run clones the Epsilon v20 simulator and builds it (takes several minutes)
just sim

# Speed up the simulator build with more parallel jobs
just sim 8
```

> **Epsilon version 20 only.** Version 21 broke the external app API; the simulator target is pinned to `version-20`.

---

## Project Structure

```
Alg2A-Numworks/
├── src/
│   ├── main.rs              # App entry point, state machine, event loop
│   ├── constants.rs         # Screen dimensions, colors, layout constants
│   ├── eadk.rs              # EADK hardware API bindings (display, input, timing)
│   ├── storage_lib.rs       # Flash storage FFI stubs (unused by this app)
│   ├── ui/
│   │   ├── menu.rs          # Scrollable menu widget
│   │   ├── input.rs         # Numeric input buffer (digits, sign toggle, decimal)
│   │   └── draw.rs          # Header, footer, field, result, separator helpers
│   ├── tools/
│   │   ├── mod.rs           # ToolResult type, shared math helpers (fmt, fractions, GCD)
│   │   ├── linear.rs        # 5 linear function tools
│   │   ├── quadratic.rs     # 3 quadratic tools
│   │   ├── systems.rs       # 3 systems/equation tools
│   │   └── complex_tools.rs # 3 complex number tools
│   └── reference/
│       ├── mod.rs           # draw_ref_card renderer
│       └── cards.rs         # Static text for all 4 reference cards
├── assets/
│   └── icon.png             # 55×56 app icon (required dimensions for nwlink)
├── src/libs/
│   ├── storage.c            # Numworks storage C implementation (not compiled)
│   └── storage.h
├── .cargo/
│   └── config.toml          # ARM target runner + linker flags
├── build.rs                 # Icon PNG→NWI conversion via nwlink
├── Cargo.toml
├── rust-toolchain.toml      # Pins Rust to nightly
├── justfile                 # Build recipes
└── package.json             # nwlink version pin
```

---

## How it Works

The app is a `no_std` Rust binary compiled for `thumbv7em-none-eabihf` (ARM Cortex-M7 hard-float). Key details:

- **Display:** 320×240 px. Text is drawn with `eadk::display::draw_string`; rectangles filled with `push_rect_uniform`.
- **Input:** `eadk::input::event_get(-1)` blocks until a key is pressed. No polling.
- **Memory:** A 32 KB heap is initialized at startup using `embedded-alloc`, enabling `String`, `Vec`, and `format!` via the `alloc` crate.
- **Math:** `libm` provides `sqrt`, `fabs`, `round`, `floor` etc. in `no_std` context.
- **Fractions:** Results are displayed as exact fractions (e.g. `2/3`) when possible, falling back to 4-decimal floats.
- **Icon:** `build.rs` runs `nwlink png-nwi` to convert `assets/icon.png` to the `.nwi` format embedded in the binary.

---

## Troubleshooting

**`unable to find library -lstorage_c`**
Stale build cache. Run `cargo clean` then `just build`.

**`bunx: command not found`**
Install Bun: `curl -fsSL https://bun.sh/install | bash`, then restart your shell.

**`error: no such target 'thumbv7em-none-eabihf'`**
```bash
rustup target add thumbv7em-none-eabihf
```

**Calculator not detected by `just send`**
Make sure the USB cable supports data transfer (not charge-only). On Linux, you may need a udev rule:
```bash
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0483", MODE="0666"' | sudo tee /etc/udev/rules.d/99-numworks.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
```

**Simulator build fails**
The simulator requires the full Numworks Epsilon build toolchain (Python, Node, make, etc.). See the [Numworks developer docs](https://www.numworks.com/engineering/software/build/) for details. The calculator build (`just build`) has no such dependency.
