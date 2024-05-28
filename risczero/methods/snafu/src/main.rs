#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn snafu_char_to_int(c: char) -> i64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!("Incorrect SNAFU character."),
    }
}

fn int_to_snafu_char(digit: i8) -> char {
    match digit {
        0 => '0',
        1 => '1',
        2 => '2',
        -1 => '-',
        -2 => '=',
        _ => panic!("Incorrect digit in base 5."),
    }
}

fn main() {
    // read the input
    let input: Vec<String> = env::read();
    
    let mut sum: i64 = 0;
    for snafu in input {
        let l = snafu.len();
        let mut snafu_iter = snafu.chars();
        let mut conversion: i64 = 0;
        for _ in 0..l {
            conversion *= 5;
            conversion += snafu_char_to_int(snafu_iter.next().unwrap());
        }
        sum += conversion;
    }

    // Convert the answer to snafu_format
    let mut digits: Vec<i8> = Vec::new();
    while sum != 0 {
        digits.push((sum % 5) as i8);
        sum /= 5;
    }
    for i in 0..digits.len() {
        if digits[i] == 3 || digits[i]== 4 {
            digits[i] -= 5;
            if i == digits.len()-1 {
                digits.push(1);
            } else {
                digits[i+1] += 1;
            }
        }
    }

    let mut answer: String = String::new();
    for d in (0..digits.len()).rev() {
        answer.push(int_to_snafu_char(digits[d]));
    }
    
    // write public output to the journal
    env::commit(&answer);
}