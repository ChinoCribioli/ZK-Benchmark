extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::components::components;
use risc0_zkvm::Receipt;
use methods::COMPONENTS_ID;
use std::time::Instant;

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
    assert_eq!(receipt.journal.decode::<usize>().unwrap(), answer);
    
    // TODO: Measure memory bytes of proof 
}

benchy::main!(
    bench_components,
);