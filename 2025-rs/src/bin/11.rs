use std::collections::HashMap;

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
    let devices = parse_devices(input);
    let you = devices.iter().find(|d| d.name == "you").unwrap().id;
    let out = devices.len();

    let mut memo: HashMap<usize, usize> = HashMap::new();
    let part_1 = get_paths_from(&devices, you, out, &mut memo);

    let svr = devices.iter().find(|d| d.name == "svr").unwrap().id;
    let dac = devices.iter().find(|d| d.name == "dac").unwrap().id;
    let fft = devices.iter().find(|d| d.name == "fft").unwrap().id;

    let mut memo_2: HashMap<(usize, (bool, bool)), usize> = HashMap::new();
    let part_2 = get_fft_dacs_from(&devices, svr, out, (dac, fft), (false, false), &mut memo_2);

    (Some(part_1), Some(part_2))
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
    total
}

fn get_fft_dacs_from(
    devices: &[Device],
    from: usize,
    to: usize,
    find: (usize, usize),
    found: (bool, bool),
    memo: &mut HashMap<(usize, (bool, bool)), usize>,
) -> usize {
    if let Some(cached) = memo.get(&(from, found)) {
        return *cached;
    }

    if from == to {
        if found.0 && found.1 {
            return 1;
        }
        return 0;
    }
    let dac_found = found.0 || from == find.0;
    let fft_found = found.1 || from == find.1;

    let mut total = 0;
    for next in &devices[from].outputs {
        total += get_fft_dacs_from(devices, *next, to, find, (dac_found, fft_found), memo);
    }

    memo.insert((from, found), total);
    total
}

fn parse_devices(input: &str) -> Vec<Device> {
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

    devices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(5), Some(2)));
    }
}
