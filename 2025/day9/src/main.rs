
use core::panic;
use std::{collections::{BTreeMap, BTreeSet}, env::args};

use bitvec::{bitvec, vec};
use itertools::Itertools;
use neerajsi::read_stdin_input;
use rangemap::{RangeMap, RangeSet, set::Intersection};
use std::cmp::{min, max};

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let debug = args().any(|a| a == "debug");

    let points = input
        .lines()
        .map(|l| {
            let (x, y) = l.trim_ascii().split_once(',').unwrap();

            [x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()]
        })
        .collect_vec();

    let max_area = part1(&points);

    println!("p1: {}", max_area);

    let p2 = part2(&points, debug);
    println!("p2: {}", p2);
}

fn rect_area<P>(points: &[P;2]) -> u64 
    where P: std::borrow::Borrow<[u32; 2]>
{
    let points = points.as_ref();
    let p1 = points[0].borrow();
    let p2 = points[1].borrow();
    (0..2).map(|i| (p1[i].abs_diff(p2[i]) + 1) as u64).product::<u64>()
}


fn part1(points: &[[u32; 2]]) -> u64 {
    points.iter()
        .array_combinations::<2>()
        .map(|points| {
            rect_area(&points)
        })
        .max()
        .unwrap()
}

fn part2(points: &[[u32; 2]], debug: bool) -> u32 {
    let upper_left_bound = points.iter().fold([u32::MAX; 2], |acc, p| {
        [min(acc[0], p[0]), min(acc[1], p[1])]
    });

    let low_right_bound = points.iter().fold([0u32; 2], |acc, p| {
        [max(acc[0], p[0]), max(acc[1], p[1])]
    });

    println!("Bounding box: {:?} to {:?}", upper_left_bound, low_right_bound);


    let vertical_lines = points.iter().tuple_windows()
        .filter(|(p, q)| p[1] == q[1])
        .map(|(p, q)| {
            let col = p[1] as usize;
            let start_row = min(p[0], q[0]) as usize;
            let end_row = max(p[0], q[0]) as usize;
            (col, start_row, end_row)
        })
        .sorted()
        .collect_vec();

    println!("Found {} vertical lines", vertical_lines.len());
    

    let mut max_area = 0;
    let mut rects = points
        .iter()
        .array_combinations::<2>()
        .map(|rect| {
            let top_left = [min(rect[0][0], rect[1][0]), min(rect[0][1], rect[1][1])];
            let bottom_right = [max(rect[0][0], rect[1][0]), max(rect[0][1], rect[1][1])];
            [top_left, bottom_right]
        })
        .sorted_by(|r1, r2| {
            rect_area(r2).cmp(&rect_area(r1))
        })
        .collect_vec();
    
    // for each rectangle in descending area,
    // check for any lines that lie within the rectangle.
    // This is essentially the clipping algorithm for a
    // general polygon.

    'rect_loop:
    for rect in rects.iter() {
        for (col, start_row, end_row) in vertical_lines.iter() {
            let col = *col as u32;
            // line outside rectangle by column
            if col <= rect[0][1] || col >= rect[1][1] {
                continue;
            }

            // line outside rectangle by row
            if *start_row >= rect[1][0] as usize || *end_row <= rect[0][0] as usize {
                continue;
            }

            continue 'rect_loop;
        }

        let area = rect_area(rect) as u32;
        max_area = max(max_area, area);
        break;
    }

    max_area
}

fn part2_doesnt_work(points: &[[u32; 2]], debug: bool) -> u32 {
    let bounding_box = points.iter().fold([0u32; 2], |acc, p| {
        [max(acc[0], p[0]), max(acc[1], p[1])]
    });

    let bounding_box = bounding_box.map(|v| v + 1);

    let stride = (bounding_box[1] as usize).div_ceil(size_of::<usize>()) * size_of::<usize>();
    let mut bitmap = bitvec![0; (stride * bounding_box[0] as usize)];

    println!("Bounding box: {:?}, Stride: {}", bounding_box, stride);


    for (p, q) in points.iter().tuple_windows() {
        assert!(p[0] == q[0] || p[1] == q[1], "Points {p:?} and {q:?} not on same column or row");
        
        if debug {
            println!("Drawing line from {:?} to {:?}", p, q);
        }

        if p[0] == q[0] {
            let row = p[0] as usize * stride;
            let start = row + min(p[1], q[1]) as usize;
            let end = row + max(p[1], q[1]) as usize;
            bitmap[start..=end].fill(true);
        } else {
            let col = p[1] as usize;
            let start = min(p[0], q[0]) as usize;
            let end = max(p[0], q[0]) as usize;
            for row in start..=end {
                bitmap.set(row * stride + col, true);
            }
        }
    }

    if debug {
        for row in 0..bounding_box[0] as usize {
            for col in 0..bounding_box[1] as usize {
                if bitmap[row * stride + col] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // scanline fill
    println!("Performing scanline fill");
    for row in 0..bounding_box[0] as usize {
        let mut row_slice = &mut bitmap[row * stride..(row + 1) * stride];
        loop {
            let start = row_slice.first_one();
            if start.is_none() {
                break;
            }
            let start = start.unwrap() + 1;
            row_slice = &mut row_slice[start..];
            let Some(end) = row_slice.first_one() else {
                panic!("Line at row {} does not terminate", row);
            };

            row_slice[..end].fill(true);

            row_slice = &mut row_slice[(end + 1)..];
        }
    }

    if debug {
        println!("Filled bitmap:");
        for row in 0..bounding_box[0] as usize {
            for col in 0..bounding_box[1] as usize {
                if bitmap[row * stride + col] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    println!("Finding largest filled rectangle");
    points.iter()
        .array_combinations::<2>()
        .filter(|[p, q]| {
            let top_left = [min(p[0], q[0]), min(p[1], q[1])];
            let bottom_right = [max(p[0], q[0]), max(p[1], q[1])];
            // check if all bits in the rectangle defined by top_left and bottom_right are set
            for row in top_left[0] as usize..=bottom_right[0] as usize {
                let row_start = row * stride + top_left[1] as usize;
                let row_end = row * stride + bottom_right[1] as usize;
                if !bitmap[row_start..=row_end].all() {
                    return false;
                }
            }
            true
        })
        .map(|[p, q]| {
            (0..2).map(|i| (p[i].abs_diff(q[i]) + 1) as u32).product::<u32>()
        })
        .max()
        .unwrap()
}