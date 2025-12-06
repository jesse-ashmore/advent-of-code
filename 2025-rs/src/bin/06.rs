use std::str::Chars;

use advent_of_code::Grid;
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(6);

enum Operation {
    Add,
    Multiply,
}

struct Equation {
    terms: Vec<String>,
    op: Operation,
}

impl Equation {
    fn from_row(row: &Vec<String>) -> Equation {
        let op = match row.last().unwrap().chars().next() {
            Some('+') => Operation::Add,
            Some('*') => Operation::Multiply,
            _ => panic!("Unexpected value for operation"),
        };

        let terms = row
            .iter()
            .take(row.len() - 1)
            .map(|item| item.parse().expect("Couldn't parse term"))
            .collect_vec();

        Equation { op, terms }
    }

    fn apply(&self) -> u64 {
        match self.op {
            Operation::Add => self
                .terms
                .iter()
                .map(|t| t.trim().parse::<u64>().unwrap())
                .sum(),
            Operation::Multiply => self
                .terms
                .iter()
                .map(|t| t.trim().parse::<u64>().unwrap())
                .product(),
        }
    }

    fn apply_ceph(&self) -> u64 {
        let mut re_read = Vec::new();
        let extent = self
            .terms
            .iter()
            .map(|t| t.len())
            .max()
            .expect("Couldn't derive max extent");

        /*
         * Read through each term in reverse columnar order.
         * e.g.
         * [
         *   "123",
         *   "45 ",
         *   " 1 "
         * ] => [3, 251, 14]
         */
        for index in (0..extent).rev() {
            let mut new_term: u64 = 0;
            for term in &self.terms {
                if let Some(c) = term.chars().nth(index) {
                    if let Some(digit) = c.to_digit(10) {
                        new_term = (new_term * 10) + (digit as u64);
                    }
                }
            }
            re_read.push(new_term);
        }
        match self.op {
            Operation::Add => re_read.iter().sum(),
            Operation::Multiply => re_read.iter().product(),
        }
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let equations = parse_equations(input);
    (
        Some(equations.iter().map(Equation::apply).sum()),
        Some(equations.iter().map(Equation::apply_ceph).sum()),
    )
}

fn parse_equations(input: &str) -> Vec<Equation> {
    // Find the bounds for all terms for a given equation by using the operations
    // to denote the start of each.
    let op_line = input
        .lines()
        .last()
        .expect("Couldn't find last line with operations");
    let eq_positions = op_line.chars().positions(|c| c != ' ').collect_vec();

    // Build up a vector of equation terms.
    let mut parsed: Vec<Vec<String>> = vec![Vec::new(); eq_positions.len()];
    for row in input.lines().take(input.lines().count() - 1) {
        for (eqn, (start, end)) in eq_positions
            .iter()
            // Ensure we grab the last term too.
            .chain([op_line.len() + 1].iter())
            // Use pairs of operation positions to grab the equation-aligned terms.
            .tuple_windows()
            .enumerate()
        {
            parsed[eqn].push(row.chars().skip(*start).take((end - start) - 1).join(""));
        }
    }

    // Add each operation to the end of the equation vector.
    eq_positions
        .iter()
        .map(|pos| op_line.chars().nth(*pos).unwrap())
        .enumerate()
        .for_each(|(eqn, op)| parsed[eqn].push(op.to_string()));

    parsed
        .iter()
        .map(|row| Equation::from_row(row))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(4277556), Some(3263827)));
    }
}
