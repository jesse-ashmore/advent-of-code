use itertools::Itertools;

advent_of_code::solution!(3);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let banks = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Couldn't parse input digit") as u64)
                .collect_vec()
        })
        .collect_vec();

    let part_1 = banks
        .iter()
        .map(|bank| largest_fixed_joltage(bank, 2))
        .sum();
    let part_2 = banks
        .iter()
        .map(|bank| largest_fixed_joltage(bank, 12))
        .sum();
    (Some(part_1), Some(part_2))
}

fn largest_fixed_joltage(bank: &[u64], num_digits: usize) -> u64 {
    let mut joltage = 0;
    let mut last_pos = None;

    for d in 1..=num_digits {
        let search_to = bank.len() - (num_digits - d);
        last_pos = Some(
            (search_to - 1)
                - bank[last_pos.map_or(0, |v| v + 1)..search_to]
                    .iter()
                    .rev()
                    .position_max()
                    .expect("Couldn't find first element"),
        );
        joltage = (joltage * 10) + bank[last_pos.unwrap_or(0)];
    }

    joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(357), Some(3121910778619)));
    }
}
