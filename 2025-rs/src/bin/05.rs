use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let fresh: Vec<RangeInclusive<usize>> =
        dbg!(p1.lines().map(|line| parse_ingredient(line)).collect_vec());
    let available = dbg!(p2
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec());

    let part_1 = available
        .iter()
        .filter(|&ing| fresh.iter().any(|range| range.contains(ing)))
        .count();
    (Some(part_1 as u64), None)
}

fn parse_ingredient(line: &str) -> RangeInclusive<usize> {
    let (a, b) = line.split_once("-").unwrap();

    a.parse().unwrap()..=b.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(3), None));
    }
}
