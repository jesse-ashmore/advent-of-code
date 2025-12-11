use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use advent_of_code::{Direction, DirectionAll, Grid};
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (start, manifold) = parse_manifold(input);
    let beam_splits = fire_tachyon(start, &manifold);
    // beam_splits.show_map(|f| if *f { '^' } else { '.' });
    let splits = beam_splits.scan().filter(|(_, v)| **v).count() as u64;

    let mut memo = HashMap::new();
    let timelines = get_total_paths_from(start, &manifold, &mut memo);
    (Some(splits), Some(timelines))
}

struct BeamState {
    pos: (usize, usize),
    split_by: Option<(usize, usize)>,
}

fn get_total_paths_from(
    start: (usize, usize),
    manifold: &Vec<Vec<bool>>,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if manifold.getxy_pos(start).is_none() {
        return 0;
    }

    if let Some(memoed) = memo.get(&start) {
        return *memoed;
    }

    let mut beam = DirectionAll::Down.step_usize(start);

    while let Some(splitter) = manifold.getxy_pos(beam) {
        if *splitter {
            // Split beam either side.
            let left = DirectionAll::Left.step_usize(beam);
            let right = DirectionAll::Right.step_usize(beam);
            let total_left = get_total_paths_from(left, manifold, memo);
            let total_right = get_total_paths_from(right, manifold, memo);
            memo.insert(start, total_left + total_right);
            return total_left + total_right;
        } else {
            beam = DirectionAll::Down.step_usize(beam);
        }
    }

    1
}

fn fire_tachyon(start: (usize, usize), manifold: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut beam_splits = vec![vec![false; manifold.width()]; manifold.height()];
    let mut tachyon_map = vec![vec![false; manifold.width()]; manifold.height()];
    let mut beam_fronts = VecDeque::new();
    beam_fronts.push_back(BeamState {
        split_by: None,
        pos: DirectionAll::Down.step_usize(start),
    });

    while let Some(BeamState { split_by, pos }) = beam_fronts.pop_back() {
        if let Some(already_present) = tachyon_map.getxy_pos_mut(pos) {
            *already_present = true;
            if let Some(splitter) = split_by {
                beam_splits.getxy_pos_mut(splitter).map(|p| *p = true);
            }
        }
        let next_beam = DirectionAll::Down.step_usize(pos);
        // Hit the end for this beam, track its path
        if beam_can_exist(manifold, next_beam) {
            beam_fronts.push_back(BeamState {
                pos: next_beam,
                split_by: None,
            });
        } else if manifold.getxy_pos(next_beam) == Some(&true) {
            // Split beam either side.
            let left = DirectionAll::Left.step_usize(next_beam);
            let right = DirectionAll::Right.step_usize(next_beam);
            if beam_can_exist(manifold, left) && tachyon_map.getxy_pos(left) == Some(&false) {
                beam_fronts.push_back(BeamState {
                    pos: left,
                    split_by: Some(next_beam),
                });
            }
            if beam_can_exist(manifold, right) && tachyon_map.getxy_pos(right) == Some(&false) {
                beam_fronts.push_back(BeamState {
                    pos: right,
                    split_by: Some(next_beam),
                });
            }
        }
    }

    beam_splits
}

fn beam_can_exist(manifold: &Vec<Vec<bool>>, next_beam: (usize, usize)) -> bool {
    manifold.getxy_pos(next_beam) == Some(&false)
}

fn parse_manifold(input: &str) -> ((usize, usize), Vec<Vec<bool>>) {
    let start = (
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .find_position(|c| *c == 'S')
            .expect("couldn't find S")
            .0,
        0,
    );
    let manifold = input
        .lines()
        .map(|line| line.chars().map(|c| c == '^').collect_vec())
        .collect_vec();

    (start, manifold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(21), Some(40)));
    }
}
