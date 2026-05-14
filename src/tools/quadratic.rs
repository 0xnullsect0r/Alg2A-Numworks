#[cfg(target_os = "none")]
use alloc::string::ToString;
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::ui::input::InputBuffer;
use super::{ToolResult, fmt_f64, try_fraction};

pub fn vertex_form_convert(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);

    if libm::fabs(a) < 1e-12 {
        result.add("Error:", "a cannot be 0");
        result.finish();
        return;
    }

    let h = -b / (2.0 * a);
    let k = c - (b * b) / (4.0 * a);

    let a_str = try_fraction(a, 100);
    let h_str = try_fraction(h, 1000);
    let k_str = try_fraction(k, 1000);

    let step1 = {
        let mut s = "a(x^2 + ".to_string();
        s.push_str(&try_fraction(b / a, 1000));
        s.push_str("x) + ");
        s.push_str(&fmt_f64(c));
        s
    };
    result.add("Step 1:", &step1);

    let step2 = {
        let mut s = "h=-b/2a = ".to_string();
        s.push_str(&h_str);
        s
    };
    result.add("Step 2:", &step2);

    let step3 = {
        let mut s = "k=c-b2/4a = ".to_string();
        s.push_str(&k_str);
        s
    };
    result.add("Step 3:", &step3);

    let vf = {
        let mut s = a_str.clone();
        s.push_str("(x");
        if h >= 0.0 { s.push_str(" - "); } else { s.push_str(" + "); }
        s.push_str(&try_fraction(libm::fabs(h), 1000));
        s.push_str(")^2 + ");
        s.push_str(&k_str);
        s
    };
    result.add("Vtx Form:", &vf);

    let mut vtx = "(".to_string();
    vtx.push_str(&h_str);
    vtx.push_str(", ");
    vtx.push_str(&k_str);
    vtx.push(')');
    result.add("Vertex:", &vtx);

    let mut axis = "x = ".to_string();
    axis.push_str(&h_str);
    result.add("Axis:", &axis);
    result.finish();
}

pub fn quad_analyzer(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);

    if libm::fabs(a) < 1e-12 {
        result.add("Error:", "a cannot be 0");
        result.finish();
        return;
    }

    let disc = b * b - 4.0 * a * c;
    let disc_str = try_fraction(disc, 10000);
    let h = -b / (2.0 * a);
    let k = c - (b * b) / (4.0 * a);

    result.add("Discrim:", &disc_str);

    if disc > 1e-9 {
        result.add("Roots:", "2 distinct real");
        let sqrt_disc = libm::sqrt(disc);
        let x1 = (-b + sqrt_disc) / (2.0 * a);
        let x2 = (-b - sqrt_disc) / (2.0 * a);
        result.add("x1 =", &fmt_f64(x1));
        result.add("x2 =", &fmt_f64(x2));
    } else if libm::fabs(disc) <= 1e-9 {
        result.add("Roots:", "1 repeated real");
        result.add("x =", &try_fraction(h, 1000));
    } else {
        result.add("Roots:", "2 complex (no x-int)");
        let real_part = try_fraction(-b / (2.0 * a), 1000);
        let imag_part = try_fraction(libm::sqrt(-disc) / (2.0 * libm::fabs(a)), 1000);
        let mut r1 = real_part.clone();
        r1.push_str(" + ");
        r1.push_str(&imag_part);
        r1.push_str("i");
        let mut r2 = real_part.clone();
        r2.push_str(" - ");
        r2.push_str(&imag_part);
        r2.push_str("i");
        result.add("x1 =", &r1);
        result.add("x2 =", &r2);
    }

    let mut vtx = "(".to_string();
    vtx.push_str(&try_fraction(h, 1000));
    vtx.push_str(", ");
    vtx.push_str(&try_fraction(k, 1000));
    vtx.push(')');
    result.add("Vertex:", &vtx);

    let mut axis = "x = ".to_string();
    axis.push_str(&try_fraction(h, 1000));
    result.add("Axis:", &axis);
    result.add("Opens:", if a > 0.0 { "Up (min at vertex)" } else { "Down (max at vtx)" });
    result.finish();
}

pub fn vertex_to_std(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let h = inputs[0].parse().unwrap_or(0.0);
    let k = inputs[1].parse().unwrap_or(0.0);
    let px = inputs[2].parse().unwrap_or(1.0);
    let py = inputs[3].parse().unwrap_or(0.0);

    if libm::fabs(px - h) < 1e-12 {
        result.add("Error:", "Point x = vertex x");
        result.finish();
        return;
    }

    let denom = (px - h) * (px - h);
    let a = (py - k) / denom;

    let a_str = try_fraction(a, 10000);
    let h_str = try_fraction(h, 1000);
    let k_str = try_fraction(k, 1000);

    let vf = {
        let mut s = a_str.clone();
        s.push_str("(x");
        if h >= 0.0 { s.push_str(" - "); } else { s.push_str(" + "); }
        s.push_str(&try_fraction(libm::fabs(h), 1000));
        s.push_str(")^2");
        if k >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(k), 1000));
        s
    };

    let b_coef = -2.0 * a * h;
    let c_coef = a * h * h + k;

    let std_form = {
        let mut s = a_str.clone();
        s.push_str("x^2");
        if b_coef >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(b_coef), 10000));
        s.push_str("x");
        if c_coef >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(c_coef), 10000));
        s
    };

    result.add("a =", &a_str);
    result.add("Vtx Form:", &vf);
    result.add("Std Form:", &std_form);
    let mut vtx = "(".to_string();
    vtx.push_str(&h_str);
    vtx.push_str(", ");
    vtx.push_str(&k_str);
    vtx.push(')');
    result.add("Vertex:", &vtx);
    result.finish();
}
