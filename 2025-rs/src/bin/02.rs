use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap(); //?.parse::<u64>().ok()?;
            let end = parts.next().unwrap(); //?.parse::<u64>().ok()?;
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
        println!("{:?} {:?}", range, results);
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

    // Start with first digit in block, if under, increment
    // If over, move onto next block size
    // otherwise increment and check
    // If block size over length/2.ceil(), quit
    // Start with block size 1
    for block_size in 1..=(range.0.len() / 2) {
        // println!("{}", block_size);
        let max_for_block = max.min(
            (0..block_size)
                .into_iter()
                .map(|_| "9")
                .join("")
                .parse()
                .unwrap(),
        );
        let mut block_string = range.0.chars().take(1).join("");

        if block_size > 1 {
            block_string += &(1..block_size)
                .into_iter()
                .map(|_| "0")
                .join("")
                .to_string();
        }
        let mut block = block_string.parse::<u64>().unwrap();
        // println!("BLOCK START {}", block);
        // println!("BLOCK SIZE {}", block_size);

        // Increment digits in each block and duplicate across range

        while block <= max_for_block {
            let test: u64 = (0..(range.1.len() / block_size))
                .map(|_| block)
                .join("")
                .parse()
                .unwrap();

            // println!("TEST {}", test)
            if test > max {
                break 
            }
            if test >= min {
                if test <= max {
                    invalid.insert(test);
                }
            }

            block += 1
        }
        // loop {
        //     if min <= test {
        //         if test <= max_for_block {
        //             invalid.insert(test);
        //         } else {
        //             break;
        //         }
        //     }
        //     block += 1;
        // }
    }

    invalid
}

fn find_invalid(range: &(&str, &str)) -> HashSet<u64> {
    let mut invalid = HashSet::new();
    let min = range.0.parse::<u64>().unwrap();
    let max = range.1.parse::<u64>().unwrap();

    // Start with block size 1
    // Increment digits in each block and duplicate across range
    // Start with first digit in block, if under, increment
    // If over, move onto next block size
    // otherwise increment and check
    // If block size over length/2.ceil(), quit

    let even = range.0.len().rem_euclid(2) == 0;
    let mut block_size = if even {
        (range.0.len() / 2)
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
        ("1".to_owned() + &(1..block_size).into_iter().map(|_| "0").join(""))
            .parse::<u64>()
            .unwrap()
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
