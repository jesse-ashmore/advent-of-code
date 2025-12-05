use advent_of_code::{Direction, DirectionAll, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();

    let mut map_sweeper: Vec<Vec<usize>> = vec![vec![0; map.width()]; map.height()];
    for (pos, val) in map.scan() {
        if *val {
            if let Some(sweep) = map_sweeper.getxy_pos_mut(pos) {
                *sweep = map.neighbours_around(pos).filter(|v| *v.1).count();
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    while let Some(to_remove) = remove_paper(&map, &mut map_sweeper) {
        if part_1 == 0 {
            part_1 = to_remove.len();
        }
        part_2 += to_remove.len();
        to_remove.iter().for_each(|pos| {
            if let Some(to_update) = map.getxy_pos_mut(*pos) {
                *to_update = false;
                if let Some(to_update_again) = map_sweeper.getxy_pos_mut(*pos) {
                    *to_update_again = 0;
                }
            }
            DirectionAll::iterator()
                .map(|dir| dir.step().add(&(pos.0 as i32, pos.1 as i32)))
                .for_each(|(x, y)| {
                    if let Some(to_decr) = map_sweeper.getxy_pos_mut((x as usize, y as usize)) {
                        if *to_decr > 0 {
                            *to_decr -= 1;
                        }
                    }
                });
        });
    }

    (Some(part_1 as u64), Some(part_2 as u64))
}

fn remove_paper(
    map: &Vec<Vec<bool>>,
    map_sweeper: &mut Vec<Vec<usize>>,
) -> Option<Vec<(usize, usize)>> {
    let to_remove = map_sweeper
        .scan()
        .filter(|(pos, v)| *map.getxy_pos(*pos).unwrap_or(&false) && **v < 4)
        .map(|(pos, _)| pos)
        .collect_vec();

    if to_remove.is_empty() {
        None
    } else {
        Some(to_remove)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(13), Some(43)));
    }
}
