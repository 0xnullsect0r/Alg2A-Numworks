/// Simplifier — step-by-step algebraic simplification tools.
/// Each function shows every intermediate step and labels the rule used.

#[cfg(target_os = "none")]
use alloc::{string::{String, ToString}, vec::Vec};
#[cfg(target_os = "none")]
use alloc::vec;
#[cfg(target_os = "none")]
use alloc::format;
#[cfg(target_os = "none")]
use alloc::boxed::Box;
#[cfg(not(target_os = "none"))]
use std::{string::{String, ToString}, vec::Vec};

use crate::ui::input::InputBuffer;
use super::{ToolResult, fmt_f64, try_fraction, gcd};

// ── Helper: format  c·x^n  as a readable monomial string ─────────────────────
// coef=1 x^1 → "x", coef=2 x^3 → "2x^3", coef=1 x^0 → "1", etc.
fn fmt_mono(coef: f64, exp: i64) -> String {
    let c_str = try_fraction(libm::fabs(coef), 10000);
    let neg = coef < -1e-12;
    let mut s = if neg { "-".to_string() } else { String::new() };
    if exp == 0 {
        s.push_str(&c_str); return s;
    }
    if libm::fabs(libm::fabs(coef) - 1.0) > 1e-9 { s.push_str(&c_str); }
    s.push('x');
    if exp != 1 { s.push('^'); s.push_str(&super::fmt_i64_pub(exp)); }
    s
}

fn fmt_mono_var(coef: f64, var: &str, exp: i64) -> String {
    let c_str = try_fraction(libm::fabs(coef), 10000);
    let neg = coef < -1e-12;
    let mut s = if neg { "-".to_string() } else { String::new() };
    if exp == 0 { s.push_str(&c_str); return s; }
    if libm::fabs(libm::fabs(coef) - 1.0) > 1e-9 { s.push_str(&c_str); }
    s.push_str(var);
    if exp != 1 { s.push('^'); s.push_str(&super::fmt_i64_pub(exp)); }
    s
}

// ── 1. Multiply Monomials ─────────────────────────────────────────────────────
// c1·x^a  ×  c2·x^b  =  (c1·c2)·x^(a+b)
// Rule: Product of Powers — x^m · x^n = x^(m+n)

pub fn multiply_monomials(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let c1 = inputs[0].parse().unwrap_or(1.0);
    let a  = inputs[1].parse().unwrap_or(1.0) as i64;
    let c2 = inputs[2].parse().unwrap_or(1.0);
    let b  = inputs[3].parse().unwrap_or(1.0) as i64;

    let m1 = fmt_mono(c1, a);
    let m2 = fmt_mono(c2, b);
    let mut input_str = m1.clone(); input_str.push_str(" * "); input_str.push_str(&m2);
    result.add("Input:", &input_str);
    result.add("Rule:", "x^m * x^n = x^(m+n)");

    // Step 1: multiply coefficients
    let c_prod = c1 * c2;
    let mut step1 = "Coefs: (".to_string();
    step1.push_str(&try_fraction(c1, 1000)); step1.push_str(")(");
    step1.push_str(&try_fraction(c2, 1000)); step1.push_str(") = ");
    step1.push_str(&try_fraction(c_prod, 10000));
    result.add("Step 1:", &step1);

    // Step 2: add exponents
    let exp_sum = a + b;
    let mut step2 = "Exps: ".to_string();
    step2.push_str(&super::fmt_i64_pub(a)); step2.push_str(" + ");
    step2.push_str(&super::fmt_i64_pub(b)); step2.push_str(" = ");
    step2.push_str(&super::fmt_i64_pub(exp_sum));
    result.add("Step 2:", &step2);

    result.add("Result:", &fmt_mono(c_prod, exp_sum));
    result.finish();
}

// ── 2. Divide Monomials ───────────────────────────────────────────────────────
// c1·x^a  ÷  c2·x^b  =  (c1/c2)·x^(a-b)
// Rule: Quotient of Powers — x^m / x^n = x^(m-n)

pub fn divide_monomials(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let c1 = inputs[0].parse().unwrap_or(1.0);
    let a  = inputs[1].parse().unwrap_or(1.0) as i64;
    let c2 = inputs[2].parse().unwrap_or(1.0);
    let b  = inputs[3].parse().unwrap_or(1.0) as i64;

    if libm::fabs(c2) < 1e-12 {
        result.add("Error:", "Cannot divide by 0");
        result.finish(); return;
    }

    let mut input_str = fmt_mono(c1, a);
    input_str.push_str(" / ");
    input_str.push_str(&fmt_mono(c2, b));
    result.add("Input:", &input_str);
    result.add("Rule:", "x^m / x^n = x^(m-n)");

    let c_quot = c1 / c2;
    let mut step1 = "Coefs: ".to_string();
    step1.push_str(&try_fraction(c1, 1000)); step1.push_str(" / ");
    step1.push_str(&try_fraction(c2, 1000)); step1.push_str(" = ");
    step1.push_str(&try_fraction(c_quot, 10000));
    result.add("Step 1:", &step1);

    let exp_diff = a - b;
    let mut step2 = "Exps: ".to_string();
    step2.push_str(&super::fmt_i64_pub(a)); step2.push_str(" - ");
    step2.push_str(&super::fmt_i64_pub(b)); step2.push_str(" = ");
    step2.push_str(&super::fmt_i64_pub(exp_diff));
    result.add("Step 2:", &step2);

    if exp_diff == 0 {
        result.add("Note:", "x^0 = 1 (zero exp rule)");
        result.add("Result:", &try_fraction(c_quot, 10000));
    } else if exp_diff < 0 {
        result.add("Note:", "Neg exp -> move to denom");
        let mut res = "1 / ".to_string();
        res.push_str(&fmt_mono(1.0 / c_quot, -exp_diff));
        result.add("Result:", &res);
    } else {
        result.add("Result:", &fmt_mono(c_quot, exp_diff));
    }
    result.finish();
}

// ── 3. Power of a Power ───────────────────────────────────────────────────────
// (x^a)^b  =  x^(a·b)
// Rule: Power of a Power — (x^m)^n = x^(m·n)

