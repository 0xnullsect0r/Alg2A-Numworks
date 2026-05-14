use crate::eadk::Color;

// Screen dimensions
pub const W: u16 = 320;
pub const H: u16 = 240;

// Layout
pub const HEADER_H: u16 = 22;
pub const FOOTER_H: u16 = 18;
pub const CONTENT_Y: u16 = HEADER_H;
pub const CONTENT_H: u16 = H - HEADER_H - FOOTER_H;
pub const FOOTER_Y: u16 = H - FOOTER_H;

// Row heights
pub const ROW_H: u16 = 18;
pub const SMALL_CHAR_W: u16 = 7;
pub const LARGE_CHAR_W: u16 = 11;

// Colors
pub const C_BG: Color = Color::from_888(255, 255, 255);
pub const C_HEADER: Color = Color::from_888(25, 50, 120);
pub const C_HEADER_TXT: Color = Color::from_888(255, 255, 255);
pub const C_SELECTED: Color = Color::from_888(200, 220, 255);
pub const C_HIGHLIGHT: Color = Color::from_888(180, 210, 250);
pub const C_INPUT_BG: Color = Color::from_888(240, 240, 240);
pub const C_TEXT: Color = Color::from_888(0, 0, 0);
pub const C_DIM: Color = Color::from_888(100, 100, 100);
pub const C_RESULT: Color = Color::from_888(0, 100, 0);
pub const C_WARN: Color = Color::from_888(180, 60, 0);
pub const C_FOOTER: Color = Color::from_888(230, 230, 230);
pub const C_FOOTER_TXT: Color = Color::from_888(80, 80, 80);
pub const C_SEP: Color = Color::from_888(180, 180, 180);

// Input field dimensions
pub const INPUT_X: u16 = 100;
pub const INPUT_W: u16 = 100;
pub const LABEL_X: u16 = 5;
