use fontdue::{self, Font};

use crate::media::frame::{Color, Frame, PixelFormat, Pos};
pub struct Text {
    data: String,
    font: Font,
    size: f32,
    pos: Pos,
    color: Color,
    format: PixelFormat,
}
#[derive(Debug)]
pub enum TextError {
    FontNotFound,
    FrameCreationFailed,
    InvalidDimensions,
    EmptyText,
    UnsupportedPixelFormat,
}
impl Text {
    pub fn new(
        data: &str,
        font: Font,
        size: f32,
        pos: Pos,
        color: Color,
        format: PixelFormat,
    ) -> Self {
        Self {
            data: data.to_string(),
            font,
            size,
            pos,
            color,
            format,
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
    pub fn picturize(&self) -> Result<Frame, TextError> {
        let font = &self.font;
        let data = &self.data;
        let size = self.size;
        let format = self.format;
        let color = self.color;

        if data.is_empty() {
            return Err(TextError::EmptyText);
        }

        let mut pen_x = 0.0;

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut max_top = f32::MIN;
        let mut min_bottom = f32::MAX;

        for ch in data.chars() {
            let (metrics, _) = font.rasterize(ch, size);

            let gx_min = pen_x + metrics.xmin as f32;
            let gx_max = gx_min + metrics.width as f32;

            let gy_min = metrics.ymin as f32;
            let gy_max = gy_min + metrics.height as f32;

            min_x = min_x.min(gx_min);
            max_x = max_x.max(gx_max);
            min_y = min_y.min(gy_min);
            max_y = max_y.max(gy_max);
            let top = metrics.ymin as f32 + metrics.height as f32;
            let bottom = metrics.ymin as f32;
            max_top = max_top.max(top);
            min_bottom = min_bottom.min(bottom);

            pen_x += metrics.advance_width;
        }

        let width = (max_x - min_x).ceil() as u32;
        let height = (max_top - min_bottom).ceil() as u32;
        let baseline = max_top;
        if width == 0 || height == 0 {
            return Err(TextError::InvalidDimensions);
        }

        let bpp = format.bytes_per_pixel();
        let mut buffer = vec![0u8; width as usize * height as usize * bpp];

        let mut pen_x = 0.0;

        for ch in data.chars() {
            let (metrics, bitmap) = font.rasterize(ch, size);

            let glyph_w = metrics.width as u32;
            let glyph_h = metrics.height as u32;

            let x_off = (pen_x + metrics.xmin as f32 - min_x).round() as i32;
            let y_off = (baseline - (metrics.ymin as f32 + metrics.height as f32)).round() as i32;

            for row in 0..glyph_h {
                for col in 0..glyph_w {
                    let src_index = (row * glyph_w + col) as usize;
                    let coverage = bitmap[src_index];

                    if coverage == 0 {
                        continue;
                    }

                    let dx = x_off + col as i32;
                    let dy = y_off + row as i32;

                    if dx < 0 || dy < 0 || dx >= width as i32 || dy >= height as i32 {
                        continue;
                    }

                    let dst_index = (dy as usize * width as usize + dx as usize) * bpp;

                    match format {
                        PixelFormat::Gray8 => {
                            buffer[dst_index] = buffer[dst_index].max(coverage);
                        }

                        PixelFormat::RGB24 => {
                            let alpha = coverage as f32 / 255.0;

                            buffer[dst_index] = (color.r() as f32 * alpha) as u8;
                            buffer[dst_index + 1] = (color.g() as f32 * alpha) as u8;
                            buffer[dst_index + 2] = (color.b() as f32 * alpha) as u8;
                        }

                        PixelFormat::RGBA32 => {
                            let alpha = coverage as f32 / 255.0;

                            buffer[dst_index] = (color.r() as f32 * alpha) as u8;
                            buffer[dst_index + 1] = (color.g() as f32 * alpha) as u8;
                            buffer[dst_index + 2] = (color.b() as f32 * alpha) as u8;
                            buffer[dst_index + 3] = coverage;
                        }
                    }
                }
            }

            pen_x += metrics.advance_width;
        }

        Frame::new(width, height, format, buffer).map_err(|_| TextError::FrameCreationFailed)
    }
}