pub fn power_of_power(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let c = inputs[0].parse().unwrap_or(1.0);
    let a = inputs[1].parse().unwrap_or(1.0) as i64;
    let b = inputs[2].parse().unwrap_or(1.0) as i64;

    let mut input_str = "(".to_string();
    input_str.push_str(&fmt_mono(c, a));
    input_str.push_str(")^");
    input_str.push_str(&super::fmt_i64_pub(b));
    result.add("Input:", &input_str);
    result.add("Rule:", "(x^m)^n = x^(m*n)");

    // Step 1: raise coefficient to power b
    let c_pow = libm::pow(libm::fabs(c), b as f64) * if c < 0.0 && b % 2 != 0 { -1.0 } else { 1.0 };
    let mut step1 = "Coef: (".to_string();
    step1.push_str(&try_fraction(c, 1000));
    step1.push_str(")^");
    step1.push_str(&super::fmt_i64_pub(b));
    step1.push_str(" = ");
    step1.push_str(&try_fraction(c_pow, 10000));
    result.add("Step 1:", &step1);

    // Step 2: multiply exponents
    let exp_prod = a * b;
    let mut step2 = "Exps: ".to_string();
    step2.push_str(&super::fmt_i64_pub(a));
    step2.push_str(" * ");
    step2.push_str(&super::fmt_i64_pub(b));
    step2.push_str(" = ");
    step2.push_str(&super::fmt_i64_pub(exp_prod));
    result.add("Step 2:", &step2);

    if exp_prod < 0 {
        result.add("Note:", "Neg exp -> move to denom");
        let mut res = "1 / ".to_string();
        res.push_str(&fmt_mono(1.0 / c_pow, -exp_prod));
        result.add("Result:", &res);
    } else {
        result.add("Result:", &fmt_mono(c_pow, exp_prod));
    }
    result.finish();
}

// ── 4. Power of a Product ─────────────────────────────────────────────────────
// (c·x^a)^n  =  c^n · x^(a·n)
// Rule: Power of a Product — (ab)^n = a^n · b^n

pub fn power_of_product(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let c = inputs[0].parse().unwrap_or(1.0);
    let a = inputs[1].parse().unwrap_or(1.0) as i64;
    let n = inputs[2].parse().unwrap_or(2.0) as i64;

    let mut input_str = "(".to_string();
    input_str.push_str(&fmt_mono(c, a));
    input_str.push_str(")^");
    input_str.push_str(&super::fmt_i64_pub(n));
    result.add("Input:", &input_str);
    result.add("Rule:", "(ab)^n = a^n * b^n");

    let c_pow = libm::pow(libm::fabs(c), n as f64) * if c < 0.0 && n % 2 != 0 { -1.0 } else { 1.0 };
    let exp_new = a * n;

    let mut step1 = "Coef: (".to_string();
    step1.push_str(&try_fraction(c, 1000));
    step1.push_str(")^");
    step1.push_str(&super::fmt_i64_pub(n));
    step1.push_str(" = ");
    step1.push_str(&try_fraction(c_pow, 10000));
    result.add("Step 1:", &step1);

    let mut step2 = "Exp: ".to_string();
    step2.push_str(&super::fmt_i64_pub(a));
    step2.push_str(" * ");
    step2.push_str(&super::fmt_i64_pub(n));
    step2.push_str(" = ");
    step2.push_str(&super::fmt_i64_pub(exp_new));
    result.add("Step 2:", &step2);

    result.add("Result:", &fmt_mono(c_pow, exp_new));
    result.finish();
}

// ── 5. Negative Exponent ──────────────────────────────────────────────────────
// c·x^-n  =  c / x^n     (and reverse)
// Rule: Negative Exponent — x^-n = 1/x^n

pub fn negative_exponent(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let c = inputs[0].parse().unwrap_or(1.0);
    let n = inputs[1].parse().unwrap_or(1.0) as i64; // user enters the negative exp as positive; sign shown

    let neg_exp = -n;
    let mut input_str = fmt_mono(c, neg_exp);
    result.add("Input:", &input_str);
    result.add("Rule:", "x^-n = 1/x^n");

    let mut step1 = "Split: ".to_string();
    step1.push_str(&try_fraction(c, 1000));
    step1.push_str(" * x^(-");
    step1.push_str(&super::fmt_i64_pub(n));
    step1.push(')');
    result.add("Step 1:", &step1);

    let mut step2 = "x^-".to_string();
    step2.push_str(&super::fmt_i64_pub(n));
    step2.push_str(" = 1/x^");
    step2.push_str(&super::fmt_i64_pub(n));
    result.add("Step 2:", &step2);

    let c_str = try_fraction(c, 1000);
    let mut res = c_str.clone();
    res.push_str("/x^");
    res.push_str(&super::fmt_i64_pub(n));
    result.add("Result:", &res);

    if n == 0 {
        result.add("Note:", "x^0 = 1");
    }
    result.finish();
}

// ── 6. Combine Like Terms ─────────────────────────────────────────────────────
// a·x^n + b·x^n + c·x^n  =  (a+b+c)·x^n
// Rule: Like Terms — same variable & exponent, add coefficients

pub fn combine_like_terms(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let a  = inputs[0].parse().unwrap_or(0.0);
    let b  = inputs[1].parse().unwrap_or(0.0);
    let c  = inputs[2].parse().unwrap_or(0.0);
    let n  = inputs[3].parse().unwrap_or(1.0) as i64;

    // Build the input expression string
    let mut expr = fmt_mono(a, n);
    if libm::fabs(b) > 1e-12 {
        if b >= 0.0 { expr.push_str(" + "); } else { expr.push_str(" - "); }
        expr.push_str(&fmt_mono(libm::fabs(b), n));
    }
    if libm::fabs(c) > 1e-12 {
        if c >= 0.0 { expr.push_str(" + "); } else { expr.push_str(" - "); }
        expr.push_str(&fmt_mono(libm::fabs(c), n));
    }
    result.add("Input:", &expr);
    result.add("Rule:", "ax^n+bx^n = (a+b)x^n");

    // Identify like terms
    result.add("Like?:", "All have same x^n ✓");

    // Step: add coefficients
    let sum = a + b + c;
    let mut step = "Coefs: ".to_string();
    step.push_str(&try_fraction(a, 1000));
    if b >= 0.0 { step.push_str(" + "); } else { step.push_str(" - "); }
    step.push_str(&try_fraction(libm::fabs(b), 1000));
    if libm::fabs(c) > 1e-12 {
        if c >= 0.0 { step.push_str(" + "); } else { step.push_str(" - "); }
        step.push_str(&try_fraction(libm::fabs(c), 1000));
    }
    step.push_str(" = ");
    step.push_str(&try_fraction(sum, 10000));
    result.add("Step:", &step);

    if libm::fabs(sum) < 1e-12 {
        result.add("Result:", "0  (terms cancel)");
    } else {
        result.add("Result:", &fmt_mono(sum, n));
    }
    result.finish();
}

// ── 7. Distribute ─────────────────────────────────────────────────────────────
// a · (b·x^p + c·x^q)  →  expand each term
// Rule: Distributive Property — a(b+c) = ab + ac

