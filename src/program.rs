use crate::{
    config::PROG_LINES,
    env::State,
    instructions::{INSTRUCTIONS, INSTRUCTIONS_NAME},
    rand::Rng,
};

/// A [Program] consists on 8 instructions,
/// each coded on 8 bits.
///
/// - 3 bits for the [Instruction] index (see [OPERATORS]).
/// - 2 bits for the first operand ref.
/// - 1 bit to designate weither the second operand points
///   an input (1) or a register (0).
/// - 2 bits for the second operand ref.
///
/// **Note** : The value returned by the operator is stored in
/// the first operand. Hence the first operand can only reference
/// registers (as inputs are immutable).
pub type Program = u64;

/// Bit mask of the instruction
const INS_M: u64 = 0b00000111;
/// Bit mask of the first operand
const OP1_M: u64 = 0b00011000;
/// Bit mask of the kind of the second operand
/// - if 0 then register
/// - if 1 then input
const O2K_M: u64 = 0b00100000;
/// Bit mask of the second operand
const OP2_M: u64 = 0b11000000;
/// Bit mask of a complete line
const LINE_M: u64 = 0b11111111;

/// Evaluate a [Program].
pub fn eval_prog(prog: Program, inp: State) -> f32 {
    let mut reg = [0., 0., 0., 0.];
    for line_num in 0..PROG_LINES {
        let line = (prog >> (8 * line_num)) & LINE_M;

        let i_ins = (line & INS_M) as usize;
        let i_op1 = ((line & OP1_M) >> 4) as usize;
        let i_op2 = ((line & OP2_M) >> 6) as usize;

        let op1 = reg[i_op2];
        let op2 = if line & O2K_M != 0 {
            inp[i_op2]
        } else {
            reg[i_op2]
        };

        let instruction = INSTRUCTIONS[i_ins];
        reg[i_op1] = instruction(op1, op2);
    }
    reg[0]
}

/// Generate a random bit mask with around 6.25% of bits being ones.
fn generate_mutation_mask(rng: &mut Rng) -> u64 {
    rng.rand_64() & rng.rand_64() & rng.rand_64() & rng.rand_64()
}

/// Mutate a [Program] by randomly flipping bits.
/// The mutation has 6.25% chance to flip each bits.
pub fn mutate_program(program: Program, rng: &mut Rng) -> Program {
    let mask = generate_mutation_mask(rng);
    // Xor is used to flip some bits in the programm
    program ^ mask
}

/// Prints a [Program] in the terminal (for debug).
pub fn print_program(mut prog: Program) {
    println!("===============");
    for _ in 0..PROG_LINES {
        // TODO
    }
    println!("===============");
}
