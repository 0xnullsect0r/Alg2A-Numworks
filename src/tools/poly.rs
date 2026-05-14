#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::ui::input::InputBuffer;
use super::{ToolResult, try_fraction, gcd};

// ── FOIL / Binomial Expand ────────────────────────────────────────────────────
// (ax + b)(cx + d)  →  ACx² + (AD+BC)x + BD

pub fn foil_expand(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(1.0);
    let d = inputs[3].parse().unwrap_or(0.0);

    let coef2 = a * c;            // x² coefficient
    let coef1 = a * d + b * c;   // x coefficient
    let coef0 = b * d;            // constant

    let b1 = fmt_binomial_factor(a, b, false);
    let b2 = fmt_binomial_factor(c, d, false);
    let mut factors = b1;
    factors.push('*');
    factors.push_str(&b2);
    result.add("Input:", &factors);

    // Show FOIL steps
    let f = try_fraction(a * c, 10000);
    let o = try_fraction(a * d, 10000);
    let i = try_fraction(b * c, 10000);
    let l = try_fraction(b * d, 10000);
    let mut foil = "F=".to_string(); foil.push_str(&f);
    foil.push_str(" O="); foil.push_str(&o);
    foil.push_str(" I="); foil.push_str(&i);
    foil.push_str(" L="); foil.push_str(&l);
    result.add("FOIL:", &foil);

    let expanded = fmt_poly3(coef2, coef1, coef0);
    result.add("Result:", &expanded);

    // Also show coefficient form
    result.add("a(x^2):", &try_fraction(coef2, 10000));
    result.add("b(x):",   &try_fraction(coef1, 10000));
    result.add("c:",      &try_fraction(coef0, 10000));
    result.finish();
}

// ── Factor Trinomial ──────────────────────────────────────────────────────────
// ax² + bx + c  →  factored form, showing steps

pub fn factor_trinomial(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a_f = inputs[0].parse().unwrap_or(1.0);
    let b_f = inputs[1].parse().unwrap_or(0.0);
    let c_f = inputs[2].parse().unwrap_or(0.0);

    // Work in integers scaled by LCM of denominators (max denom 1000)
    // For simplicity use floating approach with fraction display
    let poly_str = fmt_poly3(a_f, b_f, c_f);
    result.add("Input:", &poly_str);

    if libm::fabs(a_f) < 1e-12 {
        // Linear: bx + c = 0
        if libm::fabs(b_f) < 1e-12 {
            result.add("Form:", "Constant");
            result.finish(); return;
        }
        let root = -c_f / b_f;
        let mut s = try_fraction(b_f, 1000); s.push_str("(x - ");
        s.push_str(&try_fraction(root, 1000)); s.push(')');
        result.add("Factored:", &s);
        result.finish(); return;
    }

    // 1. Extract integer GCF from coefficients
    let (gcf, ra, rb, rc) = extract_gcf(a_f, b_f, c_f);
    if libm::fabs(gcf - 1.0) > 1e-9 {
        let gcf_str = try_fraction(gcf, 1000);
        let inner = fmt_poly3(ra, rb, rc);
        let mut with_gcf = gcf_str.clone(); with_gcf.push('(');
        with_gcf.push_str(&inner); with_gcf.push(')');
        result.add("GCF pull:", &with_gcf);
    }

    // 2. Try to factor ra·x² + rb·x + rc
    let disc = rb * rb - 4.0 * ra * rc;
    result.add("D=b^2-4ac:", &try_fraction(disc, 100000));

    if disc < -1e-9 {
        result.add("Factors:", "Not factorable (D<0)");
        // show complex roots
        let re = try_fraction(-rb / (2.0 * ra), 10000);
        let im = try_fraction(libm::sqrt(-disc) / (2.0 * libm::fabs(ra)), 10000);
        let mut r1 = re.clone(); r1.push_str("+"); r1.push_str(&im); r1.push('i');
        let mut r2 = re.clone(); r2.push_str("-"); r2.push_str(&im); r2.push('i');
        result.add("Root 1:", &r1);
        result.add("Root 2:", &r2);
        result.finish(); return;
    }

    let sqrt_disc = libm::sqrt(libm::fabs(disc));

    // Check if it's a perfect square discriminant
    let is_perfect = libm::fabs(sqrt_disc - libm::round(sqrt_disc)) < 1e-6;

    if is_perfect || libm::fabs(disc) < 1e-9 {
        // Rational roots: use AC method
        let x1 = (-rb + sqrt_disc) / (2.0 * ra);
        let x2 = (-rb - sqrt_disc) / (2.0 * ra);

        // Express as (pa·x - qa)(pb·x - qb) with integer coefficients
        // Using the AC method to find integer pair
        let factored = build_factored_form(ra, rb, rc, x1, x2, gcf);
        result.add("Factored:", &factored);
        result.add("Root 1:", &try_fraction(x1, 10000));
        if libm::fabs(disc) > 1e-9 {
            result.add("Root 2:", &try_fraction(x2, 10000));
        }
    } else {
        // Irrational roots
        result.add("Irrational roots:", "");
        let a_str = try_fraction(ra, 1000);
        let neg_b = try_fraction(-rb, 1000);
        let sqrt_str = try_fraction(sqrt_disc, 10000);
        let two_a = try_fraction(2.0 * ra, 1000);
        let mut r1 = "(".to_string(); r1.push_str(&neg_b); r1.push('+'); r1.push_str(&sqrt_str);
        r1.push_str(")/"); r1.push_str(&two_a);
        let mut r2 = "(".to_string(); r2.push_str(&neg_b); r2.push('-'); r2.push_str(&sqrt_str);
        r2.push_str(")/"); r2.push_str(&two_a);
        result.add("Root 1:", &r1);
        result.add("Root 2:", &r2);

        let gcf_str = if libm::fabs(gcf - 1.0) > 1e-9 { try_fraction(gcf, 1000) } else { String::new() };
        let mut approx = gcf_str;
        approx.push_str(&a_str);
        approx.push_str("(x-r1)(x-r2)");
        result.add("Form:", &approx);
    }

    result.finish();
}

