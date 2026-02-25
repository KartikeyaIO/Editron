use crate::{
    filter::Filter,
    media::frame::{Color, Frame, Pos},
};
pub struct GaussianBlur {
    pub sigma: f32,
    kernel: Vec<f32>,
}

impl GaussianBlur {
    pub fn build_kernel(sigma: &f32) -> Vec<f32> {
        if *sigma <= 0.0 {
            return vec![1.0];
        }
        let radius = (3.0 * sigma).ceil() as i32;
        let mut sum = 0.0;
        let mut kernel: Vec<f32> = Vec::new();
        for i in -radius..=radius {
            let weight = (-(i * i) as f32 / (2.0 * sigma * sigma)).exp();
            sum += weight;
            kernel.push(weight);
        }
        for k in &mut kernel {
            *k /= sum;
        }
        kernel
    }
    pub fn new(sigma: f32) -> Self {
        let kernel = GaussianBlur::build_kernel(&sigma);
        Self { sigma, kernel }
    }
}

impl Filter for GaussianBlur {
    fn apply(&self, mut frame: Frame) -> Frame {
        let w = *(&frame.width());
        let h = *(&frame.height());
        let kernel = &self.kernel;
        let radius = (kernel.len() / 2) as i32;
        let temp = frame.clone();
        for y in 0..h {
            for x in 0..w {
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;

                for k in 0..kernel.len() {
                    let offset = k as i32 - radius;
                    let nx = (x as i32 + offset).clamp(0, (w - 1) as i32) as u32;
                    let pos = Pos(nx, y);
                    let Color(r, g, b) = temp.get_pixel(&pos).unwrap();
                    r_sum += r as f32 * kernel[k];
                    g_sum += g as f32 * kernel[k];
                    b_sum += b as f32 * kernel[k];
                }
                let new = Color(
                    r_sum.clamp(0.0, 255.0) as u8,
                    g_sum.clamp(0.0, 255.0) as u8,
                    b_sum.clamp(0.0, 255.0) as u8,
                );
                frame.set_pixel(&Pos(x, y), &new).unwrap();
            }
        }
        for y in 0..h {
            for x in 0..w {
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;

                for k in 0..kernel.len() {
                    let offset = k as i32 - radius;
                    let ny = (y as i32 + offset).clamp(0, (h - 1) as i32) as u32;

                    let pos = Pos(x, ny);
                    let Color(r, g, b) = temp.get_pixel(&pos).unwrap();

                    r_sum += r as f32 * kernel[k];
                    g_sum += g as f32 * kernel[k];
                    b_sum += b as f32 * kernel[k];
                }

                let new_color = Color(
                    r_sum.clamp(0.0, 255.0) as u8,
                    g_sum.clamp(0.0, 255.0) as u8,
                    b_sum.clamp(0.0, 255.0) as u8,
                );

                frame.set_pixel(&Pos(x, y), &new_color).unwrap();
            }
        }
        frame
    }
}
