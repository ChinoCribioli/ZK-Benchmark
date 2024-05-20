//! A simple script to generate and verify the proof of a given program.

use std::fs::read_to_string;

use sp1_sdk::{ProverClient, SP1Proof, SP1Stdin};
// use sp1_core::stark::types::ShardProof;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let file_content = read_to_string("./day3of2022.txt")
        .expect("Should have been able to read the file");
    let string_iter = file_content.split('\n');
    let mut string_array: Vec<String> = string_iter.map(|s| s.to_string()).collect();
    string_array.pop();
    stdin.write(&string_array);
    
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

    print_type_of(&proof);

    println!("successfully generated and verified proof for the program!");

    let mut deserialized_proof = SP1Proof::load("proof-with-io.json")
        .expect("couldn't retrieve proof");
    client.verify(&deserialized_proof, &vk).expect("deserialized proof is invalid.");
    println!("se verifico la prueba deserializada: {}", deserialized_proof.public_values.read::<usize>());
}