fn build_factored_form(a: f64, _b: f64, _c: f64, x1: f64, x2: f64, gcf: f64) -> String {
    // Find integer multipliers p, q such that (px - r1*p)(qx - r2*q)/pq = a(x-x1)(x-x2)
    // Simplest: express as gcf * a * (x - x1)(x - x2) or find integer binomial form
    let gcf_str = if libm::fabs(gcf - 1.0) > 1e-9 {
        let mut s = try_fraction(gcf, 1000); s.push('*'); s
    } else { String::new() };

    // Try to express a*(x-x1)*(x-x2) with nice numerators
    // Find integers p, q, r, s such that (px+r)(qx+s) = a*x^2 - a*(x1+x2)*x + a*x1*x2
    // Use: p*q = a, p*s + q*r = -a*(x1+x2), r*s = a*x1*x2
    // Simplest form: if a is integer, try to factor into binomials
    let a_int = libm::round(a) as i64;
    let x1_n_d = best_fraction(x1, 1000);
    let x2_n_d = best_fraction(x2, 1000);

    // Factor 1: (d1*x - n1) where x1 = n1/d1
    let (n1, d1) = x1_n_d;
    let (n2, d2) = x2_n_d;

    // Check if d1*d2 == |a_int|, meaning we can write a*(x-x1)*(x-x2) as (d1*x-n1)*(d2*x-n2)
    // up to sign / GCF
    let prod_d = d1 * d2;
    let g = gcd(prod_d.abs(), a_int.abs()).max(1);
    let leftover = a_int / g; // remaining coefficient
    let scale_d = prod_d / g;

    let mut s = gcf_str;

    if libm::fabs(leftover as f64 - 1.0) < 1e-9 && libm::fabs(scale_d as f64 - 1.0) < 1e-9 {
        // Clean: just two binomials
        s.push_str(&fmt_binomial_factor(d1 as f64, -n1 as f64, true));
        s.push('*');
        s.push_str(&fmt_binomial_factor(d2 as f64, -n2 as f64, true));
    } else if libm::fabs(leftover as f64 - scale_d as f64) < 1e-9 {
        s.push_str(&fmt_binomial_factor(d1 as f64, -n1 as f64, true));
        s.push('*');
        s.push_str(&fmt_binomial_factor(d2 as f64, -n2 as f64, true));
    } else {
        // Fallback: show as a(x-r1)(x-r2)
        let a_s = try_fraction(a, 1000);
        if a_s != "1" { s.push_str(&a_s); }
        s.push_str(&fmt_binomial_factor(1.0, -x1, true));
        s.push('*');
        s.push_str(&fmt_binomial_factor(1.0, -x2, true));
    }
    s
}

