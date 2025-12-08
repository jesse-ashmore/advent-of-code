use std::collections::HashMap;

use advent_of_code::Pairs;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Debug)]
struct Junction(i32, i32, i32);

pub fn solve(input: &str) -> (Option<i32>, Option<i32>) {
    let junction_boxes = parse_junctions(input);

    dbg!(build_circuits_by_closest(&junction_boxes));
    (None, None)
}

fn parse_junctions(input: &str) -> Vec<Junction> {
    input
        .lines()
        .map(|l| {
            let jnct = l
                .split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32, i32)>()
                .expect("Expected 3 values in junction location");
            Junction(jnct.0, jnct.1, jnct.2)
        })
        .collect_vec()
}

// fn build_circuit_by_key<'a, F>(boxes: &'a Vec<Junction>, comparator: F)
// where
//     F: Fn(&'a Junction, &'a [Junction]) -> &'a Junction,
// {

// }

fn build_circuits_by_closest<'a>(boxes: &'a Vec<Junction>) -> HashMap<u16, usize> {
    let mut circuit_assignment = [0u16; 1000];
    let boxes_with_index = boxes.iter().enumerate().collect_vec();
    (0..boxes.len()).for_each(|i| circuit_assignment[i] = (i + 1) as u16);

    for ((pos_a, a), (pos_b, b)) in boxes_with_index
        .pairs()
        .sorted_by_key(|(a, b)| dist_euclid(&(a.1, b.1)))
    {
        // dbg!((a, b));
        let a_circuit = circuit_assignment[*pos_a];
        let b_circuit = circuit_assignment[*pos_b];
        if a_circuit != b_circuit {
            // Combine two circuits (B -> A)
            circuit_assignment
                .iter_mut()
                .filter(|jnc| **jnc == b_circuit)
                .for_each(|to_combine| *to_combine = a_circuit);
        }
        // dbg!(&circuit_assignment[0..boxes.len()]);
        // break;
    }
    circuit_assignment[0..boxes.len()].iter().counts_by(|a| *a)
}

fn dist_euclid((a, b): &(&Junction, &Junction)) -> u32 {
    ((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2))
        .isqrt()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(40), None));
    }
}
