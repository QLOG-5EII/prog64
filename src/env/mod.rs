pub mod blackjack;
pub mod cartpole;

/// The [Score] is the value representing the performances of
/// a [Program] on an episode (or various episodes).
pub type Score = f32;

pub type State = [f32; 4];

pub trait Environment {
    fn step(&mut self, action: f32) -> (State, Score, bool);
    fn reset(&mut self) -> State;
    fn seed(&mut self, seed: u64);
}
