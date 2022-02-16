#![no_std]

#[inline(always)]
pub fn wyrand(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0xa0761d6478bd642f);
    let r = u128::from(*seed) * u128::from(*seed ^ 0xe7037ed1a0b428db);
    ((r >> 64) ^ r) as u64
}

pub struct WyRng {
    seed: u64,
}

impl WyRng {
    pub fn new(seed: u64) -> Self {
        WyRng {
            seed: seed,
        }
    }

    pub fn generate(&mut self) -> u64 {
        wyrand(&mut self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::WyRng;

    #[test]
    fn wyrand_test() {
        let mut rng = WyRng::new(3);
        assert_eq!(0x3e9_9a77_2750_dcbe, rng.generate());
    }
}
