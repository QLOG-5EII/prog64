use crate::{population::Population, program::Program};

/// Size of the [Population].
pub const POPULATION_SIZE: usize = 1000;

/// Number of line in a [Program]. This number
/// is fixed such that no [Program] can have fewer``
/// or more lines.
pub const PROG_LINES: usize = 6;

/// Number of [Program]s to be mutated.
/// The reste ([POPULATION_SIZE] - [NUM_BEST_PROG])
/// is discarded during mutation.
/// For simplicity, this number must be a divider of
/// [POPULATION_SIZE].
pub const NUM_BEST_PROG: usize = 250;
