use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

fn main() {
    let seed = 1;
    let mut rng = PCGRng::seed(seed);
    let mut env = CartPole::default();
    let mut total_reward = 0.0;

    loop {
        let action = (rng.next_u32() % 4) as f32;
        let (_, reward, done) = env.step(action, &mut rng);

        total_reward += reward;
        if done {
            break;
        }
    }

    println!("Total reward: {}", total_reward)
}
