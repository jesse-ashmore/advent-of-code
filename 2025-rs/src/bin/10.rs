use good_lp::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    start_pattern: u16,
    buttons: Vec<u16>,
    cardinal_buttons: Vec<Vec<usize>>,
    requirements: Vec<usize>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let parts = line.split(" ").collect_vec();
        let indicator_part = &parts[0][1..parts[0].len() - 1];
        let indicator_size = indicator_part.len();
        let start_pattern = indicator_part
            .chars()
            .rev()
            .map(|c| if c == '#' { 1u16 } else { 0u16 })
            .enumerate()
            .map(|(shift, v)| v << shift)
            .sum();
        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|block| {
                block[1..block.len() - 1]
                    .split(",")
                    .map(|t| 1 << ((indicator_size as u16 - t.parse::<u16>().unwrap()) - 1))
                    .sum()
            })
            .collect_vec();
        let cardinal_buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|block| {
                block[1..block.len() - 1]
                    .split(",")
                    .map(|t| t.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let requirements = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(",")
            .map(|t| t.parse().unwrap())
            .collect_vec();

        Machine {
            start_pattern,
            buttons,
            cardinal_buttons,
            requirements,
        }
    }
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let machines = input.lines().map(Machine::from_line).collect_vec();
    let part_1 = machines
        .iter()
        .map(|m| get_min_presses(m.start_pattern, &m.buttons))
        .sum();

    let part_2 = machines.iter().map(get_min_joltage_presses).sum();

    (Some(part_1), Some(part_2))
}

fn get_min_joltage_presses(machine: &Machine) -> usize {
    // Using good_lp: https://github.com/wilkotom/AdventOfCode/blob/main/rust/2025/day10/src/main.rs
    let mut variables: ProblemVariables = ProblemVariables::new();
    let presses = (0..machine.cardinal_buttons.len())
        // Each variable represents the number of times each button is pressed.
        .map(|_| variables.add(variable().min(0).integer()))
        .collect_vec();

    let mut problem = good_lp::highs(variables.minimise(presses.iter().sum::<Expression>()));
    // Each expression represents a joltage requirement.
    let mut expressions =
        vec![Expression::with_capacity(machine.cardinal_buttons.len()); machine.requirements.len()];

    // Increment each counter by the effect each button has on that counter.
    for (i, button) in machine.cardinal_buttons.iter().enumerate() {
        for cardinal in button {
            expressions[*cardinal] += presses[i]; // This is a variable we are adding to the expression.
        }
    }

    // Require that each joltage expression match what the machine needs to start.
    for (exp, joltage) in expressions.into_iter().zip(&machine.requirements) {
        problem.add_constraint(exp.eq(*joltage as f64));
    }

    // Evaluate the problem we have set up (minimum number of button presses to achieve the joltage requirements).
    let solution = problem.solve().expect("Failed to solve problem");
    presses
        .iter()
        .map(|press| solution.value(*press))
        .sum::<f64>() as usize
}

fn get_min_presses(end: u16, buttons: &[u16]) -> usize {
    let mut seen: HashMap<u16, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    for button in buttons.iter() {
        queue.push_front((*button, 1usize));
    }

    while let Some(state) = queue.pop_back() {
        seen.insert(state.0, state.1);

        if state.0 == end {
            return state.1;
        }
        for button in buttons.iter() {
            let next = state.0 ^ *button;
            if let Some(previous) = seen.get(&next) {
                if *previous <= state.1 + 1 {
                    continue;
                }
            }
            queue.push_back((state.0 ^ *button, state.1 + 1));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(7), Some(33)));
    }
}
