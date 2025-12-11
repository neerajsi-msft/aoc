use core::panic;
use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::{fmt, u32};

use bitvec::prelude::*;
use bitvec::array::BitArray;
use neerajsi::read_stdin_input;

struct Machine {
    lights: u16,
    light_count: u8,
    buttons: Vec<u16>,
    joltages: Vec<u32>,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.light_count {
            if (self.lights & (1u16 << i as u32)) != 0 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "]")?;
        for button in &self.buttons {
            write!(f, " (")?;
            let mut first = true;
            for i in 0..self.light_count {
                if (button & (1u16 << i as u32)) != 0 {
                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;

                    write!(f, "{}", i)?;
                }
            }
            write!(f, ")")?;
        }
        write!(f, " {{")?;
        let mut first = true;
        for joltage in &self.joltages {
            if !first {
                write!(f, ",")?;
            }
            first = false;
            write!(f, "{}", joltage)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();

    let debug = args().any(|a| a == "debug");

    let machines = input.lines()
        .map(|line| {
            let line = line.trim_ascii();
            let mut machine = Machine {
                lights: 0,
                light_count: 0,
                buttons: Vec::new(),
                joltages: Vec::new(),
            };

            let (lights_part, rest) = line.split_once(' ').unwrap();
            let lights_part = sscanf::scanf!(lights_part, "[{}]", str).unwrap();
            machine.light_count = lights_part.len().try_into().unwrap();
            machine.lights = lights_part.chars().enumerate().fold(0u16, |acc, (i, c)| {
                match c {
                    '#' => acc | 1u16.checked_shl(i.try_into().unwrap()).unwrap(),
                    '.' => acc,
                    _ => panic!("Invalid light character: {}", c),
                }
            });

            let (buttons_part, joltages_part) =  rest.split_once('{').unwrap();
            machine.buttons = buttons_part.trim_end()
                .split(' ')
                .map(|b| 
                    sscanf::scanf!(b.trim(), "({})", str).unwrap()
                        .split(',')
                        .map(|num| num.parse::<u16>().unwrap())
                        .fold(0u16, |acc, n| acc | 1u16.checked_shl(n.try_into().unwrap()).unwrap())
                )
                .collect();

            machine.joltages = joltages_part.trim_end_matches('}')
                .split(',')
                .map(|j| j.trim().parse().expect(format!("Invalid joltage {}", j).as_str()))
                .collect();
            
            machine
        })
        .collect::<Vec<Machine>>();

    let machine_count = machines.len();
    println!("Machine count: {}", machine_count);

    let max_buttons = machines.iter().map(|m| m.buttons.len()).max().unwrap();
    println!("Max buttons: {}", max_buttons);

    let joltages_match_buttons = machines.iter()
        .filter(|m| m.joltages.len() == m.buttons.len())
        .count();

    println!("Machines with matching joltages and buttons count: {}", joltages_match_buttons);

    let p1 = part1(&machines, debug);
    println!("Part 1: {}", p1);

    /*
    if debug {
        for machine in &machines {
            println!("{}", machine);
        }
    }
    */

}

fn part1(machines: &[Machine], debug: bool) -> u32 {


    let mut cost_map = HashMap::new();
    let mut bfs_queue = VecDeque::new();
    let mut in_queue: BitVec<usize, Lsb0> = BitVec::with_capacity(16384);
    let mut machine_costs = 0;
    for (machine_number, machine) in machines.iter().enumerate() {
        cost_map.clear();
        bfs_queue.clear();
        in_queue.clear();
        in_queue.resize(16384, false);

        let machine_cost = (|| {

            // the bfs queue contains a u16 bitmask of buttons pushed.
            // start with 1 button pushed for each button.
            for i in 0..machine.buttons.len() {
                let button_mask = 1u16.checked_shl(i.try_into().unwrap()).unwrap();
                let current_lights = machine.buttons[i];
                in_queue.set(button_mask as usize, true);
                bfs_queue.push_back((button_mask, current_lights));
            }

            let mut best_cost = u32::MAX;
            while let Some((button_mask, current_lights)) = bfs_queue.pop_front() {
                let cost = button_mask.count_ones();
                let wrong_lights = machine.lights ^ current_lights;
                if wrong_lights == 0 {
                    // found solution
                    if debug {
                        println!("Found solution for machine {}: current_lights={:016b}, button_mask={:016b}, cost={}", 
                            machine_number, current_lights, button_mask, cost);
                    }

                    return cost;
                }

                if let Some(&(complement_cost, complement_buttons)) = cost_map.get(&wrong_lights) {
                    // found a complement
                    if debug {
                        println!("Found complement for machine {}: current_lights={:016b}, wrong_lights={:016b}, button_mask={:016b}, complement_mask={:016b}, complement_cost={}", 
                            machine_number, current_lights, wrong_lights, button_mask, complement_buttons, complement_cost);
                    }

                    best_cost = best_cost.min(cost + complement_cost);
                }

                cost_map.entry(current_lights).or_insert((cost, button_mask));

                if cost >= best_cost {
                    // no need to explore further
                    continue;
                }

                // try adding each button not already pressed to the queue
                for i in 0..machine.buttons.len() {
                    let next_button_mask = 1u16 << i as u16;
                    if (button_mask & next_button_mask) == 0 {
                        let new_button_mask = button_mask | next_button_mask;
                        if !in_queue[new_button_mask as usize] {
                            in_queue.set(new_button_mask as usize, true);
                            let new_lights = current_lights ^ machine.buttons[i];
                            
                            bfs_queue.push_back((new_button_mask, new_lights));
                        }
                    }
                }
            }

            best_cost
        })();

        if debug {
            println!("Machine {} cost: {}", machine_number, machine_cost);
        }

        machine_costs += machine_cost;
    }

    machine_costs
}