fn best_fraction(v: f64, max_d: i64) -> (i64, i64) {
    if libm::fabs(v) < 1e-12 { return (0, 1); }
    let neg = v < 0.0;
    let v_abs = libm::fabs(v);
    for d in 1..=max_d {
        let n = libm::round(v_abs * d as f64) as i64;
        if libm::fabs(n as f64 / d as f64 - v_abs) < 1e-9 {
            return if neg { (-n, d) } else { (n, d) };
        }
    }
    (libm::round(v * 1000.0) as i64, 1000)
}

// ── Difference of Squares ─────────────────────────────────────────────────────
// Input: A² coefficient and B² value: Ax² - B = (√A·x - √B)(√A·x + √B)

pub fn diff_of_squares(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);

    // Expression: a*x^2 - b
    let expr = fmt_poly3(a, 0.0, -b);
    result.add("Input:", &expr);

    if a < -1e-12 || b < -1e-12 {
        result.add("Error:", "a and b must be >= 0");
        result.finish(); return;
    }

    // Check if a and b are perfect squares
    let sqrt_a = libm::sqrt(a);
    let sqrt_b = libm::sqrt(b);
    let a_is_sq = libm::fabs(sqrt_a * sqrt_a - a) < 1e-6;
    let b_is_sq = libm::fabs(sqrt_b * sqrt_b - b) < 1e-6;

    let sa_str = try_fraction(sqrt_a, 1000);
    let sb_str = try_fraction(sqrt_b, 1000);

    result.add("sqrt(a):", &sa_str);
    result.add("sqrt(b):", &sb_str);

    let f1 = fmt_binomial_factor(sqrt_a, -sqrt_b, true);
    let f2 = fmt_binomial_factor(sqrt_a, sqrt_b, true);
    let mut factored = f1; factored.push('*'); factored.push_str(&f2);
    result.add("Factored:", &factored);

    if !a_is_sq { result.set_warn("a is not a perfect square"); }
    else if !b_is_sq { result.set_warn("b is not a perfect square"); }

    result.finish();
}

// ── Perfect Square Trinomial ──────────────────────────────────────────────────
// ax² + bx + c  →  check if it equals (px + q)²

pub fn perfect_square(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);

    result.add("Input:", &fmt_poly3(a, b, c));

    if a < -1e-12 || c < -1e-12 {
        result.add("PST?:", "No (need a,c >= 0)");
        result.finish(); return;
    }

    let sqrt_a = libm::sqrt(a);
    let sqrt_c = libm::sqrt(libm::fabs(c));

    // Test (sqrt_a * x + sqrt_c)^2 = a*x^2 + 2*sqrt_a*sqrt_c*x + c
    let middle_pos = 2.0 * sqrt_a * sqrt_c;
    // Test (sqrt_a * x - sqrt_c)^2 = a*x^2 - 2*sqrt_a*sqrt_c*x + c
    let middle_neg = -2.0 * sqrt_a * sqrt_c;

    let sa_str = try_fraction(sqrt_a, 1000);
    let sc_str = try_fraction(sqrt_c, 1000);

    result.add("sqrt(a):", &sa_str);
    result.add("sqrt(c):", &sc_str);

    let mut need = "2*sqrt(a)*sqrt(c) = ".to_string();
    need.push_str(&try_fraction(middle_pos, 10000));
    result.add("Need |b|:", &need);
    result.add("Actual b:", &try_fraction(b, 1000));

    if libm::fabs(b - middle_pos) < 1e-6 {
        let mut s = "(".to_string(); s.push_str(&sa_str); s.push_str("x + "); s.push_str(&sc_str); s.push_str(")^2");
        result.add("PST! =", &s);
    } else if libm::fabs(b - middle_neg) < 1e-6 {
        let mut s = "(".to_string(); s.push_str(&sa_str); s.push_str("x - "); s.push_str(&sc_str); s.push_str(")^2");
        result.add("PST! =", &s);
    } else {
        result.add("PST?:", "Not a perfect square");
        // Show what b would need to be
        let mut hint = "b = +/-".to_string(); hint.push_str(&try_fraction(middle_pos, 10000));
        result.add("Hint:", &hint);
    }
    result.finish();
}

