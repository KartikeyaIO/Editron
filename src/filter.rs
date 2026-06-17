use crate::media::frame::Color;

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,

    // Parameter names in declaration order
    pub params: Vec<String>,

    // Compiled programs
    pub r_program: Vec<Instruction>,
    pub g_program: Vec<Instruction>,
    pub b_program: Vec<Instruction>,
    pub a_program: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // ─────────────────────────────
    // Channel Access
    // ─────────────────────────────
    LoadR,
    LoadG,
    LoadB,
    LoadA,

    // ─────────────────────────────
    // Filter Parameters
    // brightness(amount)
    // amount -> LoadParam(0)
    // ─────────────────────────────
    LoadParam(usize),

    // ─────────────────────────────
    // Frame Metadata
    // ─────────────────────────────
    LoadX,
    LoadY,
    LoadWidth,
    LoadHeight,

    // ─────────────────────────────
    // Constants
    // ─────────────────────────────
    PushInt(i64),
    PushFloat(f32),

    // ─────────────────────────────
    // Arithmetic
    // ─────────────────────────────
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // ─────────────────────────────
    // Unary
    // ─────────────────────────────
    Neg,

    // ─────────────────────────────
    // Comparison
    // Result: 1.0 or 0.0
    // ─────────────────────────────
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,

    // ─────────────────────────────
    // Logic
    // ─────────────────────────────
    And,
    Or,
    Not,

    // ─────────────────────────────
    // Math
    // ─────────────────────────────
    Abs,
    Min,
    Max,
    Clamp,

    Sin,
    Cos,
    Tan,

    Asin,
    Acos,
    Atan,

    Sqrt,
    Exp,
    Log,
    Log10,

    Floor,
    Ceil,
    Round,
    StoreLocal(usize),
    LoadLocal(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct PixelContext {
    pub color: Color,

    pub x: u32,
    pub y: u32,

    pub width: u32,
    pub height: u32,
}

pub struct FilterVM {
    stack: Vec<f32>,
    locals: Vec<f32>,
}

impl FilterVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(64),
            locals: vec![0.0; 16],
        }
    }

    fn pop(&mut self) -> f32 {
        self.stack.pop().unwrap_or(0.0)
    }

    fn push(&mut self, value: f32) {
        self.stack.push(value);
    }

    pub fn execute(&mut self, program: &[Instruction], ctx: &PixelContext, params: &[f32]) -> f32 {
        self.stack.clear();

        for instruction in program {
            match instruction {
                // -------------------------
                // CHANNELS
                // -------------------------
                Instruction::LoadR => {
                    self.push(ctx.color.r() as f32);
                }

                Instruction::LoadG => {
                    self.push(ctx.color.g() as f32);
                }

                Instruction::LoadB => {
                    self.push(ctx.color.b() as f32);
                }

                Instruction::LoadA => {
                    let alpha = match ctx.color {
                        Color::RGBA(_, _, _, a) => a,
                        _ => 255,
                    };

                    self.push(alpha as f32);
                }

                // -------------------------
                // PARAMS
                // -------------------------
                Instruction::LoadParam(index) => {
                    self.push(params.get(*index).copied().unwrap_or(0.0));
                }

                // -------------------------
                // PIXEL POSITION
                // -------------------------
                Instruction::LoadX => {
                    self.push(ctx.x as f32);
                }

                Instruction::LoadY => {
                    self.push(ctx.y as f32);
                }

                Instruction::LoadWidth => {
                    self.push(ctx.width as f32);
                }

                Instruction::LoadHeight => {
                    self.push(ctx.height as f32);
                }

                // -------------------------
                // CONSTANTS
                // -------------------------
                Instruction::PushInt(v) => {
                    self.push(*v as f32);
                }

                Instruction::PushFloat(v) => {
                    self.push(*v);
                }

                // -------------------------
                // ARITHMETIC
                // -------------------------
                Instruction::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }

                Instruction::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }

                Instruction::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a * b);
                }

                Instruction::Div => {
                    let b = self.pop();

                    if b == 0.0 {
                        self.push(0.0);
                    } else {
                        let a = self.pop();
                        self.push(a / b);
                    }
                }

                Instruction::Mod => {
                    let b = self.pop();

                    if b == 0.0 {
                        self.push(0.0);
                    } else {
                        let a = self.pop();
                        self.push(a % b);
                    }
                }

                Instruction::Pow => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(a.powf(b));
                }

                // -------------------------
                // UNARY
                // -------------------------
                Instruction::Neg => {
                    let a = self.pop();
                    self.push(-a);
                }

                Instruction::Abs => {
                    let a = self.pop();
                    self.push(a.abs());
                }

                // -------------------------
                // COMPARISON
                // -------------------------
                Instruction::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a == b) as u8 as f32);
                }

                Instruction::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a != b) as u8 as f32);
                }

                Instruction::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a > b) as u8 as f32);
                }

                Instruction::Ge => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a >= b) as u8 as f32);
                }

                Instruction::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a < b) as u8 as f32);
                }

                Instruction::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push((a <= b) as u8 as f32);
                }

                // -------------------------
                // LOGIC
                // -------------------------
                Instruction::And => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(((a != 0.0) && (b != 0.0)) as u8 as f32);
                }

                Instruction::Or => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(((a != 0.0) || (b != 0.0)) as u8 as f32);
                }

                Instruction::Not => {
                    let a = self.pop();

                    self.push((a == 0.0) as u8 as f32);
                }

                // -------------------------
                // MIN/MAX
                // -------------------------
                Instruction::Min => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(a.min(b));
                }

                Instruction::Max => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(a.max(b));
                }

                Instruction::Clamp => {
                    let max = self.pop();
                    let min = self.pop();
                    let value = self.pop();

                    self.push(value.clamp(min, max));
                }

                // -------------------------
                // TRIG
                // -------------------------
                Instruction::Sin => {
                    let x = { self.pop().sin() };
                    self.push(x);
                }

                Instruction::Cos => {
                    let x = { self.pop().cos() };
                    self.push(x);
                }

                Instruction::Tan => {
                    let x = { self.pop().tan() };
                    self.push(x);
                }

                Instruction::Asin => {
                    let x = { self.pop().asin() };
                    self.push(x);
                }

                Instruction::Acos => {
                    let x = { self.pop().acos() };
                    self.push(x);
                }

                Instruction::Atan => {
                    let x = { self.pop().atan() };
                    self.push(x);
                }

                // -------------------------
                // EXPONENTIAL
                // -------------------------
                Instruction::Sqrt => {
                    let x = { self.pop().sqrt() };
                    self.push(x);
                }

                Instruction::Exp => {
                    let x = { self.pop().exp() };
                    self.push(x);
                }

                Instruction::Log => {
                    let x = { self.pop().ln() };
                    self.push(x);
                }

                Instruction::Log10 => {
                    let x = { self.pop().log10() };
                    self.push(x);
                }

                // -------------------------
                // ROUNDING
                // -------------------------
                Instruction::Floor => {
                    let x = { self.pop().floor() };
                    self.push(x);
                }

                Instruction::Ceil => {
                    let x = { self.pop().ceil() };
                    self.push(x);
                }

                Instruction::Round => {
                    let x = { self.pop().round() };
                    self.push(x);
                }
                Instruction::StoreLocal(index) => {
                    let val = self.pop();
                    if *index >= self.locals.len() {
                        self.locals.resize(*index + 1, 0.0);
                    }
                    self.locals[*index] = val;
                }
                Instruction::LoadLocal(index) => {
                    let val = self.locals.get(*index).copied().unwrap_or(0.0);
                    self.push(val);
                }
            }
        }

        self.pop()
    }
}

impl Filter {
    pub fn apply(
        &self,
        color: Color,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        params: &[f32],
        vm: &mut FilterVM,
    ) -> Color {
        let ctx = PixelContext {
            color,
            x,
            y,
            width,
            height,
        };

        let r = vm.execute(&self.r_program, &ctx, params).clamp(0.0, 255.0) as u8;

        let g = vm.execute(&self.g_program, &ctx, params).clamp(0.0, 255.0) as u8;

        let b = vm.execute(&self.b_program, &ctx, params).clamp(0.0, 255.0) as u8;

        match color {
            Color::RGB(_, _, _) => Color::RGB(r, g, b).to_rgba(),

            Color::RGBA(_, _, _, _) => {
                let a = vm.execute(&self.a_program, &ctx, params).clamp(0.0, 255.0) as u8;

                Color::RGBA(r, g, b, a)
            }

            Color::Gray(_) => Color::Gray(r).to_rgba(),
        }
    }
}
