use core::panic;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::env::args;
use std::{any, fmt, u32};

use bitvec::prelude::*;
use nalgebra::{DMatrix, DVector, LU};
use neerajsi::read_stdin_input;
use num_rational::{Rational, Rational32};

struct Machine {
    lights: u16,
    light_count: u8,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    pub fn parse(line: &str) -> Self {
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

    }
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

    let debug1 = debug || args().any(|a| a == "debug1");
    let debug2 = debug || args().any(|a| a == "debug2");


    let machines = input.lines()
        .map(|line| Machine::parse(line.trim_ascii()))
        .collect::<Vec<Machine>>();

    let machine_count = machines.len();
    println!("Machine count: {}", machine_count);

    let max_buttons = machines.iter().map(|m| m.buttons.len()).max().unwrap();
    println!("Max buttons: {}", max_buttons);

    let joltages_match_buttons = machines.iter()
        .filter(|m| m.joltages.len() == m.buttons.len())
        .count();

    println!("Machines with matching joltages and buttons count: {}", joltages_match_buttons);

    let joltages_match_lights = machines.iter()
        .filter(|m| m.joltages.len() == m.light_count as usize)
        .count();

    println!("Machines with matching joltages and lights count: {}", joltages_match_lights);

    let buttons_le_joltages = machines.iter()
        .filter(|m| m.buttons.len() <= m.joltages.len())
        .count();

    println!("Machines with buttons count less or equal to joltages count: {}", buttons_le_joltages);
    let p1 = part1(&machines, debug1);
    println!("Part 1: {}", p1);

