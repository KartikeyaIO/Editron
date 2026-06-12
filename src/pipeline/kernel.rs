use crate::media::frame::{Color, Frame, Pos};

// ── Kernel ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Kernel {
    pub name: String,
    pub matrix: Vec<f32>,
    pub size: usize,  // e.g., 3 means a 3x3 matrix
    pub divisor: f32, // Divisor to normalize weights
}

impl Kernel {
    /// Applies a spatial convolution around a specific (x, y) coordinate.
    pub fn apply_to_pixel(&self, x: u32, y: u32, original_frame: &Frame) -> Color {
        let half = (self.size / 2) as i32;
        let mut r_sum = 0.0;
        let mut g_sum = 0.0;
        let mut b_sum = 0.0;

        let width = original_frame.width() as i32;
        let height = original_frame.height() as i32;

        for ky in 0..self.size as i32 {
            for kx in 0..self.size as i32 {
                // Clamp coordinates to edges to avoid boundary out-of-bounds
                let px = (x as i32 + kx - half).clamp(0, width - 1) as u32;
                let py = (y as i32 + ky - half).clamp(0, height - 1) as u32;

                let weight = self.matrix[(ky * self.size as i32 + kx) as usize];

                if let Ok(color) = original_frame.get_pixel(&Pos(px, py)) {
                    r_sum += color.r() as f32 * weight;
                    g_sum += color.g() as f32 * weight;
                    b_sum += color.b() as f32 * weight;
                }
            }
        }

        let div = if self.divisor == 0.0 {
            1.0
        } else {
            self.divisor
        };

        Color::RGB(
            (r_sum / div).clamp(0.0, 255.0) as u8,
            (g_sum / div).clamp(0.0, 255.0) as u8,
            (b_sum / div).clamp(0.0, 255.0) as u8,
        )
    }
}
