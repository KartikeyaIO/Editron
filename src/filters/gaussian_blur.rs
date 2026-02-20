use std::fmt::Debug;

use crate::{filter::Filter, media::frame::Frame};
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
    fn apply(&self, frame: Frame) -> Frame {
        let w = &frame.width();
        let h = &frame.height();
        let fmt = &frame.format();
        let data = &mut frame.data_mut();
    }
}