pub fn distribute(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let outer_c = inputs[0].parse().unwrap_or(1.0); // outer coefficient
    let outer_e = inputs[1].parse().unwrap_or(0.0) as i64; // outer exponent
    let t1_c    = inputs[2].parse().unwrap_or(1.0); // term 1 coef
    let t1_e    = inputs[3].parse().unwrap_or(1.0) as i64; // term 1 exp
    let t2_c    = inputs[4].parse().unwrap_or(0.0); // term 2 coef
    let t2_e    = inputs[5].parse().unwrap_or(0.0) as i64; // term 2 exp

    let outer = fmt_mono(outer_c, outer_e);
    let term1 = fmt_mono(t1_c, t1_e);

    let mut inside = term1.clone();
    if libm::fabs(t2_c) > 1e-12 {
        if t2_c >= 0.0 { inside.push_str(" + "); } else { inside.push_str(" - "); }
        inside.push_str(&fmt_mono(libm::fabs(t2_c), t2_e));
    }

    let mut input_str = outer.clone();
    input_str.push('(');
    input_str.push_str(&inside);
    input_str.push(')');
    result.add("Input:", &input_str);
    result.add("Rule:", "a(b+c) = ab + ac");

    // Distribute to term 1
    let r1_c = outer_c * t1_c;
    let r1_e = outer_e + t1_e;
    let r1 = fmt_mono(r1_c, r1_e);
    let mut s1 = outer.clone(); s1.push_str("*"); s1.push_str(&term1); s1.push_str("="); s1.push_str(&r1);
    result.add("Term 1:", &s1);

    if libm::fabs(t2_c) > 1e-12 {
        let r2_c = outer_c * t2_c;
        let r2_e = outer_e + t2_e;
        let r2 = fmt_mono(r2_c, r2_e);
        let t2_str = fmt_mono(libm::fabs(t2_c), t2_e);
        let mut s2 = outer.clone(); s2.push_str("*"); s2.push_str(&t2_str); s2.push_str("="); s2.push_str(&r2);
        result.add("Term 2:", &s2);

        let mut res = r1.clone();
        if r2_c >= 0.0 { res.push_str(" + "); } else { res.push_str(" "); }
        res.push_str(&r2);
        result.add("Result:", &res);
    } else {
        result.add("Result:", &r1);
    }
    result.finish();
}

// ── 8. Binomial Power Expand ──────────────────────────────────────────────────
// (ax + b)^n  for n = 2 or 3, using binomial theorem / Pascal's triangle
// Rule: Binomial Theorem — (a+b)^n = Σ C(n,k) a^(n-k) b^k

pub fn binomial_power(inputs: &[InputBuffer], n_override: usize, result: &mut ToolResult) {
    result.clear();
    let a = inputs[0].parse().unwrap_or(1.0);
    let b = inputs[1].parse().unwrap_or(0.0);
    let n = (n_override + 2) as i64; // 0→n=2, 1→n=3

    let a_str = try_fraction(libm::fabs(a), 1000);
    let b_str = try_fraction(libm::fabs(b), 1000);

    let mut input_str = "(".to_string();
    if libm::fabs(libm::fabs(a) - 1.0) > 1e-9 { input_str.push_str(&a_str); }
    input_str.push('x');
    if b >= 0.0 { input_str.push_str(" + "); } else { input_str.push_str(" - "); }
    input_str.push_str(&b_str);
    input_str.push_str(")^");
    input_str.push_str(&super::fmt_i64_pub(n));
    result.add("Input:", &input_str);

    if n == 2 {
        result.add("Rule:", "(ax+b)^2=a^2x^2+2abx+b^2");
        let a2 = a * a;
        let two_ab = 2.0 * a * b;
        let b2 = b * b;

        let mut s1 = "a^2x^2: (".to_string(); s1.push_str(&a_str); s1.push_str(")^2 = "); s1.push_str(&try_fraction(a2, 10000)); result.add("C1:", &s1);
        let mut s2 = "2abx: 2*".to_string(); s2.push_str(&a_str); s2.push('*'); s2.push_str(&b_str); s2.push_str(" = "); s2.push_str(&try_fraction(two_ab, 10000)); result.add("C2:", &s2);
        let mut s3 = "b^2: (".to_string(); s3.push_str(&b_str); s3.push_str(")^2 = "); s3.push_str(&try_fraction(b2, 10000)); result.add("C3:", &s3);

        result.add("Result:", &super::poly::fmt_poly3(a2, two_ab, b2));
    } else {
        // n == 3: Pascal row [1, 3, 3, 1]
        result.add("Rule:", "(ax+b)^3: Pascal [1,3,3,1]");
        let c0 = a * a * a;                            // 1 * a^3 * b^0 * x^3
        let c1 = 3.0 * a * a * b;                     // 3 * a^2 * b^1 * x^2
        let c2 = 3.0 * a * b * b;                     // 3 * a^1 * b^2 * x^1
        let c3 = b * b * b;                             // 1 * a^0 * b^3 * x^0

        let mut s0 = "1*a^3: ".to_string(); s0.push_str(&try_fraction(c0, 10000)); s0.push_str("x^3"); result.add("Term1:", &s0);
        let mut s1 = "3*a^2*b: ".to_string(); s1.push_str(&try_fraction(c1, 10000)); s1.push_str("x^2"); result.add("Term2:", &s1);
        let mut s2 = "3*a*b^2: ".to_string(); s2.push_str(&try_fraction(c2, 10000)); s2.push('x'); result.add("Term3:", &s2);
        let mut s3 = "b^3: ".to_string(); s3.push_str(&try_fraction(c3, 10000)); result.add("Term4:", &s3);

        let mut res = try_fraction(c0, 10000); res.push_str("x^3");
        if c1 >= 0.0 { res.push_str(" + "); } else { res.push_str(" - "); }
        res.push_str(&try_fraction(libm::fabs(c1), 10000)); res.push_str("x^2");
        if c2 >= 0.0 { res.push_str(" + "); } else { res.push_str(" - "); }
        res.push_str(&try_fraction(libm::fabs(c2), 10000)); res.push('x');
        if c3 >= 0.0 { res.push_str(" + "); } else { res.push_str(" - "); }
        res.push_str(&try_fraction(libm::fabs(c3), 10000));
        result.add("Result:", &res);
    }
    result.finish();
}

// ── 9. Zero, One & Identity Rules ─────────────────────────────────────────────
// Shows all exponent identity rules for the entered value

pub fn exp_identity_rules(inputs: &[InputBuffer], result: &mut ToolResult) {
    result.clear();
    let x = inputs[0].parse().unwrap_or(2.0);
    let x_str = try_fraction(x, 1000);

    result.add("Base x =", &x_str);
    result.add("---", "--- Exponent Rules ---");

    // x^0 = 1
    result.add("x^0 = 1:", "ANY base^0 = 1 (not 0^0)");
    let mut r0 = x_str.clone(); r0.push_str("^0 = 1"); result.add("  =>", &r0);

    // x^1 = x
    result.add("x^1 = x:", "Power of 1 = identity");
    let mut r1 = x_str.clone(); r1.push_str("^1 = "); r1.push_str(&x_str); result.add("  =>", &r1);

    // 1^n = 1
    result.add("1^n = 1:", "1 to any power = 1");

    // 0^n = 0 (n > 0)
    result.add("0^n = 0:", "0 to any pos power = 0");

    // x^-1 = 1/x
    if libm::fabs(x) > 1e-12 {
        let inv = 1.0 / x;
        let mut r_inv = x_str.clone(); r_inv.push_str("^-1 = 1/"); r_inv.push_str(&x_str);
        r_inv.push_str(" = "); r_inv.push_str(&try_fraction(inv, 10000));
        result.add("x^-1:", &r_inv);
    } else {
        result.add("x^-1:", "Undef (x=0)");
    }

    // x^(1/2) = sqrt(x)  (if x >= 0)
    if x >= 0.0 {
        let sq = libm::sqrt(x);
        let mut r_sq = x_str.clone(); r_sq.push_str("^(1/2) = sqrt = "); r_sq.push_str(&try_fraction(sq, 10000));
        result.add("x^(1/2):", &r_sq);
    }

    result.finish();
}

