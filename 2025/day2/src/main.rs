use std::{cmp::max, collections::BTreeMap, mem, str::from_utf8, time::Instant};

use itertools::Itertools;
use neerajsi::*;

fn handle_range(r: &[&[u8]; 2]) -> u64 {
    let mut id_sum = 0u64;
    println!("Range: {}-{}", from_utf8(r[0]).unwrap(), from_utf8(r[1]).unwrap());

    let mut cur = r[0].iter().map(|c| *c).collect_vec();
    let mut increment = false;

    loop {
        if increment {
            let mut add_digit = true;
            for d in (0..cur.len()/2).rev() {
                if cur[d] < b'9' {
                    cur[d] += 1;
                    add_digit = false;
                    break;
                } else {
                    cur[d] = b'0';
                }
            }

            if add_digit {
                cur[0] = b'1';
                cur[1..].fill(b'0');
                cur.push(b'0');
            }
        }

        increment = true;

        if cur.len() > r[1].len() ||
            (cur.len() == r[1].len() && cur.as_slice() > r[1]) {

            break;
        }

        if cur.len() % 2 != 0 {
            continue;
        }

        let half_len = cur.len() / 2;
        let (left_half, right_half) = cur.split_at_mut(half_len);
            
        right_half.copy_from_slice(left_half);

        if cur.len() == r[0].len() &&
            cur.as_slice() < r[0] {
            continue;
        }

        if (cur.len() == r[1].len() && cur.as_slice() > r[1]) {
            break;
        }

        let id = from_utf8(cur.as_slice()).unwrap().parse::<u64>().unwrap();
        
        println!("\t{id}");
        id_sum += id;
        

    }

    id_sum
}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let range_list = input
        .split(',')
        .map(|s| s.trim_ascii().as_bytes())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(|&c| c == b'-')
             .collect_array::<2>()
             .unwrap()
        })
        .collect::<Vec<_>>();

    let mut id_sum = 0u64;
    let mut max_range_width = 0u64;
    let mut max_num = 0u64;
    for r in &range_list {
        id_sum += handle_range(&r);
        let start = from_utf8(r[0]).unwrap().parse::<u64>().unwrap();
        let end = from_utf8(r[1]).unwrap().parse::<u64>().unwrap();
        max_range_width = max(max_range_width, end - start);
        max_num = max(max_num, end);
    }

    let mut id_sum2 = 0u64;
    for r in &range_list {
        let start = from_utf8(r[0]).unwrap().parse::<u64>().unwrap();
        let end = from_utf8(r[1]).unwrap().parse::<u64>().unwrap();

        println!("Range: {}-{}", start, end);
        for id in start..=end {
            if id < 10 {
                continue;
            }

            for pattern_len in 1..=5 {
                let divisor = 10u64.pow(pattern_len);
                let min_value = divisor / 10;
                let piece = id % divisor;
                let mut remain = id / divisor;
                let mut match_found = false;
                while remain > 0 {
                    if remain % divisor == piece {
                        match_found = true;
                        if remain < min_value {
                            match_found = false;
                        }
                        remain /= divisor;
                        continue;
                    } else {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    println!("\t{id} {divisor:?}");
                    id_sum2 += id;
                    break;
                }
                
            }
        }
    }

    dbg!(id_sum);
    dbg!(id_sum2);
    dbg!(max_range_width);
    dbg!(max_num);
    dbg!(range_list.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_range() {
        let r = [b"2121212118".as_slice(), b"2121212124".as_slice()];
        let result = handle_range(&r);
        assert_eq!(result, 0);
    }

    #[test] 
    fn test_handle_range_2_20() {
        let r = [b"2".as_slice(), b"20".as_slice()];
        let result = handle_range(&r);
        assert_eq!(result, 11);
    }
}