// ── GCF Factor ────────────────────────────────────────────────────────────────
// Enter up to 4 integer terms; outputs their GCF and factored form

pub fn gcf_factor(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);

    let ai = libm::round(libm::fabs(a)) as i64;
    let bi = libm::round(libm::fabs(b)) as i64;
    let ci = libm::round(libm::fabs(c)) as i64;

    let g = if ci == 0 {
        gcd(ai, bi).max(1)
    } else {
        gcd(gcd(ai, bi), ci).max(1)
    };

    result.add("GCF:", &super::fmt_i64_pub(g));

    let mut terms = String::new();
    for (i, coef) in [a, b, c].iter().enumerate() {
        if libm::fabs(*coef) < 1e-12 { continue; }
        let reduced = *coef / g as f64;
        let s = try_fraction(reduced, 10000);
        if !terms.is_empty() {
            if reduced >= 0.0 { terms.push_str(" + "); } else { terms.push_str(" "); }
        }
        terms.push_str(&s);
        match i {
            0 => terms.push_str("x^2"),
            1 => terms.push_str("x"),
            _ => {}
        }
    }

    let mut factored = super::fmt_i64_pub(g);
    factored.push('(');
    factored.push_str(&terms);
    factored.push(')');
    result.add("Factored:", &factored);
    result.finish();
}

// ── Polynomial Evaluator ──────────────────────────────────────────────────────
// f(x) = ax² + bx + c  at  x = val

pub fn poly_eval(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(0.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);
    let x = inputs[3].parse().unwrap_or(0.0);

    result.add("f(x):", &fmt_poly3(a, b, c));
    result.add("x =", &try_fraction(x, 1000));

    let ax2 = a * x * x;
    let bx  = b * x;
    let mut steps = try_fraction(ax2, 10000);
    steps.push_str(" + ");
    steps.push_str(&try_fraction(bx, 10000));
    steps.push_str(" + ");
    steps.push_str(&try_fraction(c, 10000));
    result.add("Steps:", &steps);

    let val = ax2 + bx + c;
    result.add("f(x) =", &try_fraction(val, 10000));
    result.finish();
}

// ── Synthetic Division ────────────────────────────────────────────────────────
// Divide ax³ + bx² + cx + d  by  (x - r)

pub fn synthetic_div(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let c = inputs[2].parse().unwrap_or(0.0);
    let d = inputs[3].parse().unwrap_or(0.0);
    let r = inputs[4].parse().unwrap_or(0.0);

    let poly = {
        let mut s = fmt_poly3(a, b, c);
        if libm::fabs(d) > 1e-12 {
            if d >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
            s.push_str(&try_fraction(libm::fabs(d), 10000));
        }
        s
    };
    result.add("Dividend:", &poly);
    let mut div_str = "(x - ".to_string();
    div_str.push_str(&try_fraction(r, 1000));
    div_str.push(')');
    result.add("Divisor:", &div_str);

    // Synthetic division algorithm: coefficients [a, b, c, d]
    let q0 = a;
    let q1 = b + q0 * r;
    let q2 = c + q1 * r;
    let rem = d + q2 * r;

    // Quotient is q0·x² + q1·x + q2, remainder = rem
    let quot = fmt_poly3(q0, q1, q2);
    result.add("Quotient:", &quot);
    result.add("Remainder:", &try_fraction(rem, 10000));

    if libm::fabs(rem) < 1e-9 {
        result.add("(x-r) is:", "a factor!");
        result.add("Root x=r:", &try_fraction(r, 1000));
    } else {
        result.set_warn("Not a factor (rem != 0)");
    }
    result.finish();
}