// ── General Expression Simplifier ────────────────────────────────────────────

#[derive(Clone)]
enum GToken {
    Num(f64),
    Var,
    Plus, Minus, Star, Slash, Caret, LParen, RParen,
}

fn g_tokenize(s: &str) -> Vec<GToken> {
    let mut tokens = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'=' => { i += 1; }
            b'0'..=b'9' | b'.' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                let s2 = core::str::from_utf8(&bytes[start..i]).unwrap_or("0");
                if let Some(n) = crate::ui::input::parse_f64(s2) {
                    tokens.push(GToken::Num(n));
                }
            }
            b'x' | b'X' => { tokens.push(GToken::Var); i += 1; }
            b'+' => { tokens.push(GToken::Plus); i += 1; }
            b'-' => { tokens.push(GToken::Minus); i += 1; }
            b'*' => { tokens.push(GToken::Star); i += 1; }
            b'/' => { tokens.push(GToken::Slash); i += 1; }
            b'^' => { tokens.push(GToken::Caret); i += 1; }
            b'(' => { tokens.push(GToken::LParen); i += 1; }
            b')' => { tokens.push(GToken::RParen); i += 1; }
            _ => { i += 1; }
        }
    }
    tokens
}

#[derive(Clone)]
struct GTerm { coeff: f64, power: u32 }
type GPoly = Vec<GTerm>;

fn g_const(c: f64) -> GPoly { vec![GTerm { coeff: c, power: 0 }] }
fn g_term(c: f64, p: u32) -> GPoly { vec![GTerm { coeff: c, power: p }] }

fn g_scale(p: &GPoly, s: f64) -> GPoly {
    p.iter().map(|t| GTerm { coeff: t.coeff * s, power: t.power }).collect()
}

fn g_add(a: &GPoly, b: &GPoly) -> GPoly {
    let mut r = a.clone(); r.extend(b.iter().cloned()); r
}

fn g_sub(a: &GPoly, b: &GPoly) -> GPoly { g_add(a, &g_scale(b, -1.0)) }

fn g_mul(a: &GPoly, b: &GPoly) -> GPoly {
    let mut r = Vec::new();
    for ta in a { for tb in b {
        r.push(GTerm { coeff: ta.coeff * tb.coeff, power: ta.power + tb.power });
    }}
    r
}

fn g_pow(p: &GPoly, n: u32) -> GPoly {
    if n == 0 { return g_const(1.0); }
    let mut r = p.clone();
    for _ in 1..n { r = g_mul(&r, p); }
    r
}

fn g_collect(p: GPoly) -> GPoly {
    let mut map: Vec<(u32, f64)> = Vec::new();
    for t in &p {
        if let Some(e) = map.iter_mut().find(|(pw, _)| *pw == t.power) { e.1 += t.coeff; }
        else { map.push((t.power, t.coeff)); }
    }
    let mut r: GPoly = map.into_iter()
        .filter(|(_, c)| libm::fabs(*c) > 1e-12)
        .map(|(power, coeff)| GTerm { coeff, power })
        .collect();
    r.sort_by(|a, b| b.power.cmp(&a.power));
    if r.is_empty() { r.push(GTerm { coeff: 0.0, power: 0 }); }
    r
}

fn g_fmt(p: &GPoly) -> String {
    if p.is_empty() { return "0".into(); }
    let mut s = String::new();
    for (i, t) in p.iter().enumerate() {
        let c = t.coeff; let pw = t.power;
        let ac = libm::fabs(c);
        if i > 0 {
            if c < 0.0 { s.push_str(" - "); } else { s.push_str(" + "); }
        }
        let display_c = if i == 0 { c } else { ac };
        match pw {
            0 => s.push_str(&g_fmt_num(display_c)),
            1 => {
                if ac == 1.0 {
                    if i == 0 && c < 0.0 { s.push_str("-x"); } else { s.push('x'); }
                } else {
                    s.push_str(&format!("{}x", g_fmt_num(display_c)));
                }
            }
            _ => {
                if ac == 1.0 {
                    if i == 0 && c < 0.0 { s.push_str(&format!("-x^{}", pw)); }
                    else { s.push_str(&format!("x^{}", pw)); }
                } else {
                    s.push_str(&format!("{}x^{}", g_fmt_num(display_c), pw));
                }
            }
        }
    }
    if s.is_empty() { "0".into() } else { s }
}

fn g_fmt_num(v: f64) -> String {
    if v == libm::round(v) && libm::fabs(v) < 1e9 { format!("{}", v as i64) }
    else { format!("{:.3}", v) }
}

struct GParser<'a> {
    toks: &'a [GToken],
    pos: usize,
    steps: Vec<(String, String)>,
}

