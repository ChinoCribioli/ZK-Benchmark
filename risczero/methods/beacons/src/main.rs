#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use serde_json::from_str;

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    // read the input
    let input: Vec<String> = env::read();
    let target_row: i32 = env::read();
    let mut interval_openers: Vec<i32> = Vec::new();
    let mut interval_closers: Vec<i32> = Vec::new();

    for line in input {
        let mut split = line.split([',','=',':']);
        let s_x: i32 = from_str::<i32>(split.nth(1).unwrap()).unwrap();
        let s_y: i32 = from_str::<i32>(split.nth(1).unwrap()).unwrap();
        let b_x: i32 = from_str::<i32>(split.nth(1).unwrap()).unwrap();
        let b_y: i32 = from_str::<i32>(split.nth(1).unwrap()).unwrap();
        let distance_to_beacon: i32 = (s_x-b_x).abs() + (s_y-b_y).abs();
        let distance_to_target_row: i32 = (target_row-s_y).abs();
        if distance_to_beacon <= distance_to_target_row {// In this case we have no interval to measure
            continue;
        }
        let (mut opener_adjust, mut closer_adjust) = (0,0);
        if b_y == target_row {// If the beacon is in the target row, must shrink the measured interval
            if s_x < b_x {// either by subtracting 1 to the closer
                closer_adjust = -1;
            } else {// or by adding 1 to the opener
                opener_adjust = 1;
            }
        }
        interval_openers.push(s_x - distance_to_beacon + distance_to_target_row + opener_adjust);
        interval_closers.push(s_x + distance_to_beacon - distance_to_target_row + closer_adjust);
    }

    interval_openers.sort();
    interval_closers.sort();

    let mut answer: i32 = 0;
    let l  = interval_openers.len();
    let mut last_event = interval_openers[0];
    let mut active_intervals: u16 = 0;
    let (mut opener, mut closer): (usize, usize) = (0,0);
    while closer < l {
        if opener == l || interval_openers[opener] > interval_closers[closer] {
            if active_intervals > 0 {
                answer += interval_closers[closer] - last_event;
            }
            last_event = interval_closers[closer];
            active_intervals -= 1;
            closer += 1;
            if active_intervals == 0 {
                answer += 1;
            }
        } else {
            if active_intervals > 0 {
                answer += interval_openers[opener] - last_event;
            }
            last_event = interval_openers[opener];
            active_intervals += 1;
            opener += 1;
        }
    }

    // write public output to the journal
    env::commit(&answer);

}