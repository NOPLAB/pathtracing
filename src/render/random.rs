pub struct XorShiftRandom {
    state: u32,
}

impl XorShiftRandom {
    pub fn new(seed: u32) -> XorShiftRandom {
        XorShiftRandom { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next() as f64 / std::u32::MAX as f64
    }
}
