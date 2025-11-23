use crate::{
    fault::BandwidthType,
    process::{ProcessHandle, ProcessId},
    random::Seed,
    simulation::Simulation,
    simulation_result::SimulationResult,
    time::Jiffies,
};

pub struct SimulationBuilder<F>
where
    F: Fn() -> Box<dyn ProcessHandle>,
{
    seed: Seed,
    max_steps: Jiffies,
    max_network_latency: Jiffies,
    process_count: usize,
    factory: Option<F>,
    bandwidth: BandwidthType,
}

impl<F> SimulationBuilder<F>
where
    F: Fn() -> Box<dyn ProcessHandle>,
{
    pub fn new() -> SimulationBuilder<F> {
        SimulationBuilder {
            seed: 0,
            max_steps: Jiffies(1000),
            max_network_latency: Jiffies(10),
            process_count: 0,
            factory: None,
            bandwidth: BandwidthType::Unbounded,
        }
    }

    pub fn with_seed(mut self, seed: Seed) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_max_steps(mut self, max_steps: Jiffies) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn with_max_network_latency(mut self, max_network_latency: Jiffies) -> Self {
        self.max_network_latency = max_network_latency;
        self
    }

    pub fn with_process_count(mut self, count: usize) -> Self {
        self.process_count = count;
        self
    }

    pub fn with_factory<G>(self, factory: G) -> SimulationBuilder<G>
    where
        G: Fn() -> Box<dyn ProcessHandle>,
    {
        SimulationBuilder {
            bandwidth: self.bandwidth,
            seed: self.seed,
            max_steps: self.max_steps,
            max_network_latency: self.max_network_latency,
            process_count: self.process_count,
            factory: Some(factory),
        }
    }

    pub fn with_bandwidth(mut self, bandwidth: BandwidthType) -> Self {
        self.bandwidth = bandwidth;
        self
    }

    pub fn build(self) -> Simulation {
        let factory = self
            .factory
            .expect("Factory function must be provided before building");

        let mut simulation = Simulation::new(self.seed, self.max_steps, self.max_network_latency);

        (1..=self.process_count).map(|id| {
            simulation.add_process(id, self.bandwidth, factory());
        });

        simulation
    }
}
