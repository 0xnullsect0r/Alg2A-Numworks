#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::string::ToString;
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

pub mod eadk;
pub mod constants;
pub mod ui;
pub mod tools;
pub mod reference;

use eadk::input::{Event, event_get};
use constants::*;
use ui::menu::Menu;
use ui::draw::*;

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 12] = *b"Alg2A Tools\0";

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[cfg(target_os = "none")]
const ICON_NWI: &[u8] = include_bytes!("../target/icon.nwi");
#[cfg(target_os = "none")]
const ICON_SIZE: usize = ICON_NWI.len();

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; ICON_SIZE] = *include_bytes!("../target/icon.nwi");

// ── State Machine ────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    LinearMenu,
    QuadMenu,
    SystemsMenu,
    ComplexMenu,
    RefMenu,
    // Linear tools
    TwoPtsLine,
    FormConvert,
    SlopeCalc,
    InterceptFinder,
    ParPerpSlope,
    // Quadratic tools
    VertexConvert,
    QuadAnalyzer,
    VertexToStd,
    // Systems tools
    SystemSolver,
    AbsValueSolver,
    InequalSolver,
    // Complex tools
    ComplexArith,
    PowersOfI,
    ConjModulus,
    // Reference
    RefFieldAxioms,
    RefFormulas,
    RefProperties,
    RefQuadRef,
}

pub struct AppState {
    pub screen: Screen,
    pub menu_sel: usize,
    pub inputs: [ui::input::InputBuffer; 6],
    pub active_input: usize,
    pub num_inputs: usize,
    pub result: tools::ToolResult,
    pub dirty: bool,
    pub complex_op: usize,
    pub inequal_op: usize,
    pub scroll: usize,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            screen: Screen::MainMenu,
            menu_sel: 0,
            inputs: Default::default(),
            active_input: 0,
            num_inputs: 0,
            result: tools::ToolResult::new(),
            dirty: true,
            complex_op: 0,
            inequal_op: 0,
            scroll: 0,
        }
    }

    pub fn go_to(&mut self, screen: Screen) {
        self.screen = screen;
        self.menu_sel = 0;
        self.active_input = 0;
        self.result.clear();
        self.dirty = true;
        self.scroll = 0;
        for inp in self.inputs.iter_mut() { inp.clear(); }
    }
}

// ── Tool screen info ─────────────────────────────────────────────────────────