// ── Sum / Difference of Cubes ─────────────────────────────────────────────────
// a³ ± b³  =  (a ± b)(a² ∓ ab + b²)

pub fn sum_diff_cubes(inputs: &[InputBuffer], op: usize, result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(1.0);
    let is_sum = op == 0;

    let a3 = a * a * a;
    let b3 = b * b * b;

    let mut expr = try_fraction(a3, 10000);
    if is_sum { expr.push_str(" + "); } else { expr.push_str(" - "); }
    expr.push_str(&try_fraction(b3, 10000));
    result.add("Input:", &expr);

    result.add("a =", &try_fraction(a, 1000));
    result.add("b =", &try_fraction(b, 1000));

    // (a ± b)(a² ∓ ab + b²)
    let a_str = try_fraction(a, 1000);
    let b_str = try_fraction(b, 1000);
    let a2 = try_fraction(a * a, 10000);
    let ab = try_fraction(a * b, 10000);
    let b2 = try_fraction(b * b, 10000);

    let mut f1 = "(".to_string();
    f1.push_str(&a_str);
    if is_sum { f1.push_str(" + "); } else { f1.push_str(" - "); }
    f1.push_str(&b_str);
    f1.push(')');

    let mut f2 = "(".to_string();
    f2.push_str(&a2);
    if is_sum { f2.push_str(" - "); } else { f2.push_str(" + "); }
    f2.push_str(&ab);
    f2.push_str(" + ");
    f2.push_str(&b2);
    f2.push(')');

    result.add("Factor 1:", &f1);
    result.add("Factor 2:", &f2);

    let mut full = f1.clone(); full.push('*'); full.push_str(&f2);
    result.add("Factored:", &full);

    let rule = if is_sum { "(a+b)(a^2-ab+b^2)" } else { "(a-b)(a^2+ab+b^2)" };
    result.add("Rule:", rule);
    result.finish();
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn fmt_binomial_factor(a: f64, b: f64, parens: bool) -> String {
    let mut s = if parens { "(".to_string() } else { String::new() };
    let a_str = try_fraction(a, 10000);
    if a_str != "1" && a_str != "-1" {
        s.push_str(&a_str);
        s.push_str("x");
    } else if a_str == "-1" {
        s.push_str("-x");
    } else {
        s.push('x');
    }
    if libm::fabs(b) > 1e-12 {
        if b >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        s.push_str(&try_fraction(libm::fabs(b), 10000));
    }
    if parens { s.push(')'); }
    s
}

pub fn fmt_poly3(a: f64, b: f64, c: f64) -> String {
    let mut s = String::new();
    // x² term
    if libm::fabs(a) > 1e-12 {
        let a_str = try_fraction(a, 10000);
        if a_str != "1" && a_str != "-1" { s.push_str(&a_str); }
        else if a_str == "-1" { s.push('-'); }
        s.push_str("x^2");
    }
    // x term
    if libm::fabs(b) > 1e-12 {
        if !s.is_empty() {
            if b >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        } else if b < 0.0 { s.push('-'); }
        let b_str = try_fraction(libm::fabs(b), 10000);
        if b_str != "1" { s.push_str(&b_str); }
        s.push('x');
    }
    // constant
    if libm::fabs(c) > 1e-12 {
        if !s.is_empty() {
            if c >= 0.0 { s.push_str(" + "); } else { s.push_str(" - "); }
        } else if c < 0.0 { s.push('-'); }
        s.push_str(&try_fraction(libm::fabs(c), 10000));
    }
    if s.is_empty() { s.push('0'); }
    s
}

fn extract_gcf(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    // Find the GCF of the three floating-point coefficients (as rationals)
    // For display purposes use the smallest common integer scale
    let ai = libm::round(libm::fabs(a) * 1000.0) as i64;
    let bi = libm::round(libm::fabs(b) * 1000.0) as i64;
    let ci = libm::round(libm::fabs(c) * 1000.0) as i64;
    let g_int = if ci == 0 { gcd(ai, bi).max(1) } else { gcd(gcd(ai, bi), ci).max(1) };
    let g = g_int as f64 / 1000.0;
    (g, a / g, b / g, c / g)
}
