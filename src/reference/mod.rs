pub mod cards;

#[cfg(target_os = "none")]
use alloc::string::ToString;
#[cfg(not(target_os = "none"))]
use std::string::{String, ToString};

use crate::eadk::{self, Point, Rect};
use crate::constants::*;
use crate::ui::draw::*;
use crate::tools::fmt_f64;

pub fn draw_ref_card(title: &str, lines: &[&str], scroll: usize) {
    clear_screen();
    draw_header(title);
    draw_footer("Up/Dn=Scroll  Back=Back");

    let max_lines = (CONTENT_H / ROW_H) as usize;
    let start = if lines.is_empty() { 0 } else { scroll.min(lines.len().saturating_sub(1)) };
    let end = (start + max_lines).min(lines.len());

    for (i, line) in lines[start..end].iter().enumerate() {
        let y = CONTENT_Y + i as u16 * ROW_H;
        eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, C_BG);
        eadk::display::draw_string(line, Point { x: LABEL_X, y: y + 1 }, false, C_TEXT, C_BG);
    }

    if lines.len() > max_lines {
        let pct = (scroll * 100) / (lines.len() - max_lines + 1);
        let mut ind = "-- ".to_string();
        ind.push_str(&fmt_f64(pct as f64));
        ind.push_str("% --");
        let ind_y = FOOTER_Y - ROW_H;
        eadk::display::push_rect_uniform(Rect { x: 0, y: ind_y, width: W, height: ROW_H }, C_FOOTER);
        eadk::display::draw_string(&ind, Point { x: 5, y: ind_y + 1 }, false, C_FOOTER_TXT, C_FOOTER);
    }
}
