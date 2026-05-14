/// Simplifier — step-by-step algebraic simplification tools.
/// Each function shows every intermediate step and labels the rule used.

#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

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
