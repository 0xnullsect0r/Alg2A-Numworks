#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::ui::input::InputBuffer;
use super::{ToolResult, fmt_f64, try_fraction, gcd, fmt_fraction};

pub fn two_pts_to_line(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let x1 = inputs[0].parse().unwrap_or(0.0);
    let y1 = inputs[1].parse().unwrap_or(0.0);
    let x2 = inputs[2].parse().unwrap_or(0.0);
    let y2 = inputs[3].parse().unwrap_or(0.0);

    if libm::fabs(x2 - x1) < 1e-12 {
        result.add("Form:", "Vertical line");
        let mut s = "x = ".to_string();
        s.push_str(&fmt_f64(x1));
        result.add("Eq:", &s);
        result.add("Slope:", "undefined");
        result.finish();
        return;
    }

    let rise = y2 - y1;
    let run = x2 - x1;
    let slope = rise / run;
    let slope_str = try_fraction(slope, 100);
    let b = y1 - slope * x1;
    let b_str = try_fraction(b, 1000);

    let si = {
        let mut s = "y = ".to_string();
        s.push_str(&slope_str);
        s.push_str("x");
        if b >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(b), 1000));
        s
    };

    let ps = {
        let mut s = "y".to_string();
        if y1 >= 0.0 { s.push_str(" - "); s.push_str(&fmt_f64(y1)); }
        else { s.push_str(" + "); s.push_str(&fmt_f64(libm::fabs(y1))); }
        s.push_str("=");
        s.push_str(&slope_str);
        s.push_str("(x");
        if x1 >= 0.0 { s.push_str("-"); s.push_str(&fmt_f64(x1)); }
        else { s.push_str("+"); s.push_str(&fmt_f64(libm::fabs(x1))); }
        s.push_str(")");
        s
    };

    let (a_std, b_std, c_std) = slope_intercept_to_standard(slope, b);
    let std_form = {
        let mut s = String::new();
        s.push_str(&fmt_fraction(a_std, 1));
        s.push_str("x + ");
        s.push_str(&fmt_fraction(b_std, 1));
        s.push_str("y = ");
        s.push_str(&fmt_fraction(c_std, 1));
        s
    };

    result.add("Slope m:", &slope_str);
    result.add("y=mx+b:", &si);
    result.add("Pt-Slope:", &ps);
    result.add("Std form:", &std_form);
    result.add("y-intcpt:", &b_str);
    result.finish();
}

fn slope_intercept_to_standard(m: f64, b: f64) -> (i64, i64, i64) {
    let mut best_denom = 1i64;
    for d in 1i64..=100 {
        let n = libm::round(libm::fabs(m) * d as f64) as i64;
        if libm::fabs(n as f64 / d as f64 - libm::fabs(m)) < 1e-9 {
            best_denom = d;
            break;
        }
    }
    let a_raw = -(libm::round(m * best_denom as f64) as i64);
    let b_coef = best_denom;
    let c_raw = libm::round(b * best_denom as f64) as i64;

    let (a_final, b_final, c_final) = if a_raw < 0 {
        (-a_raw, -b_coef, -c_raw)
    } else {
        (a_raw, b_coef, c_raw)
    };

    let g = gcd(gcd(a_final.abs(), b_final.abs()), c_final.abs()).max(1);
    (a_final / g, b_final / g, c_final / g)
}

