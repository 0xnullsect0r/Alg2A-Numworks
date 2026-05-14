pub mod linear;
pub mod quadratic;
pub mod systems;
pub mod complex_tools;
pub mod poly;
pub mod simplifier;

#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};
#[cfg(not(target_os = "none"))]
use std::{string::String, vec::Vec};

pub struct ToolResult {
    pub lines: Vec<(String, String)>,
    pub warn: Option<String>,
    pub ready: bool,
}

impl ToolResult {
    pub fn new() -> Self { ToolResult { lines: Vec::new(), warn: None, ready: false } }
    pub fn clear(&mut self) { self.lines.clear(); self.warn = None; self.ready = false; }
    pub fn add(&mut self, label: &str, value: &str) {
        self.lines.push((label.into(), value.into()));
    }
    pub fn set_warn(&mut self, w: &str) { self.warn = Some(w.into()); }
    pub fn finish(&mut self) { self.ready = true; }
}

pub fn fmt_i64_pub(n: i64) -> String { fmt_i64(n) }

pub fn fmt_f64(v: f64) -> String {
    if v == f64::INFINITY { return "inf".into(); }
    if v == f64::NEG_INFINITY { return "-inf".into(); }
    if v.is_nan() { return "?".into(); }

    let rounded = libm::round(v);
    if libm::fabs(v - rounded) < 1e-9 {
        return fmt_i64(rounded as i64);
    }

    let mut s = fmt_decimal(v, 4);
    if s.contains('.') {
        while s.ends_with('0') { s.pop(); }
        if s.ends_with('.') { s.pop(); }
    }
    s
}

fn fmt_i64(n: i64) -> String {
    if n == 0 { return "0".into(); }
    let neg = n < 0;
    let mut n = if neg { (-(n as i128)) as u64 } else { n as u64 };
    let mut digits = [0u8; 20];
    let mut len = 0;
    while n > 0 {
        digits[len] = (n % 10) as u8;
        n /= 10;
        len += 1;
    }
    let mut s = String::new();
    if neg { s.push('-'); }
    for i in (0..len).rev() { s.push((b'0' + digits[i]) as char); }
    s
}

fn fmt_decimal(v: f64, decimals: usize) -> String {
    let neg = v < 0.0;
    let v = libm::fabs(v);
    let int_part = v as u64;
    let mut frac = v - int_part as f64;

    let mut s = String::new();
    if neg { s.push('-'); }

    let int_str = fmt_i64(int_part as i64);
    s.push_str(&int_str);

    if decimals > 0 {
        s.push('.');
        for _ in 0..decimals {
            frac *= 10.0;
            let d = frac as u8;
            s.push((b'0' + d) as char);
            frac -= d as f64;
        }
    }
    s
}

pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

pub fn fmt_fraction(numer: i64, denom: i64) -> String {
    if denom == 0 { return "undef".into(); }
    if numer == 0 { return "0".into(); }
    let g = gcd(numer.abs(), denom.abs());
    let n = numer / g;
    let d = denom / g;
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    if d == 1 { return fmt_i64(n); }
    let mut s = fmt_i64(n);
    s.push('/');
    s.push_str(&fmt_i64(d));
    s
}

pub fn try_fraction(v: f64, max_denom: i64) -> String {
    if v == 0.0 { return "0".into(); }
    let neg = v < 0.0;
    let v_abs = libm::fabs(v);
    for d in 1..=max_denom {
        let n = libm::round(v_abs * d as f64) as i64;
        if libm::fabs(n as f64 / d as f64 - v_abs) < 1e-9 {
            let n = if neg { -n } else { n };
            return fmt_fraction(n, d);
        }
    }
    fmt_f64(if neg { -v_abs } else { v_abs })
}