fn tool_info(screen: &Screen) -> (&'static str, &'static [&'static str]) {
    match screen {
        Screen::TwoPtsLine      => ("Linear > 2 Pts to Line",
                                    &["x1:", "y1:", "x2:", "y2:"]),
        Screen::FormConvert     => ("Linear > Form Converter",
                                    &["m (slope):", "b (y-int):"]),
        Screen::SlopeCalc       => ("Linear > Slope Calc",
                                    &["x1:", "y1:", "x2:", "y2:"]),
        Screen::InterceptFinder => ("Linear > Intercepts",
                                    &["m (slope):", "b (y-int):"]),
        Screen::ParPerpSlope    => ("Linear > Parallel/Perp",
                                    &["m numer:", "m denom:"]),
        Screen::VertexConvert   => ("Quad > Vertex Form Conv",
                                    &["a:", "b:", "c:"]),
        Screen::QuadAnalyzer    => ("Quad > Quad Analyzer",
                                    &["a:", "b:", "c:"]),
        Screen::VertexToStd     => ("Quad > Vertex to Std",
                                    &["h (vertex x):", "k (vertex y):", "pt x:", "pt y:"]),
        Screen::SystemSolver    => ("Systems > 2x2 Solver",
                                    &["a1:", "b1:", "c1:", "a2:", "b2:", "c2:"]),
        Screen::AbsValueSolver  => ("Systems > Abs Value",
                                    &["a:", "b:", "c:"]),
        Screen::InequalSolver   => ("Systems > Inequality",
                                    &["a:", "b:", "c:"]),
        Screen::ComplexArith    => ("Complex > Arithmetic",
                                    &["a (Re z1):", "b (Im z1):", "c (Re z2):", "d (Im z2):"]),
        Screen::PowersOfI       => ("Complex > Powers of i",
                                    &["n (exponent):"]),
        Screen::ConjModulus     => ("Complex > Conj & Mod",
                                    &["a (Real):", "b (Imag):"]),
        _ => ("", &[]),
    }
}

fn compute(state: &mut AppState) {
    match state.screen {
        Screen::TwoPtsLine      => tools::linear::two_pts_to_line(&state.inputs, &mut state.result),
        Screen::FormConvert     => tools::linear::slope_intercept_to_forms(&state.inputs, &mut state.result),
        Screen::SlopeCalc       => tools::linear::slope_calc(&state.inputs, &mut state.result),
        Screen::InterceptFinder => tools::linear::intercepts_finder(&state.inputs, &mut state.result),
        Screen::ParPerpSlope    => tools::linear::par_perp_slope(&state.inputs, &mut state.result),
        Screen::VertexConvert   => tools::quadratic::vertex_form_convert(&state.inputs, &mut state.result),
        Screen::QuadAnalyzer    => tools::quadratic::quad_analyzer(&state.inputs, &mut state.result),
        Screen::VertexToStd     => tools::quadratic::vertex_to_std(&state.inputs, &mut state.result),
        Screen::SystemSolver    => tools::systems::system_solver(&state.inputs, &mut state.result),
        Screen::AbsValueSolver  => tools::systems::abs_value_solver(&state.inputs, &mut state.result),
        Screen::InequalSolver   => tools::systems::inequal_solver(&state.inputs, state.inequal_op, &mut state.result),
        Screen::ComplexArith    => tools::complex_tools::complex_arith(&state.inputs, state.complex_op, &mut state.result),
        Screen::PowersOfI       => tools::complex_tools::powers_of_i(&state.inputs, &mut state.result),
        Screen::ConjModulus     => tools::complex_tools::conj_modulus(&state.inputs, &mut state.result),
        _ => {}
    }
    state.dirty = true;
}

fn parent_screen(screen: &Screen) -> Screen {
    match screen {
        Screen::TwoPtsLine | Screen::FormConvert | Screen::SlopeCalc |
        Screen::InterceptFinder | Screen::ParPerpSlope => Screen::LinearMenu,

        Screen::VertexConvert | Screen::QuadAnalyzer | Screen::VertexToStd => Screen::QuadMenu,

        Screen::SystemSolver | Screen::AbsValueSolver | Screen::InequalSolver => Screen::SystemsMenu,

        Screen::ComplexArith | Screen::PowersOfI | Screen::ConjModulus => Screen::ComplexMenu,

        Screen::RefFieldAxioms | Screen::RefFormulas | Screen::RefProperties | Screen::RefQuadRef => Screen::RefMenu,

        Screen::LinearMenu | Screen::QuadMenu | Screen::SystemsMenu |
        Screen::ComplexMenu | Screen::RefMenu => Screen::MainMenu,

        Screen::MainMenu => Screen::MainMenu,
    }
}

// ── Rendering ────────────────────────────────────────────────────────────────

fn render_tool_screen(state: &AppState) {
    let (title, labels) = tool_info(&state.screen);
    let num = labels.len();

    clear_screen();
    draw_header(title);

    // Extra info line for special tools
    let extra_y = CONTENT_Y;
    let field_start_y = match state.screen {
        Screen::InequalSolver => {
            let ops = ["< (less)", "> (greater)", "<= (leq)", ">= (geq)"];
            let op_str = ops[state.inequal_op.min(3)];
            let mut info = "Op: ".to_string();
            info.push_str(op_str);
            info.push_str("  (L/R change)");
            draw_line(extra_y, &info, C_DIM, C_BG);
            CONTENT_Y + ROW_H
        }
        Screen::ComplexArith => {
            let ops = ["+ (add)", "- (sub)", "x (mul)", "/ (div)"];
            let op_str = ops[state.complex_op.min(3)];
            let mut info = "Op: ".to_string();
            info.push_str(op_str);
            info.push_str("  (L/R change)");
            draw_line(extra_y, &info, C_DIM, C_BG);
            CONTENT_Y + ROW_H
        }
        _ => CONTENT_Y,
    };

    // Draw input fields
    for i in 0..num {
        let y = field_start_y + i as u16 * ROW_H;
        let val = state.inputs[i].as_str();
        draw_input_field(y, labels[i], val, i == state.active_input);
    }

    // Draw results if ready
    if state.result.ready {
        let result_start = field_start_y + num as u16 * ROW_H + 2;
        draw_sep(result_start);
        let mut ry = result_start + 2;
        for (label, value) in &state.result.lines {
            if ry + ROW_H > FOOTER_Y { break; }
            draw_result_line(ry, label, value);
            ry += ROW_H;
        }
        if let Some(warn) = &state.result.warn {
            if ry + ROW_H <= FOOTER_Y {
                draw_warn_line(ry, warn);
            }
        }
    }

    draw_footer("Up/Dn=Field  EXE=Calc  -=Sign  .=Dot  Back=Back");
}

fn render_screen(state: &AppState) {
    match &state.screen {
        Screen::MainMenu => {
            let items = &["Linear Functions", "Quadratic Fns", "Systems & Eq",
                          "Complex Nums", "Reference"];
            let mut m = Menu::new("Alg2A Tools", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::LinearMenu => {
            let items = &["2 Pts to Equation", "Form Converter", "Slope Calc",
                          "Intercepts", "Parallel/Perp"];
            let mut m = Menu::new("Linear Functions", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::QuadMenu => {
            let items = &["Vertex Form Conv", "Quad Analyzer", "Vertex to Std Form"];
            let mut m = Menu::new("Quadratic Functions", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::SystemsMenu => {
            let items = &["2x2 System Solver", "Abs Value Solver", "Inequality Solver"];
            let mut m = Menu::new("Systems & Equations", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::ComplexMenu => {
            let items = &["Complex Arithmetic", "Powers of i", "Conjugate & Modulus"];
            let mut m = Menu::new("Complex Numbers", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::RefMenu => {
            let items = &["Field Axioms", "Key Formulas", "Alg Properties", "Quad Quick Ref"];
            let mut m = Menu::new("Reference Cards", items);
            m.selected = state.menu_sel;
            m.draw();
        }
        Screen::RefFieldAxioms => {
            reference::draw_ref_card("Reference: Field Axioms",
                reference::cards::FIELD_AXIOMS, state.scroll);
        }
        Screen::RefFormulas => {
            reference::draw_ref_card("Reference: Key Formulas",
                reference::cards::KEY_FORMULAS, state.scroll);
        }
        Screen::RefProperties => {
            reference::draw_ref_card("Reference: Alg Properties",
                reference::cards::ALG_PROPERTIES, state.scroll);
        }
        Screen::RefQuadRef => {
            reference::draw_ref_card("Reference: Quad Quick Ref",
                reference::cards::QUAD_QUICK_REF, state.scroll);
        }
        _ => render_tool_screen(state),
    }
}

// ── Event Handling ───────────────────────────────────────────────────────────

fn handle_menu_event(state: &mut AppState, ev: Event) {
    let max_items = match state.screen {
        Screen::MainMenu    => 5,
        Screen::LinearMenu  => 5,
        Screen::QuadMenu    => 3,
        Screen::SystemsMenu => 3,
        Screen::ComplexMenu => 3,
        Screen::RefMenu     => 4,
        _ => 0,
    };

    match ev {
        Event::Up => {
            if state.menu_sel > 0 { state.menu_sel -= 1; }
            state.dirty = true;
        }
        Event::Down => {
            if state.menu_sel + 1 < max_items { state.menu_sel += 1; }
            state.dirty = true;
        }
        Event::Ok | Event::Exe => {
            let new_screen = match state.screen {
                Screen::MainMenu => match state.menu_sel {
                    0 => Screen::LinearMenu,
                    1 => Screen::QuadMenu,
                    2 => Screen::SystemsMenu,
                    3 => Screen::ComplexMenu,
                    4 => Screen::RefMenu,
                    _ => return,
                },
                Screen::LinearMenu => match state.menu_sel {
                    0 => Screen::TwoPtsLine,
                    1 => Screen::FormConvert,
                    2 => Screen::SlopeCalc,
                    3 => Screen::InterceptFinder,
                    4 => Screen::ParPerpSlope,
                    _ => return,
                },
                Screen::QuadMenu => match state.menu_sel {
                    0 => Screen::VertexConvert,
                    1 => Screen::QuadAnalyzer,
                    2 => Screen::VertexToStd,
                    _ => return,
                },
                Screen::SystemsMenu => match state.menu_sel {
                    0 => Screen::SystemSolver,
                    1 => Screen::AbsValueSolver,
                    2 => Screen::InequalSolver,
                    _ => return,
                },
                Screen::ComplexMenu => match state.menu_sel {
                    0 => Screen::ComplexArith,
                    1 => Screen::PowersOfI,
                    2 => Screen::ConjModulus,
                    _ => return,
                },
                Screen::RefMenu => match state.menu_sel {
                    0 => Screen::RefFieldAxioms,
                    1 => Screen::RefFormulas,
                    2 => Screen::RefProperties,
                    3 => Screen::RefQuadRef,
                    _ => return,
                },
                _ => return,
            };
            let sel = state.menu_sel;
            state.go_to(new_screen);
            // Restore menu selection for sub-menus won't matter since go_to resets it
            let _ = sel;
        }
        Event::Back => {
            let parent = parent_screen(&state.screen);
            state.go_to(parent);
        }
        _ => {}
    }
}

fn handle_ref_event(state: &mut AppState, ev: Event) {
    let lines_len = match state.screen {
        Screen::RefFieldAxioms => reference::cards::FIELD_AXIOMS.len(),
        Screen::RefFormulas    => reference::cards::KEY_FORMULAS.len(),
        Screen::RefProperties  => reference::cards::ALG_PROPERTIES.len(),
        Screen::RefQuadRef     => reference::cards::QUAD_QUICK_REF.len(),
        _ => 0,
    };
    let max_scroll = lines_len.saturating_sub((CONTENT_H / ROW_H) as usize);

    match ev {
        Event::Up => {
            if state.scroll > 0 { state.scroll -= 1; state.dirty = true; }
        }
        Event::Down => {
            if state.scroll < max_scroll { state.scroll += 1; state.dirty = true; }
        }
        Event::Back => {
            let parent = parent_screen(&state.screen);
            state.go_to(parent);
        }
        _ => {}
    }
}

fn handle_tool_event(state: &mut AppState, ev: Event) {
    let (_, labels) = tool_info(&state.screen);
    let num_inputs = labels.len();

    match ev {
        Event::Up => {
            if state.active_input > 0 { state.active_input -= 1; }
            else { state.active_input = num_inputs.saturating_sub(1); }
            state.dirty = true;
        }
        Event::Down => {
            if state.active_input + 1 < num_inputs { state.active_input += 1; }
            else { state.active_input = 0; }
            state.dirty = true;
        }
        Event::Left => {
            match state.screen {
                Screen::InequalSolver => {
                    state.inequal_op = (state.inequal_op + 3) % 4;
                    state.result.clear();
                    state.dirty = true;
                }
                Screen::ComplexArith => {
                    state.complex_op = (state.complex_op + 3) % 4;
                    state.result.clear();
                    state.dirty = true;
                }
                _ => {}
            }
        }
        Event::Right => {
            match state.screen {
                Screen::InequalSolver => {
                    state.inequal_op = (state.inequal_op + 1) % 4;
                    state.result.clear();
                    state.dirty = true;
                }
                Screen::ComplexArith => {
                    state.complex_op = (state.complex_op + 1) % 4;
                    state.result.clear();
                    state.dirty = true;
                }
                _ => {}
            }
        }
        Event::Ok | Event::Exe => {
            compute(state);
        }
        Event::Back => {
            let parent = parent_screen(&state.screen);
            state.go_to(parent);
        }
        Event::Backspace => {
            if state.active_input < num_inputs {
                state.inputs[state.active_input].backspace();
                state.result.clear();
                state.dirty = true;
            }
        }
        Event::Dot => {
            if state.active_input < num_inputs {
                state.inputs[state.active_input].push_dot();
                state.result.clear();
                state.dirty = true;
            }
        }
        Event::Minus => {
            if state.active_input < num_inputs {
                state.inputs[state.active_input].toggle_sign();
                state.result.clear();
                state.dirty = true;
            }
        }
        _ => {
            if let Some(d) = ev.to_digit() {
                if state.active_input < num_inputs {
                    state.inputs[state.active_input].push_digit(d);
                    state.result.clear();
                    state.dirty = true;
                }
            }
        }
    }
}

fn is_menu_screen(screen: &Screen) -> bool {
    matches!(screen, Screen::MainMenu | Screen::LinearMenu | Screen::QuadMenu |
             Screen::SystemsMenu | Screen::ComplexMenu | Screen::RefMenu)
}

fn is_ref_screen(screen: &Screen) -> bool {
    matches!(screen, Screen::RefFieldAxioms | Screen::RefFormulas |
             Screen::RefProperties | Screen::RefQuadRef)
}

// ── Entry Point ──────────────────────────────────────────────────────────────

#[unsafe(no_mangle)]
pub fn main() -> u32 {
    #[cfg(target_os = "none")]
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 32 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE); }
    }

    let mut state = AppState::new();

    loop {
        if state.dirty {
            render_screen(&state);
            state.dirty = false;
        }

        let ev = event_get(-1);

        if is_menu_screen(&state.screen) {
            handle_menu_event(&mut state, ev);
        } else if is_ref_screen(&state.screen) {
            handle_ref_event(&mut state, ev);
        } else {
            handle_tool_event(&mut state, ev);
        }
    }
}
