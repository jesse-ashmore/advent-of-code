use std::{collections::VecDeque, os::macos::raw::stat};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    start_pattern: Vec<bool>,
    indicators: usize,
    buttons: Vec<Vec<usize>>,
    requirements: Vec<usize>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let parts = line.split(" ").collect_vec();
        let indicator_part = &parts[0][1..parts[0].len()];
        let start_pattern = indicator_part.chars().map(|c| c == '#').collect_vec();
        let buttons = parts[1..parts.len() - 1]
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
            start_pattern: start_pattern.clone(),
            indicators: start_pattern.len(),
            buttons,
            requirements,
        }
    }
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let machines = input.lines().map(Machine::from_line).collect_vec();
    let part_1 = machines
        .iter()
        .map(|m| dbg!(get_min_presses(&m.start_pattern, &m.buttons)))
        .sum();

    (part_1, None)
}

struct State {
    indicators: Vec<bool>,
    presses: usize,
    last_button: usize,
}

fn get_min_presses(end: &[bool], buttons: &[Vec<usize>]) -> Option<usize> {
    let mut min_presses: Option<usize> = None;
    let start = end.iter().map(|_| false).collect_vec();
    let mut queue: VecDeque<State> = VecDeque::new();
    for (idx, button) in buttons.iter().enumerate() {
        queue.push_front(State {
            indicators: press(start.clone(), &button),
            presses: 1,
            last_button: idx,
        });
    }

    while let Some(state) = queue.pop_back() {
        if min_presses.is_some() && min_presses.unwrap() <= state.presses {
            continue;
        }
        if state.indicators == end {
            min_presses = Some(state.presses);
        }
        for (idx, button) in buttons.iter().enumerate() {
            if state.last_button == idx {
                continue;
            }
            queue.push_back(State {
                indicators: press(state.indicators.clone(), &button),
                presses: state.presses + 1,
                last_button: idx,
            });
        }
    }
    min_presses
}

fn press(input: Vec<bool>, button: &[usize]) -> Vec<bool> {
    let mut new_state = input.clone();
    for change in button {
        new_state[*change] = !new_state[*change];
    }
    new_state.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(7), None));
    }
}
