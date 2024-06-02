use sp1_sdk::{SP1Proof, ProverClient, SP1VerifyingKey};
use crate::helper::prepare_env;

const ELF: &[u8] = include_bytes!("../../program/snafu/elf/riscv32im-succinct-zkvm-elf");

pub fn snafu(input_path: &str) -> (SP1Proof, SP1VerifyingKey) {
    let stdin = prepare_env(input_path, None);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    (
        client.prove(&pk, stdin).expect("proving failed"),
        vk
    )
}