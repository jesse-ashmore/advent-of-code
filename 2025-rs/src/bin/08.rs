use advent_of_code::Pairs;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Debug)]
struct Junction(u64, u64, u64);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let junction_boxes = parse_junctions(input);
    let p1_pairs = if junction_boxes.len() == 20 { 10 } else { 1000 };

    let mut circuit_assignment = [0u16; 1000];
    let boxes_with_index = junction_boxes.iter().enumerate().collect_vec();
    (0..junction_boxes.len()).for_each(|i| circuit_assignment[i] = (i + 1) as u16);

    let mut part_1 = None;
    let mut part_2 = None;

    let mut connected = 0;
    for (connections, ((pos_a, a), (pos_b, b))) in boxes_with_index
        .pairs()
        .sorted_by_key(|(a, b)| pos_dist_euclid(&(a.1, b.1)))
        .enumerate()
    {
        let a_circuit = circuit_assignment[*pos_a];
        let b_circuit = circuit_assignment[*pos_b];
        if a_circuit != b_circuit {
            // Combine two circuits (B -> A)
            circuit_assignment
                .iter_mut()
                .filter(|jnc| **jnc == b_circuit)
                .for_each(|to_combine| *to_combine = a_circuit);
            // println!("Connected {:?} and {:?}", a, b);
            connected += 1;
            if connected == boxes_with_index.len() - 1 {
                part_2 = Some(a.0 * b.0);
                break;
            }
        }
        if connections == p1_pairs - 1 {
            part_1 = Some(
                circuit_assignment[0..junction_boxes.len()]
                    .iter()
                    .counts_by(|a| *a)
                    .values()
                    .sorted()
                    .rev()
                    .take(3)
                    .map(|s| *s as u64)
                    .product::<u64>(),
            );
        }
    }

    (part_1, part_2)
}

fn parse_junctions(input: &str) -> Vec<Junction> {
    input
        .lines()
        .map(|l| {
            let jnct = l
                .split(",")
                .map(|p| p.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64, u64)>()
                .expect("Expected 3 values in junction location");
            Junction(jnct.0, jnct.1, jnct.2)
        })
        .collect_vec()
}

fn pos_dist_euclid((a, b): &(&Junction, &Junction)) -> u64 {
    (a.0.abs_diff(b.0)).pow(2) + (a.1.abs_diff(b.1).pow(2)) + (a.2.abs_diff(b.2)).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(40), Some(25272)));
    }
}
