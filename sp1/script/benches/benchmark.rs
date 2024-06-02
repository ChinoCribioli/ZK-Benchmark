use script::{components::components, snafu::snafu, beacons::beacons};
use benchy::{benchmark, BenchmarkRun};
use sp1_sdk::{ProverClient, SP1Proof, SP1VerifyingKey};
use std::time::Instant;

#[benchmark("Components", [
    ("SampleInput", (String::from("./inputs/components/sampleInput.txt"), 157)),
    ("FullInput", (String::from("./inputs/components/fullInput.txt"), 7824)),
])]
fn bench_components(b: &mut BenchmarkRun, (path, answer): (String, usize)) {
    let mut now = Instant::now(); 
    let (mut proof, vk): (SP1Proof, SP1VerifyingKey) = components(&path);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    let client: ProverClient = ProverClient::new();
    now = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_proof = serde_json::to_string(&proof).unwrap();
    b.log("proof_size_bytes", serialized_proof.len());

    assert_eq!(proof.public_values.read::<usize>(), answer);
}

#[benchmark("Snafu", [
    ("SampleInput", (String::from("./inputs/snafu/sampleInput.txt"), String::from("2=-1=0"))),
    ("FullInput", (String::from("./inputs/snafu/fullInput.txt"), String::from("2=1-=02-21===-21=200"))),
])]
fn bench_snafu(b: &mut BenchmarkRun, (path, answer): (String, String)) {
    let mut now = Instant::now(); 
    let (mut proof, vk): (SP1Proof, SP1VerifyingKey) = snafu(&path);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    let client: ProverClient = ProverClient::new();
    now = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_proof = serde_json::to_string(&proof).unwrap();
    b.log("proof_size_bytes", serialized_proof.len());

    assert_eq!(proof.public_values.read::<String>(), answer);
}

#[benchmark("Beacons", [
    ("SampleInput", (String::from("./inputs/beacons/sampleInput.txt"), 10, 26)),
    ("FullInput", (String::from("./inputs/beacons/fullInput.txt"), 2_000_000, 5838453)),
])]
fn bench_beacons(b: &mut BenchmarkRun, (path, target_row, answer): (String, i32, i32)) {
    let mut now = Instant::now(); 
    let (mut proof, vk): (SP1Proof, SP1VerifyingKey) = beacons(&path, target_row);
    b.log("proof_generation_time", now.elapsed().as_millis() as usize);
    let client: ProverClient = ProverClient::new();
    now = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    b.log("verification_time", now.elapsed().as_millis() as usize);
    let serialized_proof = serde_json::to_string(&proof).unwrap();
    b.log("proof_size_bytes", serialized_proof.len());

    assert_eq!(proof.public_values.read::<i32>(), answer);
}

benchy::main!(
    bench_components,
    bench_snafu,
    bench_beacons,
);