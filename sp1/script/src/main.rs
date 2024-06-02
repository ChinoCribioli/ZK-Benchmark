//! A simple script to generate and verify the proof of the "components" problem.

use std::fs::read_to_string;
use sp1_sdk::{ProverClient, SP1Proof, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/components/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Retrieve and write input.
    let mut stdin = SP1Stdin::new();
    let file_content = read_to_string("./inputs/components/fullInput.txt")
        .expect("Should have been able to read the file");
    let string_iter = file_content.split('\n');
    let mut string_array: Vec<String> = string_iter.map(|s| s.to_string()).collect();
    string_array.pop();
    stdin.write(&string_array);
    
    // Prove program.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).expect("proving failed");

    // Read output.
    let answer = proof.public_values.read::<usize>();
    assert_eq!(answer, 7824);
    
    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!");
    
    // Deserialize proof from .json file.
    let mut deserialized_proof = SP1Proof::load("proof-with-io.json")
        .expect("couldn't retrieve proof");
    client.verify(&deserialized_proof, &vk).expect("deserialized proof is invalid.");
    println!("Answer coming from deserialized proof: {}", deserialized_proof.public_values.read::<usize>());
}
