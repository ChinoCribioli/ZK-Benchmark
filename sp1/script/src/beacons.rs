use sp1_sdk::{SP1Proof, ProverClient, SP1VerifyingKey};
use crate::helper::prepare_env;

const ELF: &[u8] = include_bytes!("../../program/beacons/elf/riscv32im-succinct-zkvm-elf");

pub fn beacons(input_path: &str, target_row: i32) -> (SP1Proof, SP1VerifyingKey) {
    let stdin = prepare_env(input_path, Some(target_row));

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    (
        client.prove(&pk, stdin).expect("proving failed"),
        vk
    )
}