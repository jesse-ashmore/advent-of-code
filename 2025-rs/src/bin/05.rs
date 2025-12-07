use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let fresh: Vec<RangeInclusive<usize>> = p1.lines().map(parse_ingredient).collect_vec();
    let available = p2
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let part_1 = available
        .iter()
        .filter(|&ing| fresh.iter().any(|range| range.contains(ing)))
        .count();
    let part_2 = merge_intervals(&fresh)
        .iter()
        .map(|r| (r.end() - r.start()) + 1)
        .sum::<usize>();
    (Some(part_1 as u64), Some(part_2 as u64))
}

fn parse_ingredient(line: &str) -> RangeInclusive<usize> {
    let (a, b) = line.split_once("-").unwrap();

    a.parse().unwrap()..=b.parse().unwrap()
}

fn merge_intervals(intervals: &[RangeInclusive<usize>]) -> Vec<RangeInclusive<usize>> {
    let mut final_ranges = Vec::new();

    let sorted = intervals.iter().sorted_by_key(|r| r.start()).collect_vec();

    let mut new_range = (*sorted.first().unwrap()).clone();
    for check in &sorted[1..] {
        if check.start() > new_range.end() {
            final_ranges.push(new_range.clone());
            new_range = (*check).clone();
        } else {
            new_range = *new_range.start()..=*new_range.end().max(check.end());
        }
    }

    final_ranges.push(new_range);

    final_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(3), Some(14)));
    }
}
