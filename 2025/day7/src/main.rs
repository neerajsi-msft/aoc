use core::panic;
use std::collections::BTreeSet;
use neerajsi::read_stdin_input;

type BitWordType = u64;
const BITS_PER_WORD: usize = core::mem::size_of::<BitWordType>() * 8;

fn set_bit(bitmap: &mut [BitWordType], index: usize) {
    let word_index = index / BITS_PER_WORD;
    let bit_index = index % BITS_PER_WORD;
    bitmap[word_index] |= 1 << bit_index;
}

fn clear_bit(bitmap: &mut [BitWordType], index: usize) {
    let word_index = index / BITS_PER_WORD;
    let bit_index = index % BITS_PER_WORD;
    bitmap[word_index] &= !(1 << bit_index);
}

fn get_bit(bitmap: &[BitWordType], index: usize) -> bool {
    let word_index = index / BITS_PER_WORD;
    let bit_index = index % BITS_PER_WORD;
    (bitmap[word_index] & (1 << bit_index)) != 0
}

fn show_vector(bitmap: &[BitWordType], width: usize) {
    for c in 0..width {
        let c = match get_bit(bitmap, c) {
            true => '|',
            false => '.',
        };

        print!("{}", c);
    }

    println!()
}

fn part1(w: usize, h: usize, start: (usize, usize), splitters: &BTreeSet<(usize, usize)>) -> usize {
    let rounded_w = (w + BITS_PER_WORD) / BITS_PER_WORD;
    let mut splitter_grid = vec![vec![]; h];
    for s in splitters {
        let row = &mut splitter_grid[s.0];
        if row.is_empty() {
            row.resize(rounded_w, 0);
        }

        set_bit(row, s.1);
    }

    let mut beams = vec![0; rounded_w];
    let mut intersection = beams.clone();
    intersection.push(0);

    set_bit(&mut beams, start.1);

    let mut split_count = 0;
    for r in &splitter_grid {
        if r.is_empty() { 
            continue;        
        }

        for c in 0..rounded_w {
            intersection[c] = beams[c] & r[c];
            beams[c] ^= intersection[c];
            split_count += intersection[c].count_ones() as usize;
        }

        let mut prev = 0;
        for c in 0..rounded_w {
            let mut bits = prev >> (BITS_PER_WORD - 1);
            bits |= intersection[c] << 1;
            bits |= intersection[c + 1] << (BITS_PER_WORD - 1);
            bits |= intersection[c] >> 1;
            prev = intersection[c];
            beams[c] |= bits;
        }

        /*
        print!("beams: ");
        show_vector(&beams, w);
        print!("inter: ");
        show_vector(&intersection, w);
        */

    }

    for i in w..rounded_w * BITS_PER_WORD {
        clear_bit(&mut beams, i);
    }

    split_count
}

fn part2(w: usize, h: usize, start: (usize, usize), splitters: &BTreeSet<(usize, usize)>) -> u64 {
    let mut beams = vec![0u64; w];
    let mut next_beams = beams.clone();

    beams[start.1] = 1;

    for r in 0..h {
        for c in 0..w {
            if beams[c] == 0 {
                continue;
            }

            let count = beams[c];

            if splitters.contains(&(r, c)) {
                // Splitter
                if c > 0 {
                    next_beams[c - 1] += count;
                }
                if c + 1 < w {
                    next_beams[c + 1] += count;
                }
            } else {
                // Straight
                next_beams[c] += count;
            }
        }
        std::mem::swap(&mut beams, &mut next_beams);
        next_beams.fill(0);
    }

    beams.iter().sum()
}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)|
            l.trim_ascii().chars()
             .enumerate()
             .map(move |(j, ch)| (i, j, ch))
        );

    let mut start = None;
    let mut splitters = BTreeSet::new();
    let mut width = 0;
    let mut height = 0;

    for (r, c, ch) in map {
        width = width.max(c + 1);
        height = height.max(r + 1);
        match ch {
            'S' => if start.replace((r, c)).is_some() {
                panic!("Multiple start positions found");
            },
            '^' => {
                splitters.insert((r, c));
            }
            '.' => {},
            _ => panic!("Unexpected character '{}' at ({}, {})", ch, r, c),
        }
    }

    let start = start.expect("No start position found");

    assert_eq!(start.0, 0, "Start position must be in the first row");

    let result = part1(width, height, start, &splitters);
    println!("p1 {}", result);

    let result = part2(width, height, start, &splitters);
    println!("p2 {}", result);

}
