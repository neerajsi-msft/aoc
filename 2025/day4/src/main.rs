use std::{cmp::max, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, mem, str::from_utf8, time::Instant};

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

    fn neighbor_count(loc: Location, map: &[impl AsRef<[u8]>]) -> usize {
        let neighbors = neighbors(&map, loc);
        neighbors.filter_map(
            |(_, &v)| {
                if v == b'@' {
                    Some(())
                } else {
                    None
                }
            }
        )
        .count()
    }

    fn is_removable(loc: Location, map: &[impl AsRef<[u8]>]) -> bool {
        let c = index2d_array!(map, loc);
        match c {
            b'@' => {
                let count = neighbor_count(loc, map);
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

                for (neighbor, &v) in neighbors(&current_map, loc) {
                    if v == b'@' {
                        next_check_set.insert(neighbor);
                    }
                }
            }
        }

        mem::swap(&mut check_set, &mut next_check_set);
        next_check_set.clear();
    }

    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);

    println!("Total removed: {}", total_removed);

    let start_time = Instant::now();
    total_removed = 0;
    
    let mut location_map = HashMap::with_capacity(grid.cell_count());
    let mut removal_queue = Vec::with_capacity(grid.cell_count() / 4);
    for loc in grid.cell_range() {
        let c = index2d_array!(map, loc);
        if c == b'@' {
            let neighbor_count = neighbor_count(loc, &map);
            let is_removable = neighbor_count <= 3;
            if is_removable {
                removal_queue.push(loc);
            }

            location_map.insert(loc, (neighbor_count, is_removable));
        }
    }

    while !removal_queue.is_empty() {
        let loc = removal_queue.pop().unwrap();
        assert!(location_map.get(&loc).unwrap().1);
        assert!(location_map.get(&loc).unwrap().0 <= 3);

        total_removed += 1;
        location_map.remove(&loc);

        for neighbor in grid.neighbors(loc) {
            let entry = location_map.entry(neighbor);
            entry.and_modify(|e| {
                assert!(e.0 > 0);
                e.0 -= 1;
                if e.0 <= 3 {
                    if !e.1 {
                        e.1 = true;
                        removal_queue.push(neighbor);
                    }
                }
            });
        }
    }

    let duration = start_time.elapsed();
    println!("Time taken (method 2): {:?}", duration);
    println!("Total removed (method 2): {}", total_removed);

}
