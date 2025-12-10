use std::{collections::VecDeque, os::macos::raw::stat};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    start_pattern: u16,
    buttons: Vec<u16>,
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
        let requirements = parts[parts.len() - 1][1..parts[parts.len() - 1].len() - 1]
            .split(",")
            .map(|t| t.parse().unwrap())
            .collect_vec();

        Machine {
            start_pattern,
            buttons,
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

    // https://docs.rs/good_lp/latest/good_lp/ ?
    // https://docs.rs/highs/latest/highs/
    // Using micro-lp: https://github.com/timvisee/advent-of-code-2025/blob/master/day10b/src/main.rs
    // Using good_lp: https://github.com/wilkotom/AdventOfCode/blob/main/rust/2025/day10/src/main.rs

    (Some(part_1), None)
}

struct State {
    indicators: u16,
    presses: usize,
}

fn get_min_presses(end: u16, buttons: &[u16]) -> usize {
    let mut queue = VecDeque::new();
    for (_, button) in buttons.iter().enumerate() {
        queue.push_front((*button, 1usize));
    }

    while let Some(state) = queue.pop_front() {
        if state.0 == end {
            return state.1;
        }
        for (_, button) in buttons.iter().enumerate() {
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
        assert_eq!(result, (Some(7), None));
    }

    #[test]
    fn test_press() {
        assert_eq!(0b0110, press(0, 0b0110));
        assert_eq!(0, press(0b0110, 0b0110));
        assert_eq!(0b0100, press(0b010, 0b0110));
    }
}
