extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::{components::components, snafu::snafu, beacons::beacons};
use risc0_zkvm::Receipt;
use methods::{COMPONENTS_ID, SNAFU_ID, BEACONS_ID};
use std::time::Instant;

// run `cargo bench` in the host directory to run benchmarks.

#[benchmark("Components", [
    ("SampleInput", (String::from("./inputs/components/sampleInput.txt"), 157)),
    ("FullInput", (String::from("./inputs/components/fullInput.txt"), 7824)),
])]
fn bench_components(b: &mut BenchmarkRun, (path, answer): (String, usize)) {
    let mut now = Instant::now(); 
    let receipt: Receipt = components(&path);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    now = Instant::now();
    receipt.verify(COMPONENTS_ID).unwrap();
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_receipt = serde_json::to_string(&receipt).unwrap();
    b.log("proof_size_bytes", serialized_receipt.len());

    assert_eq!(receipt.journal.decode::<usize>().unwrap(), answer);
}

#[benchmark("Snafu", [
    ("SampleInput", (String::from("./inputs/snafu/sampleInput.txt"), String::from("2=-1=0"))),
    ("FullInput", (String::from("./inputs/snafu/fullInput.txt"), String::from("2=1-=02-21===-21=200"))),
])]
fn bench_snafu(b: &mut BenchmarkRun, (path, answer): (String, String)) {
    let mut now = Instant::now(); 
    let receipt: Receipt = snafu(&path);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    now = Instant::now();
    receipt.verify(SNAFU_ID).unwrap();
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_receipt = serde_json::to_string(&receipt).unwrap();
    b.log("proof_size_bytes", serialized_receipt.len());

    assert_eq!(receipt.journal.decode::<String>().unwrap(), answer);
}

#[benchmark("Beacons", [
    ("SampleInput", (String::from("./inputs/beacons/sampleInput.txt"), 10, 26)),
    ("FullInput", (String::from("./inputs/beacons/fullInput.txt"), 2_000_000, 5838453)),
])]
fn bench_beacons(b: &mut BenchmarkRun, (path, target_row, answer): (String, i32, i32)) {
    let mut now = Instant::now(); 
    let receipt: Receipt = beacons(&path, target_row);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    now = Instant::now();
    receipt.verify(BEACONS_ID).unwrap();
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_receipt = serde_json::to_string(&receipt).unwrap();
    b.log("proof_size_bytes", serialized_receipt.len());

    assert_eq!(receipt.journal.decode::<i32>().unwrap(), answer);
}

benchy::main!(
    bench_components,
    bench_snafu,
    bench_beacons,
);