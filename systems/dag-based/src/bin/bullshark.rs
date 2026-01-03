use dag_based::bullshark::Bullshark;
use matrix::{BandwidthType, SimulationBuilder, metrics, time::Jiffies};

fn main() {
    metrics::Clear();
    metrics::Set::<Vec<Jiffies>>("latency", Vec::new());
    metrics::Set::<usize>("timeouts-fired", 0);
    SimulationBuilder::NewFromFactory(|| Box::new(Bullshark::New()))
        .MaxLatency(Jiffies(1000))
        .TimeBudget(Jiffies(1000000))
        .NICBandwidth(BandwidthType::Unbounded)
        .ProcessInstances(100)
        .Seed(23456765)
        .Build()
        .Run();
    println!(
        "Ordered: {:?}",
        metrics::Get::<Vec<Jiffies>>("latency").len()
    );
}
