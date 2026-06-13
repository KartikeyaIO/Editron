use crate::filter::{Filter, FilterVM};
use crate::media::frame::{Color, Frame, Pos};
use crate::pipeline::kernel::Kernel;
use crate::range::Mask;

pub enum Operation {
    PointFilter {
        filter: Filter,
        params: Vec<f32>,
        mask: Option<Mask>,
    },

    Convolution {
        kernel: Kernel,
        mask: Option<Mask>,
    },
}

// ── Unified Pipeline ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PipelineError {
    InvalidData,
    PixelError,
}

pub trait Pipeline {
    /// Runs all operations on a frame in a consolidated pass.
    fn execute(&self, frame: &mut Frame) -> Result<(), PipelineError>;
}

pub struct EffectPipeline {
    pub operations: Vec<Operation>,
}

impl Pipeline for EffectPipeline {
    fn execute(&self, frame: &mut Frame) -> Result<(), PipelineError> {
        let width = frame.width();
        let height = frame.height();

        for operation in &self.operations {
            match operation {
                Operation::PointFilter {
                    filter,
                    params,
                    mask,
                } => {
                    let mut vm = FilterVM::new();

                    for y in 0..height {
                        for x in 0..width {
                            if let Some(mask) = mask {
                                if !mask.contains(x as usize, y as usize) {
                                    continue;
                                }
                            }

                            let pos = Pos(x, y);

                            let color = frame.get_pixel(&pos).unwrap_or(Color::RGB(0, 0, 0));

                            let result = filter.apply(color, x, y, width, height, params, &mut vm);

                            frame
                                .set_pixel(&pos, &result)
                                .map_err(|_| PipelineError::PixelError)?;
                        }
                    }
                }

                Operation::Convolution { kernel, mask } => {
                    // Snapshot BEFORE this kernel pass
                    let snapshot = frame.clone();

                    for y in 0..height {
                        for x in 0..width {
                            if let Some(mask) = mask {
                                if !mask.contains(x as usize, y as usize) {
                                    continue;
                                }
                            }

                            let pos = Pos(x, y);

                            let result = kernel.apply_to_pixel(x, y, &snapshot);

                            frame
                                .set_pixel(&pos, &result)
                                .map_err(|_| PipelineError::PixelError)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
