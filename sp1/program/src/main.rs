//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

fn get_priority(component: char) -> usize {
    let char_as_int = component as usize;
    if (97..=122).contains(&char_as_int) {
        char_as_int-97
    } else {
        char_as_int-39
    }
}

pub fn main() {
    let input = sp1_zkvm::io::read::<Vec<String>>();
    let mut answer: usize = 0;
    for line in input {
        let len = line.len();
        let mut line_chars = line.chars();
        let mut is_present: Vec<bool> = vec![false; 52];
        for _ in 0..len/2 {
            let priority = get_priority(line_chars.next().unwrap());
            is_present[priority] = true;
        }
        for _ in 0..len/2 {
            let new_priority = get_priority(line_chars.next().unwrap());
            if is_present[new_priority] {
                answer += new_priority+1;
                break;
            }
        }
    }
    sp1_zkvm::io::commit(&answer);
}