pub fn slope_intercept_to_forms(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let m = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);

    let m_str = try_fraction(m, 100);
    let b_str = try_fraction(b, 1000);

    let si = {
        let mut s = "y = ".to_string();
        s.push_str(&m_str);
        s.push_str("x");
        if b >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(b), 1000));
        s
    };

    let (a_std, b_std, c_std) = slope_intercept_to_standard(m, b);
    let std_form = {
        let mut s = String::new();
        s.push_str(&fmt_fraction(a_std, 1));
        s.push_str("x+");
        s.push_str(&fmt_fraction(b_std, 1));
        s.push_str("y=");
        s.push_str(&fmt_fraction(c_std, 1));
        s
    };

    let ps = {
        let mut s = "y".to_string();
        if b >= 0.0 { s.push_str(" - "); s.push_str(&b_str); }
        else { s.push_str(" + "); s.push_str(&try_fraction(libm::fabs(b), 1000)); }
        s.push_str(" = ");
        s.push_str(&m_str);
        s.push_str("(x - 0)");
        s
    };

    result.add("Slp-Int:", &si);
    result.add("Std form:", &std_form);
    result.add("Pt-Slope:", &ps);
    result.add("m =", &m_str);
    result.add("b =", &b_str);
    result.finish();
}

pub fn slope_calc(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let x1 = inputs[0].parse().unwrap_or(0.0);
    let y1 = inputs[1].parse().unwrap_or(0.0);
    let x2 = inputs[2].parse().unwrap_or(0.0);
    let y2 = inputs[3].parse().unwrap_or(0.0);

    let rise = y2 - y1;
    let run = x2 - x1;

    if libm::fabs(run) < 1e-12 {
        result.add("Slope:", "undefined");
        result.add("Type:", "Vertical line");
        result.finish();
        return;
    }

    let slope = rise / run;
    let slope_frac = try_fraction(slope, 100);
    let slope_dec = fmt_f64(slope);
    let rise_str = fmt_f64(rise);
    let run_str = fmt_f64(run);

    result.add("Rise:", &rise_str);
    result.add("Run:", &run_str);
    let mut frac_disp = rise_str.clone();
    frac_disp.push('/');
    frac_disp.push_str(&run_str);
    result.add("Rise/Run:", &frac_disp);
    result.add("Slope m:", &slope_frac);
    if slope_frac != slope_dec { result.add("Decimal:", &slope_dec); }

    let slope_type = if slope > 0.0 { "Positive" }
        else if slope < 0.0 { "Negative" }
        else { "Zero (horiz)" };
    result.add("Type:", slope_type);
    result.finish();
}

pub fn intercepts_finder(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let m = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);

    let y_int = fmt_f64(b);
    let mut y_pt = "(0, ".to_string();
    y_pt.push_str(&y_int);
    y_pt.push(')');
    result.add("y-intcpt:", &y_pt);

    if libm::fabs(m) < 1e-12 {
        if libm::fabs(b) < 1e-12 {
            result.add("x-intcpt:", "all reals (y=0)");
        } else {
            result.add("x-intcpt:", "none (horizontal)");
        }
    } else {
        let x_int = -b / m;
        let x_str = try_fraction(x_int, 1000);
        let mut x_pt = "(".to_string();
        x_pt.push_str(&x_str);
        x_pt.push_str(", 0)");
        result.add("x-intcpt:", &x_pt);
    }

    result.add("Slope m:", &try_fraction(m, 100));
    let m_s = try_fraction(m, 100);
    let b_s = try_fraction(libm::fabs(b), 1000);
    let mut eq = "y = ".to_string();
    eq.push_str(&m_s);
    eq.push_str("x");
    if b >= 0.0 { eq.push_str(" + "); } else { eq.push_str(" - "); }
    eq.push_str(&b_s);
    result.add("y=mx+b:", &eq);
    result.finish();
}

pub fn par_perp_slope(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let m_num = inputs[0].parse().unwrap_or(0.0);
    let m_den = {
        let d = inputs[1].parse().unwrap_or(1.0);
        if libm::fabs(d) < 1e-12 { 1.0 } else { d }
    };
    let m = m_num / m_den;

    let m_str = try_fraction(m, 1000);
    result.add("Given m:", &m_str);
    result.add("Parallel:", &m_str);

    if libm::fabs(m) < 1e-12 {
        result.add("Perpend.:", "undefined (vert)");
    } else {
        let perp = -1.0 / m;
        result.add("Perpend.:", &try_fraction(perp, 1000));
        result.add("m * perp:", "-1 (check)");
    }
    result.finish();
}
