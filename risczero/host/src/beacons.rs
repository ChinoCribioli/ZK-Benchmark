use methods::BEACONS_ELF;
use risc0_zkvm::{default_prover, Receipt};
use crate::helper::prepare_env;

pub fn beacons(input_path: &str, target_row: i32) -> Receipt {
    let env = prepare_env(input_path, Some(target_row));

    let prover = default_prover();

    let receipt: Receipt = prover
        .prove(env, BEACONS_ELF)
        .unwrap();
    receipt
}
