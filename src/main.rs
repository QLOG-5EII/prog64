use prog64::{
    config::POPULATION_SIZE,
    env::{cartpole::CartPole},
    population::{evaluate_population, mutate_population},
    program::{print_program}, rand::{Rng},
};

fn main() {
    let seed = 3;
    let mut rng = Rng::seed(seed);
    let mut env = CartPole::default();
    let mut population = [0u64; POPULATION_SIZE].map(|_| rng.rand_64());
    for _ in 0..100 {
        let scored_population = evaluate_population(&population, &mut env, &mut rng);
        population = mutate_population(scored_population, &mut rng);
    }
    let scored_population = evaluate_population(&population, &mut env, &mut rng);
    println!("Best score: {}", scored_population[0].1);
    print_program(scored_population[0].0);
}
