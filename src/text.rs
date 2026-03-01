use fontdue::{self, Font};

use crate::media::frame::{Color, Frame, Pos};
pub struct Text {
    data: String,
    font: Font,
    size: f32,
    pos: Pos,
    color: Color,
}

pub enum TextError {
    FontNotFound,
}
impl Text {
    pub fn new(data: &str, font: Font, size: f32, pos: Pos, color: Color) -> Self {
        Self {
            data: data.to_string(),
            font,
            size,
            pos,
            color,
        }
    }
    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }
    pub fn set_position(&mut self, pos: &Pos) {
        self.pos = *pos;
    }
    pub fn set_size(&mut self, size: &f32) {
        self.size = *size;
    }
    pub fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }
    pub fn set_text(&mut self, content: &str) {
        self.data = content.to_string();
    }
}
