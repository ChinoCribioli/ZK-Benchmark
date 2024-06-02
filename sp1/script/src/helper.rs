use sp1_sdk::SP1Stdin;
use std::fs::read_to_string;

pub fn prepare_env(input_path: &str, target: Option<i32>) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    let file_content = read_to_string(input_path)
        .expect("Should have been able to read the file");
    let string_iter = file_content.split('\n');
    let mut string_array: Vec<String> = string_iter.map(|s| s.to_string()).collect();
    string_array.pop();
    stdin.write(&string_array);
    
    match target {
        None => stdin,
        Some(target_row) => {
            stdin.write(&target_row);
            stdin
        }
    }
}