impl<'a> GParser<'a> {
    fn new(toks: &'a [GToken]) -> Self { GParser { toks, pos: 0, steps: Vec::new() } }

    fn peek(&self) -> Option<&GToken> { self.toks.get(self.pos) }

    fn peek2(&self) -> Option<&GToken> { self.toks.get(self.pos + 1) }

    fn consume(&mut self) { self.pos += 1; }

    fn parse_expr(&mut self) -> Result<GPoly, &'static str> {
        let neg = if matches!(self.peek(), Some(GToken::Minus)) {
            self.consume(); true
        } else { false };

        let mut left = self.parse_term()?;
        if neg { left = g_scale(&left, -1.0); }

        loop {
            match self.peek() {
                Some(GToken::Plus) => {
                    self.consume();
                    let right = self.parse_term()?;
                    let before_terms = g_collect(left.clone());
                    let combined = g_collect(g_add(&left, &right));
                    if combined.len() < before_terms.len() + right.len() {
                        self.steps.push(("Combine".into(),
                            format!("{} + {} = {}", g_fmt(&before_terms), g_fmt(&right), g_fmt(&combined))));
                    }
                    left = combined;
                }
                Some(GToken::Minus) => {
                    self.consume();
                    let right = self.parse_term()?;
                    let before_terms = g_collect(left.clone());
                    let combined = g_collect(g_sub(&left, &right));
                    if combined.len() < before_terms.len() + right.len() {
                        self.steps.push(("Combine".into(),
                            format!("{} - {} = {}", g_fmt(&before_terms), g_fmt(&right), g_fmt(&combined))));
                    }
                    left = combined;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<GPoly, &'static str> {
        let mut left = self.parse_power()?;
        loop {
            match self.peek() {
                Some(GToken::Star) => {
                    self.consume();
                    let right = self.parse_power()?;
                    let ls = g_fmt(&left); let rs = g_fmt(&right);
                    left = g_collect(g_mul(&left, &right));
                    self.steps.push(("Multiply".into(), format!("({})({}) = {}", ls, rs, g_fmt(&left))));
                }
                Some(GToken::Slash) => {
                    self.consume();
                    let right = self.parse_power()?;
                    if right.len() == 1 && right[0].power == 0 && libm::fabs(right[0].coeff) > 1e-12 {
                        let d = right[0].coeff;
                        let ls = g_fmt(&left);
                        left = g_collect(g_scale(&left, 1.0 / d));
                        self.steps.push(("Divide".into(), format!("({}) / {} = {}", ls, g_fmt_num(d), g_fmt(&left))));
                    } else {
                        return Err("Can't divide by poly");
                    }
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<GPoly, &'static str> {
        let base = self.parse_primary()?;
        if matches!(self.peek(), Some(GToken::Caret)) {
            self.consume();
            if let Some(GToken::Num(e)) = self.peek().cloned() {
                self.consume();
                let exp = e as u32;
                let bs = g_fmt(&base);
                let result = g_collect(g_pow(&base, exp));
                if base.len() > 1 || exp > 1 {
                    self.steps.push(("Expand".into(), format!("({}^{}) = {}", bs, exp, g_fmt(&result))));
                }
                return Ok(result);
            }
        }
        Ok(base)
    }

    fn parse_primary(&mut self) -> Result<GPoly, &'static str> {
        match self.peek().cloned() {
            Some(GToken::Num(n)) => {
                self.consume();
                match self.peek() {
                    Some(GToken::Var) => {
                        self.consume();
                        let exp = if matches!(self.peek(), Some(GToken::Caret)) {
                            self.consume();
                            if let Some(GToken::Num(e)) = self.peek().cloned() { self.consume(); e as u32 }
                            else { return Err("Expected exponent"); }
                        } else { 1 };
                        Ok(g_term(n, exp))
                    }
                    Some(GToken::LParen) => {
                        let inner = self.parse_paren()?;
                        let is = g_fmt(&inner);
                        let result = g_collect(g_scale(&inner, n));
                        self.steps.push(("Distribute".into(), format!("{}({}) = {}", g_fmt_num(n), is, g_fmt(&result))));
                        if matches!(self.peek(), Some(GToken::LParen)) {
                            let right = self.parse_paren()?;
                            let ls = g_fmt(&result); let rs = g_fmt(&right);
                            self.push_foil_steps(&result, &right);
                            let mul = g_collect(g_mul(&result, &right));
                            self.steps.push(("Multiply".into(), format!("({})({}) = {}", ls, rs, g_fmt(&mul))));
                            return Ok(mul);
                        }
                        Ok(result)
                    }
                    _ => Ok(g_const(n)),
                }
            }
            Some(GToken::Var) => {
                self.consume();
                let exp = if matches!(self.peek(), Some(GToken::Caret)) {
                    self.consume();
                    if let Some(GToken::Num(e)) = self.peek().cloned() { self.consume(); e as u32 }
                    else { return Err("Expected exponent"); }
                } else { 1 };
                Ok(g_term(1.0, exp))
            }
            Some(GToken::LParen) => {
                let left = self.parse_paren()?;
                if matches!(self.peek(), Some(GToken::LParen)) {
                    let right = self.parse_paren()?;
                    let ls = g_fmt(&left); let rs = g_fmt(&right);
                    self.push_foil_steps(&left, &right);
                    let result = g_collect(g_mul(&left, &right));
                    self.steps.push(("Multiply".into(), format!("({})({}) = {}", ls, rs, g_fmt(&result))));
                    if matches!(self.peek(), Some(GToken::LParen)) {
                        let third = self.parse_paren()?;
                        let ls2 = g_fmt(&result); let rs2 = g_fmt(&third);
                        let result2 = g_collect(g_mul(&result, &third));
                        self.steps.push(("Multiply".into(), format!("({})({}) = {}", ls2, rs2, g_fmt(&result2))));
                        return Ok(result2);
                    }
                    return Ok(result);
                }
                Ok(left)
            }
            Some(GToken::Minus) => {
                self.consume();
                let inner = self.parse_primary()?;
                Ok(g_scale(&inner, -1.0))
            }
            _ => Err("Unexpected token"),
        }
    }

    fn parse_paren(&mut self) -> Result<GPoly, &'static str> {
        if !matches!(self.peek(), Some(GToken::LParen)) { return Err("Expected ("); }
        self.consume();
        let inner = self.parse_expr()?;
        if !matches!(self.peek(), Some(GToken::RParen)) { return Err("Expected )"); }
        self.consume();
        Ok(inner)
    }

    fn push_foil_steps(&mut self, a: &GPoly, b: &GPoly) {
        if a.len() == 2 && b.len() == 2 {
            let f = g_mul(&vec![a[0].clone()], &vec![b[0].clone()]);
            let o = g_mul(&vec![a[0].clone()], &vec![b[1].clone()]);
            let ii = g_mul(&vec![a[1].clone()], &vec![b[0].clone()]);
            let l = g_mul(&vec![a[1].clone()], &vec![b[1].clone()]);
            self.steps.push(("FOIL F".into(), format!("{}*{} = {}", g_fmt(&vec![a[0].clone()]), g_fmt(&vec![b[0].clone()]), g_fmt(&f))));
            self.steps.push(("FOIL O".into(), format!("{}*{} = {}", g_fmt(&vec![a[0].clone()]), g_fmt(&vec![b[1].clone()]), g_fmt(&o))));
            self.steps.push(("FOIL I".into(), format!("{}*{} = {}", g_fmt(&vec![a[1].clone()]), g_fmt(&vec![b[0].clone()]), g_fmt(&ii))));
            self.steps.push(("FOIL L".into(), format!("{}*{} = {}", g_fmt(&vec![a[1].clone()]), g_fmt(&vec![b[1].clone()]), g_fmt(&l))));
        }
    }
}

pub fn general_simplify(expr: &str) -> ToolResult {
    let mut result = ToolResult::new();
    let expr = expr.trim();
    if expr.is_empty() {
        result.set_warn("Enter an expression");
        result.finish();
        return result;
    }

    // Detect if expression uses functions or constants → use FExpr path
    let has_fns = expr.contains("sin") || expr.contains("cos") || expr.contains("tan")
        || expr.contains("sqrt") || expr.contains("ln") || expr.contains("log")
        || expr.contains("pi") || expr.contains("Pi") || expr.contains("PI")
        || expr.contains('e');

    if has_fns {
        let tokens = f_tokenize(expr);
        if tokens.is_empty() { result.set_warn("Empty expression"); result.finish(); return result; }
        let mut parser = FParser::new(&tokens);
        match parser.parse_expr() {
            Ok(tree) => {
                let mut steps: Vec<(String, String)> = Vec::new();
                let simplified = f_simplify(&tree, &mut steps);
                let final_str = if let Some(v) = f_eval(&simplified) {
                    // Try nice radical form for sqrt results
                    g_fmt_num(v)
                } else {
                    f_fmt(&simplified)
                };
                result.add("Result:", &final_str);
                if steps.is_empty() {
                    result.add("Note:", "Already simplified");
                } else {
                    for (label, desc) in &steps { result.add(label, desc); }
                }
            }
            Err(e) => { result.set_warn(&format!("Error: {}", e)); }
        }
    } else {
        let tokens = g_tokenize(expr);
        if tokens.is_empty() { result.set_warn("Empty expression"); result.finish(); return result; }
        let mut parser = GParser::new(&tokens);
        match parser.parse_expr() {
            Ok(poly) => {
                let final_poly = g_collect(poly);
                let final_str = g_fmt(&final_poly);
                result.add("Simplified:", &final_str);
                let degree = final_poly.iter().map(|t| t.power).max().unwrap_or(0);
                if degree > 0 { result.add("Degree:", &format!("{}", degree)); }
                if parser.steps.is_empty() {
                    result.add("Note:", "Already in simplest form");
                } else {
                    for (label, desc) in &parser.steps { result.add(label, desc); }
                }
            }
            Err(e) => { result.set_warn(&format!("Parse error: {}", e)); }
        }
    }
    result.finish();
    result
}

// ── Function-aware expression evaluator ──────────────────────────────────────

#[derive(Clone)]
enum FToken {
    Num(f64), Var, Pi, ConstE,
    FnSin, FnCos, FnTan, FnSqrt, FnLn, FnLog,
    Plus, Minus, Star, Slash, Caret, LParen, RParen,
}

fn f_tokenize(s: &str) -> Vec<FToken> {
    let mut tokens = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'=' => { i += 1; }
            b'0'..=b'9' | b'.' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') { i += 1; }
                let s2 = core::str::from_utf8(&bytes[start..i]).unwrap_or("0");
                if let Some(n) = crate::ui::input::parse_f64(s2) { tokens.push(FToken::Num(n)); }
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                let start = i;
                while i < bytes.len() && bytes[i].is_ascii_alphabetic() { i += 1; }
                let word = core::str::from_utf8(&bytes[start..i]).unwrap_or("");
                match word {
                    "sin" | "Sin" | "SIN" => tokens.push(FToken::FnSin),
                    "cos" | "Cos" | "COS" => tokens.push(FToken::FnCos),
                    "tan" | "Tan" | "TAN" => tokens.push(FToken::FnTan),
                    "sqrt" | "Sqrt" | "SQRT" => tokens.push(FToken::FnSqrt),
                    "ln"  | "Ln"  | "LN"  => tokens.push(FToken::FnLn),
                    "log" | "Log" | "LOG" => tokens.push(FToken::FnLog),
                    "pi"  | "Pi"  | "PI"  => tokens.push(FToken::Pi),
                    "e"   | "E"           => tokens.push(FToken::ConstE),
                    "x"   | "X"           => tokens.push(FToken::Var),
                    _ => {}
                }
            }
            b'+' => { tokens.push(FToken::Plus);   i += 1; }
            b'-' => { tokens.push(FToken::Minus);  i += 1; }
            b'*' => { tokens.push(FToken::Star);   i += 1; }
            b'/' => { tokens.push(FToken::Slash);  i += 1; }
            b'^' => { tokens.push(FToken::Caret);  i += 1; }
            b'(' => { tokens.push(FToken::LParen); i += 1; }
            b')' => { tokens.push(FToken::RParen); i += 1; }
            _ => { i += 1; }
        }
    }
    tokens
}

#[derive(Clone)]
enum FExpr {
    Num(f64), Var, Pi, ConstE,
    Neg(Box<FExpr>),
    Add(Box<FExpr>, Box<FExpr>),
    Sub(Box<FExpr>, Box<FExpr>),
    Mul(Box<FExpr>, Box<FExpr>),
    Div(Box<FExpr>, Box<FExpr>),
    Pow(Box<FExpr>, Box<FExpr>),
    Sin(Box<FExpr>), Cos(Box<FExpr>), Tan(Box<FExpr>),
    Sqrt(Box<FExpr>), Ln(Box<FExpr>), Log(Box<FExpr>),
}

const F_PI: f64 = 3.14159265358979323846;
const F_EU: f64 = 2.71828182845904523536;

fn f_eval(e: &FExpr) -> Option<f64> {
    match e {
        FExpr::Num(n)  => Some(*n),
        FExpr::Pi      => Some(F_PI),
        FExpr::ConstE  => Some(F_EU),
        FExpr::Var     => None,
        FExpr::Neg(a)  => Some(-f_eval(a)?),
        FExpr::Add(a,b) => Some(f_eval(a)? + f_eval(b)?),
        FExpr::Sub(a,b) => Some(f_eval(a)? - f_eval(b)?),
        FExpr::Mul(a,b) => Some(f_eval(a)? * f_eval(b)?),
        FExpr::Div(a,b) => { let bv = f_eval(b)?; if libm::fabs(bv)<1e-12 {None} else {Some(f_eval(a)?/bv)} }
        FExpr::Pow(a,b) => Some(libm::pow(f_eval(a)?, f_eval(b)?)),
        FExpr::Sin(a)   => { let v=f_eval(a)?; Some(f_round_trig(libm::sin(v))) }
        FExpr::Cos(a)   => { let v=f_eval(a)?; Some(f_round_trig(libm::cos(v))) }
        FExpr::Tan(a)   => { let v=f_eval(a)?; Some(f_round_trig(libm::tan(v))) }
        FExpr::Sqrt(a)  => { let v=f_eval(a)?; if v < -1e-12 {None} else {Some(libm::sqrt(libm::fabs(v)))} }
        FExpr::Ln(a)    => { let v=f_eval(a)?; if v<=0.0{None} else {Some(libm::log(v))} }
        FExpr::Log(a)   => { let v=f_eval(a)?; if v<=0.0{None} else {Some(libm::log10(v))} }
    }
}

fn f_round_trig(v: f64) -> f64 {
    if libm::fabs(v) < 1e-10 { 0.0 }
    else if libm::fabs(v - 1.0) < 1e-10 { 1.0 }
    else if libm::fabs(v + 1.0) < 1e-10 { -1.0 }
    else { v }
}

fn f_has_var(e: &FExpr) -> bool {
    match e {
        FExpr::Var => true,
        FExpr::Num(_) | FExpr::Pi | FExpr::ConstE => false,
        FExpr::Neg(a) => f_has_var(a),
        FExpr::Add(a,b)|FExpr::Sub(a,b)|FExpr::Mul(a,b)|FExpr::Div(a,b)|FExpr::Pow(a,b) => f_has_var(a)||f_has_var(b),
        FExpr::Sin(a)|FExpr::Cos(a)|FExpr::Tan(a)|FExpr::Sqrt(a)|FExpr::Ln(a)|FExpr::Log(a) => f_has_var(a),
    }
}

fn f_fmt(e: &FExpr) -> String {
    match e {
        FExpr::Num(n)  => g_fmt_num(*n),
        FExpr::Pi      => "pi".into(),
        FExpr::ConstE  => "e".into(),
        FExpr::Var     => "x".into(),
        FExpr::Neg(a)  => { let s = f_fmt(a); format!("-({})", s) }
        FExpr::Add(a,b) => format!("{} + {}", f_fmt(a), f_fmt(b)),
        FExpr::Sub(a,b) => format!("{} - {}", f_fmt(a), f_fmt(b)),
        FExpr::Mul(a,b) => {
            let (af, bf) = (f_fmt(a), f_fmt(b));
            match (a.as_ref(), b.as_ref()) {
                (FExpr::Add(..)|FExpr::Sub(..), _) => format!("({})*{}", af, bf),
                (_, FExpr::Add(..)|FExpr::Sub(..)) => format!("{}*({})", af, bf),
                _ => format!("{}*{}", af, bf),
            }
        }
        FExpr::Div(a,b)  => format!("({})/({})", f_fmt(a), f_fmt(b)),
        FExpr::Pow(a,b)  => {
            match a.as_ref() {
                FExpr::Add(..)|FExpr::Sub(..)|FExpr::Mul(..)|FExpr::Div(..) => format!("({})^{}", f_fmt(a), f_fmt(b)),
                _ => format!("{}^{}", f_fmt(a), f_fmt(b)),
            }
        }
        FExpr::Sin(a)  => format!("sin({})", f_fmt(a)),
        FExpr::Cos(a)  => format!("cos({})", f_fmt(a)),
        FExpr::Tan(a)  => format!("tan({})", f_fmt(a)),
        FExpr::Sqrt(a) => format!("sqrt({})", f_fmt(a)),
        FExpr::Ln(a)   => format!("ln({})", f_fmt(a)),
        FExpr::Log(a)  => format!("log({})", f_fmt(a)),
    }
}

struct FParser<'a> { toks: &'a [FToken], pos: usize }

