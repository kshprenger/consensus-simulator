use std::{fs::File, sync::Mutex};

use dag_based::sparse_bullshark::SparseBullshark;
use matrix::{
    BandwidthDescription, Distributions, LatencyDescription, SimulationBuilder, global::anykv,
    time::Jiffies,
};
use rayon::prelude::*;
use std::io::Write;

fn main() {
    let file = File::create("results_sparse_bullshark.csv").unwrap();
    let file = Mutex::new(file);

    (150..=1000)
        .step_by(100)
        .par_bridge()
        .into_par_iter()
        .for_each(|d| {
            anykv::Set::<(f64, usize)>("avg_latency", (0.0, 0));
            anykv::Set::<usize>("D", d); // Sample size

            let mut sim = SimulationBuilder::NewDefault()
                .AddPool::<SparseBullshark>("Validators", 1500)
                .LatencyTopology(&[LatencyDescription::WithinPool(
                    "Validators",
                    Distributions::Normal(Jiffies(200), Jiffies(20)),
                )])
                .TimeBudget(Jiffies(24000_000)) // Simulating 400 min of real time execution
                .NICBandwidth(BandwidthDescription::Bounded(
                    50 * 1024 * 1024 / (8 * 1000), // 50Mb/sec NICs
                ))
                .Seed(d as u64)
                .Build();

            // (avg_latency, total_vertex)
            anykv::Set::<(f64, usize)>("avg_latency", (0.0, 0));

            sim.Run();

            let ordered = anykv::Get::<(f64, usize)>("avg_latency").1;
            let avg_latency = anykv::Get::<(f64, usize)>("avg_latency").0;

            writeln!(file.lock().unwrap(), "{} {} {}", d, ordered, avg_latency).unwrap();
        });
}
