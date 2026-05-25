use crate::{
    program::Program,
    config::{NUM_BEST_PROG, POPULATION_SIZE},
    env::{Environment, Score},
    program::{eval_prog, mutate_program}, rand::Rng,
};

/// Population of [Program]s.
/// The population has a fixed size determined by [POPULATION_SIZE].
pub type Population = [Program; POPULATION_SIZE];

/// Population of [Program]s alongside their [Score]s.
/// The population has a fixed size determined by [POPULATION_SIZE].
pub type ScoredPopulation = [(Program, Score); POPULATION_SIZE];

/// Run the same episode for each [Program] in the _population_
/// and returns an array of the [Program]s along with their [Score]s
/// on this episode.
pub fn evaluate_population(population: &Population, env: &mut dyn Environment, rng: &mut Rng) -> ScoredPopulation {
    let mut res = population.map(|prog| (prog, 0.));
    env.seed(rng.rand_64());

    for (i, prog) in population.iter().enumerate() {
        let mut obs = env.reset();
        let mut episode_done = false;
        while !episode_done {
            let action = eval_prog(*prog, obs);
            let (new_obs, reward, done) = env.step(action);
            episode_done = done;
            res[i].1 += reward;
            obs = new_obs;
        }
    }
    res
}

/// Keep the best [Program]s (based on their [Score]) and mutate
/// them to create a new [Population].
/// The number of programms that are kept is determined by [NUM_BEST_PROG].
pub fn mutate_population(mut population: ScoredPopulation, rng: &mut Rng) -> Population {
    population.sort_by(|(_, s1), (_, s2)| f32::total_cmp(s2, s1)); // Sort in reverse order
    let mut new_population = [0; POPULATION_SIZE];
    for i in 0..NUM_BEST_PROG {
        let (prog, _) = population[i];
        new_population[i] = prog;
        for j in 1..POPULATION_SIZE / NUM_BEST_PROG {
            let new_prog = mutate_program(prog, rng);
            new_population[i + j * NUM_BEST_PROG] = new_prog;
        }
    }
    new_population
}
