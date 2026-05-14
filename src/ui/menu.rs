#[cfg(target_os = "none")]
use alloc::string::String;
#[cfg(not(target_os = "none"))]
use std::string::String;

use crate::eadk::{self, Point, Rect};
use crate::constants::*;
use crate::ui::draw::*;

pub struct Menu<'a> {
    pub title: &'a str,
    pub items: &'a [&'a str],
    pub selected: usize,
}

impl<'a> Menu<'a> {
    pub fn new(title: &'a str, items: &'a [&'a str]) -> Self {
        Menu { title, items, selected: 0 }
    }

    pub fn draw(&self) {
        clear_screen();
        draw_header(self.title);
        draw_footer("Up/Dn=Navigate  EXE=Select  Back=Back");

        for (i, item) in self.items.iter().enumerate() {
            let y = CONTENT_Y + i as u16 * ROW_H;
            let bg = if i == self.selected { C_SELECTED } else { C_BG };
            eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, bg);
            let prefix = if i == self.selected { "> " } else { "  " };
            let mut text = String::from(prefix);
            text.push_str(item);
            eadk::display::draw_string(&text, Point { x: LABEL_X, y: y + 1 }, false, C_TEXT, bg);
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.items.len() { self.selected += 1; }
    }
}
