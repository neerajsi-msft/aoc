use std::collections::BTreeMap;

use neerajsi::*;
use itertools::Itertools;
use union_find::{QuickFindUf, QuickUnionUf, UnionBySize, UnionFind};

fn part1(distance_list: &[(u64, (usize, usize))], connection_count: u32, total_points: usize) -> usize {
    let mut uf: QuickUnionUf<UnionBySize> = QuickUnionUf::new(total_points);

    for &(dist, (a, b)) in distance_list.iter().take(connection_count as usize) {
        let connected = uf.union(a, b);
        if connected {
            println!("Connected {} and {} d^2={}", a, b, dist);
        } else {
            println!("{} and {} are already connected d^2={}", a, b, dist);
        }
    }

    let mut sets = BTreeMap::new();
    for i in 0..total_points {
        let root = uf.find(i);
        let size = uf.get(root).size();
        let entry = sets.entry(root).or_insert(size);
        assert_eq!(*entry, size, "Inconsistent size for set root {}", root);
    }

    for (root, size) in sets.iter() {
        println!("Set rooted at {} has size {}", root, size);
    }

    sets.values().sorted().rev().take(3).product()
}

fn part1_petgraph(distance_list: &Vec<(u64, (usize, usize))>, connection_count: u32, total_points: usize) -> usize {
    use petgraph::unionfind::UnionFind;

    let mut uf = UnionFind::new(total_points);

    for &(dist, (a, b)) in distance_list.iter().take(connection_count as usize) {
        let connected = uf.union(a, b);
        if connected {
            println!("Connected {} and {} d^2={}", a, b, dist);
        } else {
            println!("{} and {} are already connected d^2={}", a, b, dist);
        }
    }

    let mut sets: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..total_points {
        let root = uf.find(i);
        let entry = sets.entry(root).or_insert(0);
        *entry += 1;
    }

    for (root, size) in sets.iter() {
        println!("Set rooted at {} has size {}", root, size);
    }

    let sorted_values = sets.values().map(|v| *v).sorted().collect_vec();
    println!("Sorted sizes: {:?}", sorted_values);

    sorted_values.iter().rev().take(3).product()
}

fn part2(distance_list: &[(u64, (usize, usize))], point_list: &[[u32; 3]]) -> u64 {
    let total_points = point_list.len();
    let mut uf: QuickUnionUf<UnionBySize> = QuickUnionUf::new(total_points);

    for &(dist, (a, b)) in distance_list.iter() {
        let connected = uf.union(a, b);
        if connected {
            let root = uf.find(a);
            let size = uf.get(root).size();
            println!("Connected {} and {} d^2={}, size={}", a, b, dist, size);
            if size == total_points {
                return point_list[a][0] as u64 * point_list[b][0] as u64;
            }
        } else {
            println!("{} and {} are already connected d^2={}", a, b, dist);
        }
    }

    panic!("Could not connect all points!");
}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let connection_count = if input.lines().count() > 20 { 1000 } else { 10 };

    let points = input.lines()
        .map(|l| {
            l.trim_ascii()
             .split(',')
             .map(|n| n.parse::<u32>().unwrap())
             .collect_array::<3>().unwrap()
        })
        .collect_vec();

    let distance_list = points.iter()
        .enumerate()
        .combinations(2)
        .map(|pair| {
            let p1 = pair[0].1;
            let p2 = pair[1].1;
            let dist = 
                p1.iter().zip(p2).map( 
                    |(a, b)| {
                        let delta = *a as i64 - *b as i64;
                        (delta * delta) as u64
                    }
                )
                .sum::<u64>();
            (dist, ( pair[0].0, pair[1].0 ))
        })
        .sorted()
        .collect_vec();

    let p1 = part1(&distance_list, connection_count, points.len());
    // println!("----------------------------------------");
    // let p1_petgraph = part1_petgraph(&distance_list, connection_count, points.len());
    
    println!("Part 1: {}", p1);
    // println!("Part 1 (petgraph): {}", p1_petgraph);
    
    /*
    println!("Lowest distances:");
    for &(dist, (a, b)) in distance_list.iter().take(connection_count as usize + 1) {
        println!("  {} -- {} : d^2={}", a, b, dist);
    }
    */

    let p2 = part2(&distance_list, &points);
    println!("Part 2: {}", p2);

}
