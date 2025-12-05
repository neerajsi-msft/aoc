use std::{ops::Range, time::Instant};

use neerajsi::read_stdin_input;
use rangemap::RangeSet;

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let mut lines = input
        .lines()
        .map(|l| l.trim());

    let ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l|{
            let (x, y) = l.split_once('-').unwrap();

            (x.parse::<u64>().unwrap()..y.parse::<u64>().unwrap() + 1)
        })
        .collect::<RangeSet<u64>>();

    let ingredients =
        lines.map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let count = ingredients
        .iter()
        .filter(|i| ranges.contains(i))
        .count();

    println!("{}", count);

    let start = Instant::now();

    let fresh_count = ranges
        .iter()
        .map(|r| r.end - r.start)
        .sum::<u64>();

    let duration = start.elapsed();
    eprintln!("Time elapsed in fresh count(): {:?}", duration);

    println!("{}", fresh_count);
}
