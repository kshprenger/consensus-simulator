use std::{fs::File, sync::Mutex};

use dag_based::bullshark::Bullshark;
use matrix::{
    BandwidthDescription, Distributions, LatencyDescription, SimulationBuilder, global::anykv,
    time::Jiffies,
};

use std::io::Write;

fn main() {
    let file = File::create("results_bullshark.csv").unwrap();
    let file = Mutex::new(file);

    (1500..=1500).into_iter().for_each(|k_validators| {
        // 1 jiffy == 1 real millisecond
        let mut sim = SimulationBuilder::NewDefault()
            .AddPool::<Bullshark>("Validators", k_validators)
            .LatencyTopology(&[LatencyDescription::WithinPool(
                "Validators",
                Distributions::Normal(Jiffies(200), Jiffies(20)),
            )])
            .TimeBudget(Jiffies(24000_000)) // Simulating 400 min of real time execution
            .NICBandwidth(BandwidthDescription::Bounded(
                30 * 1024 * 1024 / (8 * 1000), // 30Mb/sec NICs
            ))
            .Seed(k_validators as u64)
            .Build();

        // (avg_latency, total_vertex)
        anykv::Set::<(f64, usize)>("avg_latency", (0.0, 0));

        sim.Run();

        let ordered = anykv::Get::<(f64, usize)>("avg_latency").1;
        let avg_latency = anykv::Get::<(f64, usize)>("avg_latency").0;

        writeln!(
            file.lock().unwrap(),
            "{} {} {}",
            k_validators,
            ordered,
            avg_latency
        )
        .unwrap();
    })
}
