use std::{cmp::max, collections::{BTreeMap, BTreeSet}, mem, str::from_utf8, time::Instant};

use itertools::Itertools;
use neerajsi::*;

fn main() {
    let raw_input = read_stdin_input();
    let input = from_utf8(&raw_input).unwrap();

    let map = input.lines()
        .map(|line| {
            line.trim_ascii().as_bytes()
        })
        .collect::<Vec<_>>();

    let grid = Grid::from_map(&map);

    fn is_removable(loc: Location, map: &[impl AsRef<[u8]>]) -> bool {
        let c = index2d_array!(map, loc);
        match c {
            b'@' => {
                let neighbors = neighbors(&map, loc);
                let count = neighbors.filter_map(
                    |(_, &v)| {
                        if v == b'@' {
                            Some(())
                        } else {
                            None
                        }
                    }
                )
                .count();

                count <= 3
            }
            b'.' => false,
            _ => panic!("Unexpected character in grid {}", c as char)
        }
    };

    let mut accessible_count = 0u32;
    for loc in grid.cell_range() {
        accessible_count += is_removable(loc, &map) as u32;
    }

    println!("Accessible count: {}", accessible_count);

    let mut total_removed = 0u32;
    let mut check_set = BTreeSet::new();
    let mut next_check_set = BTreeSet::new();
    let mut current_map = map.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
    
    let start_time = Instant::now();
    for loc in grid.cell_range() {
        if is_removable(loc, &current_map) {
            check_set.insert(loc);
        }
    }

    while !check_set.is_empty() {
        for &loc in &check_set {
            if is_removable(loc, &current_map) {
                total_removed += 1;
                current_map[loc[0]][loc[1]] = b'.';

                for (neighbor, _) in neighbors(&current_map, loc) {
                    next_check_set.insert(neighbor);
                }
            }
        }

        mem::swap(&mut check_set, &mut next_check_set);
        next_check_set.clear();
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);

    println!("Total removed: {}", total_removed);
}
