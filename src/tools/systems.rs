#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::ui::input::InputBuffer;
use super::{ToolResult, try_fraction};

pub fn system_solver(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a1 = inputs[0].parse().unwrap_or(0.0);
    let b1 = inputs[1].parse().unwrap_or(0.0);
    let c1 = inputs[2].parse().unwrap_or(0.0);
    let a2 = inputs[3].parse().unwrap_or(0.0);
    let b2 = inputs[4].parse().unwrap_or(0.0);
    let c2 = inputs[5].parse().unwrap_or(0.0);

    result.add("Eq1:", &fmt_eq(a1, b1, c1));
    result.add("Eq2:", &fmt_eq(a2, b2, c2));

    let det = a1 * b2 - a2 * b1;

    if libm::fabs(det) < 1e-12 {
        let cross = c1 * b2 - c2 * b1;
        if libm::fabs(cross) < 1e-12 {
            result.add("Type:", "Dependent (inf sols)");
        } else {
            result.add("Type:", "Inconsistent (no sol)");
        }
        result.finish();
        return;
    }

    let x = (c1 * b2 - c2 * b1) / det;
    let y = (a1 * c2 - a2 * c1) / det;

    let x_str = try_fraction(x, 10000);
    let y_str = try_fraction(y, 10000);

    result.add("x =", &x_str);
    result.add("y =", &y_str);
    let mut sol = "(".to_string();
    sol.push_str(&x_str);
    sol.push_str(", ");
    sol.push_str(&y_str);
    sol.push(')');
    result.add("Sol:", &sol);
    result.finish();
}

fn fmt_eq(a: f64, b: f64, c: f64) -> String {
    let mut s = try_fraction(a, 100);
    s.push_str("x");
    if b >= 0.0 { s.push_str("+"); } else { s.push_str("-"); }
    s.push_str(&try_fraction(libm::fabs(b), 100));
    s.push_str("y=");
    s.push_str(&try_fraction(c, 100));
    s
}

pub fn abs_value_solver(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);

    let eq_str = {
        let mut s = "|".to_string();
        s.push_str(&try_fraction(a, 100));
        s.push_str("x");
        if b >= 0.0 { s.push_str("+"); } else { s.push_str("-"); }
        s.push_str(&try_fraction(libm::fabs(b), 1000));
        s.push_str("| = ");
        s.push_str(&try_fraction(c, 1000));
        s
    };
    result.add("Eq:", &eq_str);

    if c < -1e-12 {
        result.add("Result:", "No solution (c < 0)");
        result.finish();
        return;
    }

    if libm::fabs(a) < 1e-12 {
        result.add("Error:", "a cannot be 0");
        result.finish();
        return;
    }

    let x1 = (c - b) / a;
    let x2 = (-c - b) / a;

    let mut case1 = "ax+b=c: x=".to_string();
    case1.push_str(&try_fraction(x1, 10000));
    result.add("Case +:", &case1);

    let mut case2 = "ax+b=-c: x=".to_string();
    case2.push_str(&try_fraction(x2, 10000));
    result.add("Case -:", &case2);

    if libm::fabs(x1 - x2) < 1e-9 {
        result.add("One sol:", &try_fraction(x1, 10000));
    } else {
        result.add("x1 =", &try_fraction(x1, 10000));
        result.add("x2 =", &try_fraction(x2, 10000));
    }
    result.finish();
}

pub fn inequal_solver(inputs: &[InputBuffer], op: usize, result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);
    let ops = ["<", ">", "<=", ">="];
    let op_str = ops[op.min(3)];

    let eq_str = {
        let mut s = try_fraction(a, 100);
        s.push_str("x + ");
        s.push_str(&try_fraction(b, 1000));
        s.push(' ');
        s.push_str(op_str);
        s.push(' ');
        s.push_str(&try_fraction(c, 1000));
        s
    };
    result.add("Ineq:", &eq_str);

    if libm::fabs(a) < 1e-12 {
        result.add("Error:", "a cannot be 0");
        result.finish();
        return;
    }

    let rhs = (c - b) / a;
    let rhs_str = try_fraction(rhs, 10000);
    let flip = a < 0.0;

    let final_op = if flip {
        match op { 0 => ">", 1 => "<", 2 => ">=", 3 => "<=", _ => op_str }
    } else {
        op_str
    };

    if flip {
        result.set_warn("Sign flipped (div by neg)");
    }

    let mut sol = "x ".to_string();
    sol.push_str(final_op);
    sol.push(' ');
    sol.push_str(&rhs_str);
    result.add("Solution:", &sol);

    // Interval notation
    let interval = if final_op == "<" {
        let mut s = "(-inf, ".to_string(); s.push_str(&rhs_str); s.push(')'); s
    } else if final_op == ">" {
        let mut s = "(".to_string(); s.push_str(&rhs_str); s.push_str(", +inf)"); s
    } else if final_op == "<=" {
        let mut s = "(-inf, ".to_string(); s.push_str(&rhs_str); s.push(']'); s
    } else {
        let mut s = "[".to_string(); s.push_str(&rhs_str); s.push_str(", +inf)"); s
    };
    result.add("Interval:", &interval);
    result.finish();
}
