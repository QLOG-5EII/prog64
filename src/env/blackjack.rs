use rand::{
    Rng, SeedableRng,
    distr::uniform::{UniformSampler, UniformUsize},
};
use rand_chacha::ChaCha8Rng;

use crate::env::{Environment, Score, State};

const CARDS: [u8; 13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

pub struct BlackJack {
    player_hand: Vec<u8>,
    dealer_hand: u8,
    episode_done: bool,
    seed: Option<u64>,
    num_round: usize,
    rng: rand_chacha::ChaCha8Rng,
}

impl Environment for BlackJack {
    fn step(&mut self, action: f32) -> (State, Score, bool) {
        if self.episode_done {
            let state = self.get_state();
            return (state, 0., true);
        }

        if action >= 0. {
            // Hit
            let card = draw_card(&mut self.rng);
            self.player_hand.push(card);
            let state = self.get_state();
            let bust = state[0] > 21.;
            let score = if bust { -1. } else { 0. };
            if bust {
                self.new_round();
            }
            (state, score, self.num_round > 100)
        } else {
            // Stick
            let state = self.get_state();
            let dealer_score = self.dealer_score();
            let player_score = self.player_score();
            let score = if dealer_score > player_score {
                -1.
            } else if player_score > dealer_score {
                1.
            } else {
                0.
            };
            self.new_round();
            (state, score, self.num_round > 100)
        }
    }

    fn reset(&mut self) -> State {
        if let Some(seed) = self.seed {
            self.rng = ChaCha8Rng::seed_from_u64(seed);
        } else {
            self.rng = ChaCha8Rng::from_os_rng();
        }
        self.player_hand = vec![draw_card(&mut self.rng), draw_card(&mut self.rng)];
        self.dealer_hand = draw_card(&mut self.rng);
        self.episode_done = false;
        self.num_round = 0;
        self.get_state()
    }

    fn seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }
}

fn draw_card<R>(rng: &mut R) -> u8
where
    R: Rng + ?Sized,
{
    let sampler = UniformUsize::new(0, 13).unwrap();
    CARDS[sampler.sample(rng)]
}

fn compute_score(hand: &[u8]) -> (u8, u8) {
    let mut low_score = 0;
    let mut has_11 = false;
    for card in hand.iter() {
        low_score += card;
        if *card == 1 {
            has_11 = true;
        }
    }
    let high_score = if has_11 && low_score + 10 <= 21 {
        low_score + 10
    } else {
        low_score
    };
    (low_score, high_score)
}

impl BlackJack {
    fn player_score(&self) -> u8 {
        let (low, high) = compute_score(&self.player_hand);
        if high > 21 { low } else { high }
    }

    fn get_state(&self) -> State {
        let (low, high) = compute_score(&self.player_hand);
        let dealer = self.dealer_hand as f32;
        [low as f32, high as f32, dealer, 21.]
    }

    fn dealer_score(&mut self) -> u8 {
        let mut score;
        let mut has_11 = false;
        if self.dealer_hand == 1 {
            score = 11;
            has_11 = true;
        } else {
            score = self.dealer_hand;
        }
        while score < 17 {
            let card = draw_card(&mut self.rng);
            if card == 1 {
                has_11 = true;
                score += 11;
            } else {
                score += card;
            };
            if score > 21 && has_11 {
                has_11 = false;
                score -= 10;
            }
        }
        score
    }

    fn new_round(&mut self) {
        self.dealer_hand = draw_card(&mut self.rng);
        self.player_hand = vec![draw_card(&mut self.rng), draw_card(&mut self.rng)];
        self.num_round += 1;
    }
}

impl Default for BlackJack {
    fn default() -> Self {
        let mut rng = ChaCha8Rng::from_os_rng();
        BlackJack {
            player_hand: vec![draw_card(&mut rng), draw_card(&mut rng)],
            dealer_hand: draw_card(&mut rng),
            episode_done: false,
            seed: None,
            num_round: 0,
            rng,
        }
    }
}
