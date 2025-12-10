use std::collections::{HashMap, HashSet};

use advent_of_code::{DirectionAxes, Pairs, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut red_tiles = parse_tiles(input);
    let top_row = red_tiles.iter().map(|(_, y)| y).min().unwrap();
    let top_left = red_tiles
        .iter()
        .filter(|(_, y)| y == top_row)
        .min_by_key(|(x, _)| x)
        .unwrap();
    let start_offset = red_tiles
        .iter()
        .find_position(|x| x == &top_left)
        .unwrap()
        .0;
    red_tiles.rotate_left(start_offset);
    let square_like_ordered = red_tiles
        .pairs()
        .filter(|(a, b)| a.0 != b.0 && a.1 != b.1)
        .sorted_by_key(|pair| area_manhattan(*pair))
        .rev()
        .collect_vec();
    let part_1 = square_like_ordered
        .first()
        .map(|pair| area_manhattan(*pair));

    let mut perimeter = HashSet::new();
    let clockwise = match get_heading(red_tiles[0], red_tiles[1]) {
        DirectionAxes::Right => true,
        DirectionAxes::Down => false,
        _ => panic!("Top-most, left-most point should head right or down"),
    };

    for (a, b) in red_tiles
        .iter()
        .chain([*red_tiles.first().unwrap()].iter())
        .tuple_windows()
    {
        match get_heading(*a, *b) {
            DirectionAxes::Up => {
                // Clockwise outside => L else R
                let outside_x = if clockwise { a.0 - 1 } else { a.0 + 1 };
                (b.1.min(a.1)-1..=b.1.max(a.1)).for_each(|y| {
                    perimeter.insert((outside_x, y));
                });
            }
            DirectionAxes::Down => {
                // Clockwise outside => R else L
                let outside_x = if clockwise { a.0 + 1 } else { a.0 - 1 };
                (a.1..=b.1 + 1).for_each(|y| {
                    perimeter.insert((outside_x, y));
                });
            }
            DirectionAxes::Left => {
                // Clockwise outside => B else T
                let outside_y = if clockwise { a.1 + 1 } else { a.1 - 1 };
                (b.0 - 1..=a.0).for_each(|x| {
                    perimeter.insert((x, outside_y));
                });
            }
            DirectionAxes::Right => {
                // Clockwise outside => T else B
                let outside_y = if clockwise { a.1 - 1 } else { a.1 + 1 };
                (a.0..=b.0).for_each(|x| {
                    perimeter.insert((x, outside_y));
                });
            }
        }
        for y in 0..14 {
            for x in 0..14 {
                let x = if perimeter.contains(&(x, y)) {
                    "X"
                } else {
                    "."
                };
                print!("{}", x);
            }
            println!();
        }
    }

    // Patch over any accidentally permiterized red/green tiles (about half need to be reinstated)
    for (a, b) in red_tiles
        .iter()
        .chain([*red_tiles.last().unwrap()].iter())
        .tuple_windows()
    {
        if a.0 == b.0 {
            for y in a.1.min(b.1)..=a.1.max(b.1) {
                perimeter.remove(&(a.0, y));
            }
        } else {
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                perimeter.remove(&(x, a.1));
            }
        }
    }

    for y in 0..14 {
        for x in 0..14 {
            let x = if perimeter.contains(&(x, y)) {
                "X"
            } else {
                "."
            };
            print!("{}", x);
        }
        println!();
    }
    panic!();
    let largest_inner_rect = square_like_ordered
        .iter()
        .filter(|(a, b)| {
            let top = a.1.min(b.1);
            let right = a.0.max(b.1);
            let bottom = a.1.max(b.1);
            let left = a.0.min(b.0);

            for x in left..=right {
                if perimeter.contains(&(x, top)) || perimeter.contains(&(x, bottom)) {
                    return false;
                };
            }
            for y in top..=bottom {
                if perimeter.contains(&(left, y)) || perimeter.contains(&(right, y)) {
                    return false;
                };
            }

            true
        })
        .next()
        .map(|pair| area_manhattan(*pair));

    (part_1, largest_inner_rect)
}

fn get_heading(from: (u64, u64), to: (u64, u64)) -> DirectionAxes {
    if from.0 == to.0 {
        if from.1 < to.1 {
            return DirectionAxes::Down;
        } else {
            return DirectionAxes::Up;
        };
    } else {
        if from.0 < to.0 {
            return DirectionAxes::Right;
        } else {
            return DirectionAxes::Left;
        };
    }
}

fn parse_tiles(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|p| p.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .expect("Expected 3 values in junction location")
        })
        .collect_vec()
}

fn area_manhattan((a, b): (&(u64, u64), &(u64, u64))) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(50), Some(24)));
    }

    #[test]
    fn test_manhattan_area() {
        let a: (u64, u64) = (2, 5);
        let b: (u64, u64) = (11, 1);
        assert_eq!(area_manhattan((&a, &b)), 50)
    }
}
