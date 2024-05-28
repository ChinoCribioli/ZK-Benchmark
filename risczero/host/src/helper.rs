use std::fs::read_to_string;
use risc0_zkvm::ExecutorEnv;

pub fn prepare_env(input_path: &str) -> ExecutorEnv {
    let file_content = read_to_string(input_path)
        .expect("Should have been able to read the file");
    let string_iter = file_content.split('\n');
    let mut string_array: Vec<String> = string_iter.map(|s| s.to_string()).collect();
    string_array.pop();
    ExecutorEnv::builder()
        .write(&string_array)
        .unwrap()
        .build()
        .unwrap()
}