use std::{cmp::max, collections::BTreeMap, mem, str::from_utf8, time::Instant};

use itertools::Itertools;
use neerajsi::*;


fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let banks = input.lines()
        .map(|l| {
            l.trim_ascii().as_bytes()
        })
        .collect::<Vec<_>>();

    let mut sum = 0u64;
    for b in &banks {
        let bytes = b;
        // Find max value in the bytes slice
        // excluding the last element.

        let mut max_val = 0u8;
        let mut max_idx = 0usize;
        for i in 0..bytes.len() - 1 {
            if bytes[i] > max_val {
                max_val = bytes[i];
                max_idx = i;
            }
        }

        let mut second_max_val = 0u8;
        for j in max_idx + 1..bytes.len() {
            if bytes[j] > second_max_val {
                second_max_val = bytes[j];
            }
        }

        let val = (max_val - b'0') as u64 * 10 + 
                (second_max_val - b'0') as u64;
        sum += val;
    }

    dbg!(sum);

    let mut sum2 = 0u64;
    for b in &banks {
        let mut digit = 12;
        let mut left_bound = 0usize;
        let mut val = 0u64;
        while digit > 0 {
            let mut max_val = 0u8;
            let mut max_idx = left_bound;
            for i in left_bound..b.len() - (digit - 1) {
                if b[i] > max_val {
                    max_val = b[i];
                    max_idx = i;
                }
            }
            val = val * 10 + (max_val - b'0') as u64;
            left_bound = max_idx + 1;
            digit -= 1;
        }
        sum2 += val;
    }
    dbg!(sum2);
}