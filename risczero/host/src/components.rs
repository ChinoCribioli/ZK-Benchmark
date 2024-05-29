use methods::COMPONENTS_ELF;
use risc0_zkvm::{default_prover, Receipt};
use crate::helper::prepare_env;

pub fn components(input_path: &str) -> Receipt {
    let env = prepare_env(input_path, None);

    let prover = default_prover();

    let receipt: Receipt = prover
        .prove(env, COMPONENTS_ELF)
        .unwrap();
    receipt
}
