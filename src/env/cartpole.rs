use std::f32::consts::PI;

use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
};
use rand_chacha::ChaCha8Rng;

use crate::env::{Environment, Score, State};

pub struct CartPole {
    /// Gravity force
    g: f32,
    /// Cart mass
    mass_cart: f32,
    /// Pole mass
    mass_pole: f32,
    /// Half length of the pole
    half_length: f32,
    /// Seconds between state update
    tau: f32,
    force_mag: f32,

    // State
    x: f32,
    x_dot: f32,
    theta: f32,
    theta_dot: f32,

    // Threshold indicating when to stop the episode
    threshold_x: f32,
    threshold_theta: f32,
    episode_length: usize,
    episode_done: bool,

    // RNG
    seed: Option<u64>,
}

impl Environment for CartPole {
    fn step(&mut self, action: f32) -> (State, Score, bool) {
        if self.episode_done {
            return ([self.x, self.x_dot, self.theta, self.theta_dot], 0., true);
        }

        let force = if action > 0. {
            self.force_mag
        } else {
            -self.force_mag
        };

        let sin = self.theta.sin();
        let cos = self.theta.cos();
        let mp = self.mass_pole;
        let mc = self.mass_cart;
        let l = self.half_length;
        let t_d = self.theta_dot;
        let g = self.g;

        let temp = (force + mp * l * t_d * t_d * sin) / (mc + mp);
        let t_dd = (g * sin - cos * temp) / (l * (4. / 3. - (mp * cos * cos) / (mc + mp)));
        let x_dd = temp - mp * t_dd * cos / (mc + mp);

        self.x += self.tau * self.x_dot;
        self.x_dot += self.tau * x_dd;
        self.theta += self.tau * self.theta_dot;
        self.theta_dot += self.tau * t_dd;

        self.episode_length += 1;

        let done_x = self.x < -self.threshold_x || self.x > self.threshold_x;
        let done_theta = self.theta < -self.threshold_theta || self.theta > self.threshold_theta;
        let done = done_x || done_theta || self.episode_length >= 500;

        ([self.x, self.x_dot, self.theta, self.theta_dot], 1., done)
    }

    fn reset(&mut self) -> State {
        let dist = Uniform::new(-0.05f32, 0.05f32).unwrap();
        let mut rng = if let Some(seed) = self.seed {
            ChaCha8Rng::seed_from_u64(seed)
        } else {
            ChaCha8Rng::from_os_rng()
        };
        self.x = dist.sample(&mut rng);
        self.x_dot = dist.sample(&mut rng);
        self.theta = dist.sample(&mut rng);
        self.theta_dot = dist.sample(&mut rng);
        self.episode_length = 0;
        self.episode_done = false;
        [self.x, self.x_dot, self.theta, self.theta_dot]
    }

    fn seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }
}

impl Default for CartPole {
    fn default() -> Self {
        let dist = Uniform::new(-0.05f32, 0.05f32).unwrap();
        let mut rng = ChaCha8Rng::from_os_rng();
        CartPole {
            g: 9.8,
            mass_cart: 1.,
            mass_pole: 0.1,
            half_length: 0.5,
            tau: 0.02,
            force_mag: 10.,
            x: dist.sample(&mut rng),
            x_dot: dist.sample(&mut rng),
            theta: dist.sample(&mut rng),
            theta_dot: dist.sample(&mut rng),
            threshold_x: 2.4,
            threshold_theta: 12. * 2. * PI / 360.,
            episode_done: false,
            episode_length: 0,
            seed: Some(0),
        }
    }
}
