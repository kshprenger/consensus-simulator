use std::fs::File;

use dag_based::bullshark::Bullshark;
use matrix::{BandwidthType, SimulationBuilder, global::anykv, time::Jiffies};
use rayon::prelude::*;

use std::io::Write;

fn main() {
    let mut file = File::create("results.csv").unwrap();

    writeln!(file, "validators, throughputper20min").unwrap();

    (4..200)
        .into_par_iter()
        .map(|proc_num| {
            // 1 jiffy == 1 real millisecond
            let sim = SimulationBuilder::NewDefault()
                .AddPool("Validators", proc_num, || Bullshark::New())
                .MaxLatency(Jiffies(150)) // 150 ms of max network latency
                .TimeBudget(Jiffies(1200_000)) // Simulating 20 min of real time execution
                .NICBandwidth(BandwidthType::Bounded(10_000_000 / 1000)) // 10Gb/sec NICs
                .Seed(proc_num as u64)
                .Build();

            anykv::Set::<Vec<Jiffies>>("latency", Vec::new());
            anykv::Set::<usize>("timeouts-fired", 0);

            sim.Run();

            let average = anykv::Get::<Vec<Jiffies>>("latency")
                .iter()
                .map(|&x| x.0 as f64)
                .enumerate()
                .fold(0.0, |acc, (i, x)| acc + (x - acc) / (i + 1) as f64);

            println!("{proc_num}");

            (proc_num, average)
        })
        .collect::<Vec<(usize, f64)>>()
        .into_iter()
        .for_each(|(proc_mum, ordered)| {
            writeln!(file, "{},{}", proc_mum, ordered).unwrap();
        });
}
