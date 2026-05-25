pub struct Rng {
    state: u64,
    inc: u64,
}

impl Rng {
    pub fn srandom_r(initstate: u64, initseq: u64) -> Self {
        let mut rng = Rng { state: 0, inc: 0 };
        rng.state = 0u64;
        rng.inc = (initseq << 1u64) | 1u64;
        rng.rand_32();
        rng.state += initstate;
        rng.rand_32();
        rng
    }

    pub fn seed(seed: u64) -> Self {
      Rng::srandom_r(seed, 0u64)
    }

    pub fn rand_32(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state =  oldstate.wrapping_mul(6364136223846793005u64).wrapping_add(self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = ((oldstate >> 18u32) ^ oldstate) >> 27u32;
        let rot = oldstate >> 59u32;
        ((xorshifted >> rot) | (xorshifted << ((-(rot as i64)) & 31))) as u32
    }

    pub fn rand_64(&mut self) -> u64 {
        let high = self.rand_32() as u64;
        let low = self.rand_32() as u64;
        (high << 32) | low
    }
}
