use std::fs::read_to_string;
use risc0_zkvm::ExecutorEnv;

pub fn prepare_env(input_path: &str, target: Option<i32>) -> ExecutorEnv {
    let file_content = read_to_string(input_path)
        .expect("Should have been able to read the file");
    let string_iter = file_content.split('\n');
    let mut string_array: Vec<String> = string_iter.map(|s| s.to_string()).collect();
    string_array.pop();
    let mut env_builder = ExecutorEnv::builder();
    env_builder.write(&string_array).unwrap();
    match target {
        None => env_builder.build().unwrap(),
        Some(target_row) => env_builder.write(&target_row)
                .unwrap()
                .build()
                .unwrap()
    }
}