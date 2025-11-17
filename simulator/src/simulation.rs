use std::collections::HashSet;

use crate::{
    metrics::{self, Metrics},
    random::{self, Randomizer},
};

struct Simulation {
    randomizer: Randomizer,
}

impl Simulation {
    fn new(seed: random::Seed) -> Self {
        Self {
            randomizer: Randomizer::new(seed),
        }
    }
    fn start(&mut self) {}
    fn stop(&mut self) -> metrics::Metrics {
        Metrics {}
    }
}
