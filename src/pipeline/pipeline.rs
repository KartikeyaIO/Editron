use crate::filter::{Filter, FilterVM};
use crate::media::frame::{Color, Frame, Pos};
use crate::pipeline::kernel::Kernel;

pub enum Operation {
    PointFilter(Filter),
    Convolution(Kernel),
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

        // Take a snapshot of the starting frame for convolution passes
        // so neighborhood calculations remain stable throughout the pipeline execution
        let original_frame = frame.clone();

        for y in 0..height {
            for x in 0..width {
                let pos = Pos(x, y);
                // Begin with the unedited pixel
                let mut current_color = original_frame
                    .get_pixel(&pos)
                    .unwrap_or(Color::RGB(0, 0, 0));

                for op in &self.operations {
                    match op {
                        Operation::PointFilter(filter) => {
                            let mut filtervm = FilterVM::new();
                            // Filters just modify the color point-to-point!
                            current_color = filter.apply(
                                current_color,
                                x,
                                y,
                                width,
                                height,
                                &[0.0],
                                &mut filtervm,
                            );
                        }
                        Operation::Convolution(kernel) => {
                            // Kernels require neighborhoods, so they poll the original frame snapshot
                            current_color = kernel.apply_to_pixel(x, y, &original_frame);
                        }
                    }
                }

                // Boom. One write per pixel. Iterating efficiently.
                frame
                    .set_pixel(&pos, &current_color)
                    .map_err(|_| PipelineError::PixelError)?;
            }
        }

        Ok(())
    }
}