impl<'a> FParser<'a> {
    fn new(toks: &'a [FToken]) -> Self { FParser { toks, pos: 0 } }
    fn peek(&self) -> Option<&FToken> { self.toks.get(self.pos) }
    fn consume(&mut self) { self.pos += 1; }

    fn parse_expr(&mut self) -> Result<FExpr, &'static str> {
        let neg = if matches!(self.peek(), Some(FToken::Minus)) { self.consume(); true } else { false };
        let mut left = self.parse_term()?;
        if neg { left = FExpr::Neg(Box::new(left)); }
        loop {
            match self.peek() {
                Some(FToken::Plus)  => { self.consume(); let r = self.parse_term()?; left = FExpr::Add(Box::new(left), Box::new(r)); }
                Some(FToken::Minus) => { self.consume(); let r = self.parse_term()?; left = FExpr::Sub(Box::new(left), Box::new(r)); }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<FExpr, &'static str> {
        let mut left = self.parse_power()?;
        loop {
            match self.peek() {
                Some(FToken::Star)  => { self.consume(); let r = self.parse_power()?; left = FExpr::Mul(Box::new(left), Box::new(r)); }
                Some(FToken::Slash) => { self.consume(); let r = self.parse_power()?; left = FExpr::Div(Box::new(left), Box::new(r)); }
                // Implicit multiplication: number/const directly followed by function/var/paren
                Some(FToken::FnSin)|Some(FToken::FnCos)|Some(FToken::FnTan)|Some(FToken::FnSqrt)|
                Some(FToken::FnLn)|Some(FToken::FnLog)|Some(FToken::Var)|Some(FToken::LParen)|
                Some(FToken::Pi)|Some(FToken::ConstE) => {
                    match &left {
                        FExpr::Num(_)|FExpr::Pi|FExpr::ConstE => {
                            let r = self.parse_power()?;
                            left = FExpr::Mul(Box::new(left), Box::new(r));
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<FExpr, &'static str> {
        let base = self.parse_primary()?;
        if matches!(self.peek(), Some(FToken::Caret)) {
            self.consume();
            let exp = self.parse_primary()?;
            return Ok(FExpr::Pow(Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }

    fn parse_primary(&mut self) -> Result<FExpr, &'static str> {
        match self.peek().cloned() {
            Some(FToken::Num(n))  => { self.consume(); Ok(FExpr::Num(n)) }
            Some(FToken::Pi)      => { self.consume(); Ok(FExpr::Pi) }
            Some(FToken::ConstE)  => { self.consume(); Ok(FExpr::ConstE) }
            Some(FToken::Var)     => { self.consume(); Ok(FExpr::Var) }
            Some(FToken::Minus)   => { self.consume(); let i = self.parse_primary()?; Ok(FExpr::Neg(Box::new(i))) }
            Some(FToken::LParen)  => {
                self.consume();
                let inner = self.parse_expr()?;
                if !matches!(self.peek(), Some(FToken::RParen)) { return Err("Expected )"); }
                self.consume();
                // check for implicit mul: (a)(b)
                if matches!(self.peek(), Some(FToken::LParen)) {
                    self.consume();
                    let right = self.parse_expr()?;
                    if !matches!(self.peek(), Some(FToken::RParen)) { return Err("Expected )"); }
                    self.consume();
                    return Ok(FExpr::Mul(Box::new(inner), Box::new(right)));
                }
                Ok(inner)
            }
            Some(FToken::FnSin)  => { self.consume(); Ok(FExpr::Sin(Box::new(self.parse_fn_arg()?))) }
            Some(FToken::FnCos)  => { self.consume(); Ok(FExpr::Cos(Box::new(self.parse_fn_arg()?))) }
            Some(FToken::FnTan)  => { self.consume(); Ok(FExpr::Tan(Box::new(self.parse_fn_arg()?))) }
            Some(FToken::FnSqrt) => { self.consume(); Ok(FExpr::Sqrt(Box::new(self.parse_fn_arg()?))) }
            Some(FToken::FnLn)   => { self.consume(); Ok(FExpr::Ln(Box::new(self.parse_fn_arg()?))) }
            Some(FToken::FnLog)  => { self.consume(); Ok(FExpr::Log(Box::new(self.parse_fn_arg()?))) }
            _ => Err("Unexpected token"),
        }
    }

    fn parse_fn_arg(&mut self) -> Result<FExpr, &'static str> {
        if matches!(self.peek(), Some(FToken::LParen)) {
            self.consume();
            let inner = self.parse_expr()?;
            if !matches!(self.peek(), Some(FToken::RParen)) { return Err("Expected )"); }
            self.consume();
            Ok(inner)
        } else {
            self.parse_primary()
        }
    }
}

fn f_simplify(e: &FExpr, steps: &mut Vec<(String, String)>) -> FExpr {
    match e {
        FExpr::Sin(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                let r = f_round_trig(libm::sin(v));
                steps.push(("sin".into(), format!("sin({}) = {}", f_fmt(&a_s), g_fmt_num(r))));
                FExpr::Num(r)
            } else { FExpr::Sin(Box::new(a_s)) }
        }
        FExpr::Cos(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                let r = f_round_trig(libm::cos(v));
                steps.push(("cos".into(), format!("cos({}) = {}", f_fmt(&a_s), g_fmt_num(r))));
                FExpr::Num(r)
            } else { FExpr::Cos(Box::new(a_s)) }
        }
        FExpr::Tan(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                let r = f_round_trig(libm::tan(v));
                steps.push(("tan".into(), format!("tan({}) = {}", f_fmt(&a_s), g_fmt_num(r))));
                FExpr::Num(r)
            } else { FExpr::Tan(Box::new(a_s)) }
        }
        FExpr::Sqrt(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                let (coeff, rad) = simplify_sqrt_int(v);
                let display = if rad == 1 {
                    g_fmt_num(coeff as f64)
                } else if coeff == 1 {
                    format!("sqrt({})", rad)
                } else {
                    format!("{}*sqrt({})", coeff, rad)
                };
                steps.push(("sqrt".into(), format!("sqrt({}) = {}", f_fmt(&a_s), display)));
                FExpr::Num(libm::sqrt(libm::fabs(v)))
            } else { FExpr::Sqrt(Box::new(a_s)) }
        }
        FExpr::Ln(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                if v <= 0.0 { return FExpr::Ln(Box::new(a_s)); }
                let r = libm::log(v);
                let r = if libm::fabs(r - libm::round(r)) < 1e-10 { libm::round(r) } else { r };
                steps.push(("ln".into(), format!("ln({}) = {}", f_fmt(&a_s), g_fmt_num(r))));
                FExpr::Num(r)
            } else { FExpr::Ln(Box::new(a_s)) }
        }
        FExpr::Log(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) {
                if v <= 0.0 { return FExpr::Log(Box::new(a_s)); }
                let r = libm::log10(v);
                let r = if libm::fabs(r - libm::round(r)) < 1e-10 { libm::round(r) } else { r };
                steps.push(("log".into(), format!("log({}) = {}", f_fmt(&a_s), g_fmt_num(r))));
                FExpr::Num(r)
            } else { FExpr::Log(Box::new(a_s)) }
        }
        FExpr::Pi => {
            steps.push(("const".into(), format!("pi = {:.5}", F_PI)));
            FExpr::Pi
        }
        FExpr::ConstE => {
            steps.push(("const".into(), format!("e = {:.5}", F_EU)));
            FExpr::ConstE
        }
        FExpr::Neg(a) => {
            let a_s = f_simplify(a, steps);
            if let Some(v) = f_eval(&a_s) { FExpr::Num(-v) } else { FExpr::Neg(Box::new(a_s)) }
        }
        FExpr::Add(a, b) => {
            let a_s = f_simplify(a, steps);
            let b_s = f_simplify(b, steps);
            let node = FExpr::Add(Box::new(a_s.clone()), Box::new(b_s.clone()));
            if !f_has_var(&a_s) && !f_has_var(&b_s) {
                if let Some(v) = f_eval(&node) {
                    steps.push(("add".into(), format!("{} + {} = {}", f_fmt(&a_s), f_fmt(&b_s), g_fmt_num(v))));
                    return FExpr::Num(v);
                }
            }
            node
        }
        FExpr::Sub(a, b) => {
            let a_s = f_simplify(a, steps);
            let b_s = f_simplify(b, steps);
            let node = FExpr::Sub(Box::new(a_s.clone()), Box::new(b_s.clone()));
            if !f_has_var(&a_s) && !f_has_var(&b_s) {
                if let Some(v) = f_eval(&node) {
                    steps.push(("sub".into(), format!("{} - {} = {}", f_fmt(&a_s), f_fmt(&b_s), g_fmt_num(v))));
                    return FExpr::Num(v);
                }
            }
            node
        }
        FExpr::Mul(a, b) => {
            let a_s = f_simplify(a, steps);
            let b_s = f_simplify(b, steps);
            let node = FExpr::Mul(Box::new(a_s.clone()), Box::new(b_s.clone()));
            if !f_has_var(&a_s) && !f_has_var(&b_s) {
                if let Some(v) = f_eval(&node) {
                    steps.push(("mul".into(), format!("{}*{} = {}", f_fmt(&a_s), f_fmt(&b_s), g_fmt_num(v))));
                    return FExpr::Num(v);
                }
            }
            node
        }
        FExpr::Div(a, b) => {
            let a_s = f_simplify(a, steps);
            let b_s = f_simplify(b, steps);
            let node = FExpr::Div(Box::new(a_s.clone()), Box::new(b_s.clone()));
            if !f_has_var(&a_s) && !f_has_var(&b_s) {
                if let Some(v) = f_eval(&node) {
                    steps.push(("div".into(), format!("{}/{} = {}", f_fmt(&a_s), f_fmt(&b_s), g_fmt_num(v))));
                    return FExpr::Num(v);
                }
            }
            node
        }
        FExpr::Pow(a, b) => {
            let a_s = f_simplify(a, steps);
            let b_s = f_simplify(b, steps);
            let node = FExpr::Pow(Box::new(a_s.clone()), Box::new(b_s.clone()));
            if !f_has_var(&a_s) && !f_has_var(&b_s) {
                if let Some(v) = f_eval(&node) {
                    steps.push(("pow".into(), format!("{}^{} = {}", f_fmt(&a_s), f_fmt(&b_s), g_fmt_num(v))));
                    return FExpr::Num(v);
                }
            }
            node
        }
        other => other.clone(),
    }
}

/// Simplify sqrt(n) → coeff * sqrt(radicand) where radicand is square-free.
/// Returns (coeff, radicand). E.g. sqrt(72) → (6, 2), sqrt(9) → (3, 1).
fn simplify_sqrt_int(n: f64) -> (u32, u32) {
    if n < 0.0 { return (0, 0); }
    let n_int = libm::round(n) as u32;
    // Check perfect square
    let root = libm::sqrt(n);
    let root_int = libm::round(root) as u32;
    if root_int * root_int == n_int { return (root_int, 1); }
    // Find largest perfect-square factor
    let mut best_coeff = 1u32;
    let mut best_rad = n_int;
    let mut k = 2u32;
    while k * k <= n_int {
        let k2 = k * k;
        if n_int % k2 == 0 { best_coeff = k; best_rad = n_int / k2; }
        k += 1;
    }
    (best_coeff, best_rad)
}
