use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap();
            let end = parts.next().unwrap();
            Some((start, end))
        })
        .collect();

    let mut invalid = HashSet::new();
    for range in &ranges {
        let results = find_invalid(&range);
        // println!("{:?} {:?}", range, results);
        results.iter().for_each(|id| {
            invalid.insert(id.clone());
        });
    }

    let mut invalid_2 = HashSet::new();
    for range in ranges {
        let results = find_invalid_any(&range);
        // println!("{:?} {:?}", range, results);
        results.iter().for_each(|id| {
            invalid_2.insert(id.clone());
        });
    }

    // println!("{:?}", &ranges);
    (
        Some(invalid.iter().sum::<u64>()),
        Some(invalid_2.iter().sum::<u64>()),
    )
}

fn find_invalid_any(range: &(&str, &str)) -> HashSet<u64> {
    let mut invalid = HashSet::new();
    let min = range.0.parse::<u64>().unwrap();
    let max = range.1.parse::<u64>().unwrap();

    // Start with block size 1
    // Increment digits in each block and duplicate across range
    // Start with first digit in block, if under, increment
    // If over, move onto next block size
    // otherwise increment and check
    // If block size over length/2.ceil(), quit
    for block_size in 1..=(range.0.len() / 2) + 1 {
        let max_for_block = max.min((0..block_size).map(|idx| 9 * 10u64.pow(idx as u32)).sum());
        let mut block = 1;

        if block_size > 1 {
            block *= 10u64.pow((block_size - 1) as u32);
        }

        // println!("BLOCK START {}", block);
        // println!("BLOCK SIZE {}", block_size);

        while block <= max_for_block {
            for repeats in (range.0.len() / block_size).max(2)..(range.1.len() / block_size) + 1 {
                let test: u64 = (0..repeats)
                    .map(|idx| block as u64 * 10u64.pow(block_size as u32 * idx as u32))
                    .sum();

                if test > max {
                    break;
                }
                // println!("TEST {}", test);
                if test >= min {
                    if test <= max {
                        invalid.insert(test);
                    }
                }
            }
            block += 1
        }
    }

    invalid
}

fn find_invalid(range: &(&str, &str)) -> HashSet<u64> {
    let mut invalid = HashSet::new();
    let min = range.0.parse::<u64>().unwrap();
    let max = range.1.parse::<u64>().unwrap();

    let even = range.0.len().rem_euclid(2) == 0;
    let block_size = if even {
        range.0.len() / 2
    } else {
        (range.0.len() / 2) + 1
    };

    let mut block = if even {
        range
            .0
            .chars()
            .take(block_size)
            .join("")
            .parse::<u64>()
            .unwrap()
    } else {
        1 * 10u64.pow((block_size - 1) as u32)
    };
    // let mut test = ;
    loop {
        let test = (format!("{}{}", block, block).parse::<u64>().unwrap());
        if min <= test {
            if test <= max {
                invalid.insert(test);
            } else {
                break;
            }
        }
        block += 1;
    }

    invalid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(1227775554), Some(4174379265)));
    }
}
