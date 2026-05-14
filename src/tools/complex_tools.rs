#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::ui::input::InputBuffer;
use super::{ToolResult, fmt_f64, try_fraction};

fn fmt_complex(r: f64, i: f64) -> String {
    let r_s = try_fraction(r, 10000);
    let i_s = try_fraction(libm::fabs(i), 10000);
    let mut s = r_s;
    if i >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
    s.push_str(&i_s);
    s.push('i');
    s
}

pub fn complex_arith(inputs: &[InputBuffer], op: usize, result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);
    let d = inputs[3].parse().unwrap_or(0.0);

    let ops = ["+", "-", "x", "/"];
    let op_str = ops[op.min(3)];

    result.add("z1:", &fmt_complex(a, b));
    result.add("z2:", &fmt_complex(c, d));
    result.add("Op:", op_str);

    let (rr, ri) = match op {
        0 => (a + c, b + d),
        1 => (a - c, b - d),
        2 => (a * c - b * d, a * d + b * c),
        3 => {
            let denom = c * c + d * d;
            if libm::fabs(denom) < 1e-12 {
                result.add("Error:", "div by 0+0i");
                result.finish();
                return;
            }
            result.add("Conj z2:", &fmt_complex(c, -d));
            let mut ds = "|z2|^2 = ".to_string();
            ds.push_str(&fmt_f64(denom));
            result.add("Denom:", &ds);
            ((a * c + b * d) / denom, (b * c - a * d) / denom)
        },
        _ => (0.0, 0.0),
    };

    result.add("Result:", &fmt_complex(rr, ri));
    result.finish();
}

pub fn powers_of_i(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let n_f = inputs[0].parse().unwrap_or(0.0);
    let n = n_f as i64;

    result.add("n =", &fmt_f64(n as f64));
    result.add("n mod 4:", &fmt_f64(n.rem_euclid(4) as f64));

    let simplified = match n.rem_euclid(4) {
        0 => "1",
        1 => "i",
        2 => "-1",
        3 => "-i",
        _ => "?",
    };
    let mut s = "i^".to_string();
    s.push_str(&fmt_f64(n as f64));
    s.push_str(" = ");
    s.push_str(simplified);
    result.add("Result:", &s);
    result.add("Cycle:", "i, -1, -i, 1, ...");
    result.finish();
}

pub fn conj_modulus(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);

    result.add("z =", &fmt_complex(a, b));
    result.add("Conjug:", &fmt_complex(a, -b));

    let mod_sq = a * a + b * b;
    let modulus = libm::sqrt(mod_sq);
    result.add("|z|^2 =", &try_fraction(mod_sq, 10000));
    result.add("|z| =", &fmt_f64(modulus));
    result.add("z*conj:", &try_fraction(mod_sq, 10000));
    result.finish();
}
