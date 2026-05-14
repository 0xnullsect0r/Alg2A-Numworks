pub struct InputBuffer {
    pub buf: [u8; 16],
    pub len: usize,
}

impl Default for InputBuffer {
    fn default() -> Self { InputBuffer::new() }
}

impl InputBuffer {
    pub fn new() -> Self { InputBuffer { buf: [0u8; 16], len: 0 } }

    pub fn push_digit(&mut self, d: u8) {
        if self.len < 15 {
            self.buf[self.len] = b'0' + d;
            self.len += 1;
        }
    }

    pub fn push_dot(&mut self) {
        if self.as_str().contains('.') { return; }
        if self.len < 15 {
            self.buf[self.len] = b'.';
            self.len += 1;
        }
    }

    pub fn toggle_sign(&mut self) {
        if self.len == 0 {
            self.buf[0] = b'-';
            self.len = 1;
        } else if self.buf[0] == b'-' {
            for i in 0..self.len - 1 { self.buf[i] = self.buf[i + 1]; }
            self.len -= 1;
        } else if self.len < 15 {
            for i in (0..self.len).rev() { self.buf[i + 1] = self.buf[i]; }
            self.buf[0] = b'-';
            self.len += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.len > 0 { self.len -= 1; }
    }

    pub fn clear(&mut self) { self.len = 0; }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("0")
    }

    pub fn parse(&self) -> Option<f64> {
        let s = self.as_str();
        if s.is_empty() || s == "-" { return Some(0.0); }
        parse_f64(s)
    }
}

pub fn parse_f64(s: &str) -> Option<f64> {
    if s.is_empty() { return Some(0.0); }
    let (neg, s) = if s.starts_with('-') { (true, &s[1..]) } else { (false, s) };
    if s.is_empty() { return Some(0.0); }

    let (int_part, frac_part) = if let Some(dot) = s.find('.') {
        (&s[..dot], &s[dot+1..])
    } else {
        (s, "")
    };

    let mut result: f64 = 0.0;
    for c in int_part.chars() {
        let d = c as u8;
        if d < b'0' || d > b'9' { return None; }
        result = result * 10.0 + (d - b'0') as f64;
    }

    let mut frac_mult = 0.1f64;
    for c in frac_part.chars() {
        let d = c as u8;
        if d < b'0' || d > b'9' { return None; }
        result += (d - b'0') as f64 * frac_mult;
        frac_mult *= 0.1;
    }

    Some(if neg { -result } else { result })
}
