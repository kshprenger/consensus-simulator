use dag_based::bullshark::Bullshark;
use simulator::{BandwidthType, Jiffies, SimulationBuilder};
fn main() {
    for procs in (4..1000).step_by(10) {
        let mut sim = SimulationBuilder::NewFromFactory(|| Bullshark::New())
            .MaxLatency(Jiffies(10))
            .MaxTime(Jiffies(234))
            .NetworkBandwidth(BandwidthType::Unbounded)
            .ProcessInstances(procs)
            .Seed(69)
            .Build();
        let metrics = sim.Run();
        println!("{}, {}", procs, metrics.events_total)
    }
}
