use crate::eadk::{self, Color, Point, Rect};
use crate::constants::*;

pub fn clear_screen() {
    eadk::display::push_rect_uniform(eadk::SCREEN_RECT, C_BG);
}

pub fn draw_header(title: &str) {
    eadk::display::push_rect_uniform(Rect { x: 0, y: 0, width: W, height: HEADER_H }, C_HEADER);
    eadk::display::draw_string(title, Point { x: 5, y: 3 }, false, C_HEADER_TXT, C_HEADER);
}

pub fn draw_footer(hint: &str) {
    eadk::display::push_rect_uniform(Rect { x: 0, y: FOOTER_Y, width: W, height: FOOTER_H }, C_FOOTER);
    eadk::display::draw_string(hint, Point { x: 5, y: FOOTER_Y + 1 }, false, C_FOOTER_TXT, C_FOOTER);
}

pub fn draw_sep(y: u16) {
    eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: 1 }, C_SEP);
}

pub fn draw_line(y: u16, text: &str, fg: Color, bg: Color) {
    eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, bg);
    eadk::display::draw_string(text, Point { x: LABEL_X, y: y + 1 }, false, fg, bg);
}

pub fn draw_input_field(y: u16, label: &str, value: &str, active: bool) {
    let bg = C_BG;
    let field_bg = if active { C_HIGHLIGHT } else { C_INPUT_BG };

    eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, bg);
    eadk::display::draw_string(label, Point { x: LABEL_X, y: y + 1 }, false, C_TEXT, bg);

    eadk::display::push_rect_uniform(Rect { x: INPUT_X, y: y + 1, width: INPUT_W, height: ROW_H - 2 }, field_bg);

    let display_val = if value.is_empty() { "0" } else { value };
    let vlen = display_val.len().min(13) as u16;
    let vx = if INPUT_X + INPUT_W > vlen * SMALL_CHAR_W + 2 {
        INPUT_X + INPUT_W - vlen * SMALL_CHAR_W - 2
    } else {
        INPUT_X
    };
    eadk::display::draw_string(display_val, Point { x: vx, y: y + 2 }, false, C_TEXT, field_bg);

    if active {
        let cursor_x = INPUT_X + INPUT_W - 2;
        eadk::display::push_rect_uniform(Rect { x: cursor_x, y: y + 3, width: 1, height: ROW_H - 8 }, C_TEXT);
    }
}

pub fn draw_result_line(y: u16, label: &str, value: &str) {
    let bg = C_BG;
    eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, bg);
    eadk::display::draw_string(label, Point { x: LABEL_X, y: y + 1 }, false, C_DIM, bg);
    eadk::display::draw_string(value, Point { x: 110, y: y + 1 }, false, C_RESULT, bg);
}

pub fn draw_warn_line(y: u16, text: &str) {
    let bg = C_BG;
    eadk::display::push_rect_uniform(Rect { x: 0, y, width: W, height: ROW_H }, bg);
    eadk::display::draw_string(text, Point { x: LABEL_X, y: y + 1 }, false, C_WARN, bg);
}
