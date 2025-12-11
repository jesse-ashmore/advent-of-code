use std::{collections::HashMap, hash::Hash, mem, process::id};

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Device {
    id: usize,
    name: String,
    outputs: Vec<usize>,
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    // Build nodes with usize IDs and connectors
    // Store in array and traverse this way
    // BFS, avoiding seen paths *within current traversal*
    let (devices, out, you) = dbg!(parse_devices(input));
    let mut memo: HashMap<usize, usize> = HashMap::new();
    let part_1 = get_paths_from(&devices, you, out, &mut memo);

    (Some(part_1), None)
}

fn get_paths_from(
    devices: &[Device],
    from: usize,
    to: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(cached) = memo.get(&from) {
        return *cached;
    }

    if from == to {
        return 1;
    }

    let mut total = 0;
    for next in &devices[from].outputs {
        total += get_paths_from(devices, *next, to, memo);
    }

    memo.insert(from, total);
    return total;
}

fn parse_devices(input: &str) -> (Vec<Device>, usize, usize) {
    let mut id_map = HashMap::new();
    let input_parts = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .sorted_by_key(|(name, _)| *name)
        .collect_vec();

    input_parts
        .iter()
        .map(|(name, _)| name)
        .chain(["out"].iter())
        .enumerate()
        .for_each(|(id, name)| {
            id_map.insert(*name, id);
        });

    let devices = input_parts
        .iter()
        .map(|(name, rhs)| Device {
            id: *id_map.get(*name).unwrap(),
            name: name.to_string(),
            outputs: rhs
                .split(" ")
                .map(|p| *id_map.get(p).unwrap())
                .collect_vec(),
        })
        .collect_vec();

    (
        devices,
        *id_map.get("out").unwrap(),
        *id_map.get("you").unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(5), None));
    }
}