    let p2 = part2(&machines, debug2);
    println!("Part 2: {}", p2);

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

fn part2(machines: &[Machine], debug: bool) -> u32 {
    let mut invertible_machines = 0;
    let mut constraint_sizes = BTreeMap::new();
    let mut machine_costs = 0usize;
    let mut free_variable_space_max = 0usize;
    let mut free_variable_space_total = 0usize;
    for (machine_no, machine) in machines.iter().enumerate() {
        
        let nrows = machine.joltages.len();
        let ncols = machine.buttons.len();

        let matrix: DMatrix<Rational32> = DMatrix::from_iterator(
            nrows,
            ncols,
            (0..ncols).flat_map(|c| {
                  (0..nrows).map(move |r| {
                    if (machine.buttons[c] & (1u16 << r as u16)) != 0 {
                        Rational32::from_integer(1)
                    } else {
                        Rational32::from_integer(0)
                    }
                })
            })
        );

        let mut matrix = matrix.insert_column(ncols, Rational32::ZERO);
        for i in 0..nrows {
            matrix[(i, ncols)] = (machine.joltages[i] as i32).into();
        }

        let orig_matrix = matrix.clone();

        if debug {
            println!("Machine {}:\n{}", machine_no, machine);
            println!("Matrix:\n{}", matrix);
        }

        let mut all_pivoted = true;
        let mut reduced_rows = 0;
        for c in 0..ncols {
            // find a row with a 1 in this column
            let cur_rows = matrix.nrows();
            let pivot_row = (reduced_rows..cur_rows).find(|&r| matrix[(r, c)] != Rational32::ZERO);
            if let Some(pivot_row) = pivot_row {
                // swap to top
                if pivot_row != c {
                    matrix.swap_rows(pivot_row, c);
                }

                if matrix[(c, c)] != Rational32::ONE {
                    let denom = matrix[(c, c)];
                    for cc in c..=ncols {
                        matrix[(c, cc)] = matrix[(c, cc)] / denom;
                    }
                }

                // eliminate other rows
                for r in 0..cur_rows {
                    if r != c && matrix[(r, c)] != Rational32::ZERO {
                        let sign = matrix[(r, c)];
                        for cc in 0..=ncols {
                            matrix[(r, cc)] = (matrix[(r, cc)] - sign * matrix[(c, cc)]);
                        }
                    }
                }

                reduced_rows += 1;

            } else {
                // no pivot in this column...
                // insert a zero row if we have fewer rows than columns
                matrix = matrix.insert_row(c, Rational32::ZERO);
                reduced_rows += 1;
                all_pivoted = false;
            }
        }

        if debug {
            println!("Reduced Matrix:\n{}", matrix);
        }

        let machine_cost;;
        if all_pivoted {
            for r in 0..ncols {
                for c in 0..ncols {
                    match r == c {
                        false => {
                            if matrix[(r, c)] != Rational32::ZERO {
                                panic!("Non-zero entry in reduced matrix at ({}, {})", r, c);
                            }
                        }
                        true => {
                            if matrix[(r, c)] != Rational32::ONE {
                                panic!("Non-one entry in reduced matrix at ({}, {})", r, c);
                            }
                        }
                    }
                }
            }

            let solution = matrix.column_part(ncols, ncols);
            if (debug) {
                println!("Solution: {}", solution);
            }

            machine_cost = solution.iter().map(|x| {
                assert!(x.is_integer());
                assert!(*x >= Rational32::ZERO);
                x.to_integer() as usize
            }).sum();

            invertible_machines += 1;
        } else {
            if debug {
                let min_nonzero_column_count = (0..ncols)
                    .filter(|&r| matrix[(r, r)] != Rational32::ZERO)
                    .filter_map(|r| {
                        let count = (0..ncols).filter(|&c| matrix[(r, c)] != Rational32::ZERO).count();
                        if count > 1 {
                            Some(count)
                        } else {
                            None
                        }
                }).min().unwrap();
    
                println!("Machine {} is not invertible. Min non-zero column count in pivoted rows: {}", 
                    machine_no, min_nonzero_column_count);

                *constraint_sizes.entry(min_nonzero_column_count)
                    .or_insert(0usize) += 1;
            }

            let max_joltage = machine.joltages.iter().max().unwrap();

            let mut variable_ranges = vec![(0, *max_joltage as i32, 1); ncols];
            let var_range = |var: usize, variable_ranges: &[(i32, i32, i32)]| {
                let (min, max, denom) = variable_ranges[var];
                (Rational32::new(min, denom), Rational32::new(max, denom))
            };
            
            let mut improve_constraints = |matrix: &nalgebra::Matrix<num_rational::Ratio<i32>, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<num_rational::Ratio<i32>, nalgebra::Dyn, nalgebra::Dyn>>| {
                let mut improved_constraint = true;
                while improved_constraint {
                    improved_constraint = false;
                    for var in 0..ncols {
                        if variable_ranges[var].0 == variable_ranges[var].1 {
                            // already fixed
                            continue;
                        }
                        
                        for r in 0..matrix.nrows() {
                            // Row doesn't involve this column
                            if matrix[(r, var)] == Rational32::ZERO {
                                continue;
                            }

                            let rhs = matrix[(r, ncols)];
                            let coeff = matrix[(r, var)];
                            let (sum_other_mins, sum_other_maxes) = (0..ncols)
                                    .filter(|&c| c != var && matrix[(r, c)] != Rational32::ZERO)
                                    .map(|c| {
                                        let coeff = matrix[(r, c)];
                                        // We want to minimize each term to
                                        // produce the maximum possible value for var
                                        let range = var_range(c, &variable_ranges);
                                        let (min_extremum, max_extremum) = 
                                            if coeff > Rational32::ZERO {
                                                (range.0, range.1)
                                            } else {
                                                (range.1, range.0)
                                            };

                                        (coeff * min_extremum, coeff * max_extremum)
                                    })
                                    .fold((Rational32::ZERO, Rational32::ZERO), 
                                    |(acc_min, acc_max), (min_val, max_val)|  {
                                        (acc_min + min_val, acc_max + max_val)
                                    });

                            let max_extremum = (rhs - sum_other_mins) / coeff;
                            let min_extremum = (rhs - sum_other_maxes) / coeff;
                            let new_upper = max_extremum.to_integer();
                            let new_lower = min_extremum.to_integer();
                            let (new_upper, new_lower) = if coeff > Rational32::ZERO {
                                (new_upper, new_lower)
                            } else {
                                // Dividing by a negative flips the inequality
                                (new_lower, new_upper)
                            };

                            if new_lower > variable_ranges[var].0 {
                                if debug {
                                    println!("Variable {} lower bound improved from {} to {} based on row {}", 
                                        var, variable_ranges[var].0, new_lower, r);
                                }
                                variable_ranges[var].0 = new_lower;
                                improved_constraint = true;
                            }

                            if new_upper < variable_ranges[var].1 {
                                if debug {
                                    println!("Variable {} upper bound improved from {} to {} based on row {}", 
                                        var, variable_ranges[var].1, new_upper, r);
                                }
                                variable_ranges[var].1 = new_upper;
                                improved_constraint = true;
                            }
                        }
                    }
                }
            };

            if debug { println!("Improving constraints (original)..."); }
            improve_constraints(&orig_matrix);

            if debug { println!("Improving constraints (reduced)..."); }
            improve_constraints(&matrix);

            if debug {
                println!("Variable ranges after constraint propagation:");
                for (i, _) in variable_ranges.iter().enumerate() {
                    let range = var_range(i, &variable_ranges);
                    println!("  Var {}: [{}, {}]", i, range.0, range.1);
                }
            }

            let free_variables = (0..ncols).filter(|&var| {
                matrix[(var, var)] == Rational32::ZERO
            }).collect::<Vec<usize>>();

            let dependent_variables = (0..ncols).filter(|&var| {
                let coeff = matrix[(var, var)];
                assert!(coeff == Rational32::ONE || coeff == Rational32::ZERO);
                matrix[(var, var)] != Rational32::ZERO
            }).collect::<Vec<usize>>();

            if debug {
                let free_variable_space = free_variables.iter().map(|&var| {
                    let (min, max, _) = variable_ranges[var];
                    (max - min + 1) as usize
                }).product::<usize>();

                println!("Free variables: {:?} (of {})", free_variables, ncols);
                println!("Free variable space size: {}", free_variable_space);
            
                free_variable_space_max = free_variable_space_max.max(free_variable_space);
                free_variable_space_total += free_variable_space;
            }

            let mut assignment = DVector::from_element(ncols, Rational32::ZERO);
            for &v in &free_variables {
                // set all to minimum
                assignment[v] = Rational32::from_integer(variable_ranges[v].0);
            }
            
            let mut best_cost = usize::MAX;
            while (assignment[free_variables[0]] <= Rational32::from_integer(variable_ranges[free_variables[0]].1)) {
                
                // compute dependent variables
                let mut valid = true;
                for &v in &dependent_variables {
                    let rhs = matrix[(v, ncols)];
                    let sum_other_vars: Rational32 = (0..ncols)
                        .filter(|&c| c != v)
                        .map(|c| {
                            matrix[(v, c)] * assignment[c]
                        })
                        .sum();
                    
                    let value = rhs - sum_other_vars;
                    if !value.is_integer() || value < Rational32::ZERO {
                        valid = false;
                        break;
                    }

                    assignment[v] = value;
                }

                if valid {
                    let cost = assignment.iter().map(|x| x.to_integer() as usize).sum::<usize>();
                    if cost < best_cost {
                        if debug {
                            println!("New best cost {} with assignment {}", cost, assignment);
                        }

                        best_cost = cost;
                    }
                }

                // increment the free variables
                let mut overflow = true;
                for &v in free_variables.iter().rev() {
                    assignment[v] = assignment[v] + Rational32::from_integer(1);
                    if assignment[v] > Rational32::from_integer(variable_ranges[v].1) {
                        // overflow, reset to min and carry
                        assignment[v] = Rational32::from_integer(variable_ranges[v].0);
                    } else {
                        overflow = false;
                        break;
                    }
                }

                if overflow {
                    break;
                }

            }

            machine_cost = best_cost;

        }


        if debug {
            println!("Machine {} cost: {}", machine_no, machine_cost);
            println!("-----------------------------");
        }
        
        machine_costs += machine_cost;
    }

    println!("Invertible machines: {}", invertible_machines);
    if debug {
        println!("Constraint sizes for non-invertible machines:");
        for (size, count) in constraint_sizes {
            println!("  Size {}: {} machines", size, count);
        }

        println!("Free variable space max size: {}", free_variable_space_max);
        println!("Free variable space total size: {}", free_variable_space_total);
    }

    machine_costs as u32
}

mod tests {
    #[test]
    fn part2_one_machine() {
        let machine = super::Machine::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let machines = vec![machine];
        let result = super::part2(&machines, true);
        assert_eq!(result, 0);
    }
}