
use itertools::Itertools;
use neerajsi::read_stdin_input;

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let points = input
        .lines()
        .map(|l| {
            let (x, y) = l.trim_ascii().split_once(',').unwrap();

            [x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()]
        })
        .collect_vec();

    let max_area = part1(&points);

    println!("p1: {}", max_area);
}

fn part1(points: &[[u32; 2]]) -> u64 {
    points.iter()
        .array_combinations::<2>()
        .map(|[p, q]| {
            (0..2).map(|i| (p[i].abs_diff(q[i]) + 1) as u64).product::<u64>()
        })
        .max()
        .unwrap()
}

fn part2(points: &[[i64; 2]]) -> i64 {

    0
}