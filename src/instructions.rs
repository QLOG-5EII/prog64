/// An [Instruction] takes two input to compute an output
pub type Instruction = fn(f32, f32) -> f32;

/// Array of available [Instruction]s
/// - 0 (000): Addition: x = x + y
/// - 1 (001): Substraction: x = x - y
/// - 2 (010): Multiplication: x = x * y
/// - 3 (011): Division: x = x / y (0 if y == 0)
/// - 4 (100): Maximum: x = max(x, y)
/// - 5 (101): Load: x = y (used to load an input to a register)
/// - 6 (110): Absolute value: x = |y|
/// - 7 (111): Modulo: x % y (0 if y == 0)
pub const INSTRUCTIONS: [Instruction; 8] = [add, sub, div, mul, abs, modu, max, ld];

/// Array of [Instruction]s names. Used for printing [Progam]s
pub const INSTRUCTIONS_NAME: [&str; 8] = ["add", "sub", "div", "mul", "abs", "mod", "max", "ld"];

fn add(x: f32, y: f32) -> f32 {
    x + y
}
fn sub(x: f32, y: f32) -> f32 {
    x - y
}
fn mul(x: f32, y: f32) -> f32 {
    x * y
}
fn div(x: f32, y: f32) -> f32 {
    if y != 0. { x / y } else { 0. }
}
fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}
fn ld(_: f32, y: f32) -> f32 {
    y
}
fn abs(_: f32, y: f32) -> f32 {
    y.abs()
}
fn modu(x: f32, y: f32) -> f32 {
    if y == 0. {
        return 0.;
    }
    x % y
}
