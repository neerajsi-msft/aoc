use std::{cmp::max, collections::BTreeMap, mem, str::from_utf8, time::Instant};

use itertools::Itertools;
use neerajsi::*;


fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let rot_list = input.lines()
        .map(|l| {
            let (l_r, n) = l.split_at(1);
            let n = n.parse::<i32>().unwrap();
            match l_r.chars().next().unwrap() {
                'L' => -n,
                'R' => n,
                _ => panic!("Unknown dir {l_r}")
            }
        })
        .collect::<Vec<i32>>();

    dbg!(&rot_list);

    let mut pos = 50i32;
    let mut zero_count = 0;
    let mut pass_count = 0;
    for r in rot_list {
        assert_ne!(r, 0);

        dbg!(pos);
        dbg!(r);

        let was_zero = pos == 0;

        pos += r;
        
        let wrap_count = 
            if pos <= 0 {
                if was_zero {
                    -pos / 100
                } else {
                    1 - pos / 100
                }
            } else if pos >= 100 {
                pos / 100
            } else {
                0
            };

        dbg!(pos);
        dbg!(wrap_count);

        pass_count += wrap_count;

        // dbg!(pos);
        pos = pos.rem_euclid(100);
        // dbg!(pos);
        zero_count += (pos == 0) as i32;
    }


    dbg!(zero_count);
    dbg!(pass_count);
}