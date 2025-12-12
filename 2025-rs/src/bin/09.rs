
use advent_of_code::{DirectionAxes, Pairs};
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let red_tiles = parse_tiles(input);
    let square_like_ordered = red_tiles
        .pairs()
        .filter(|(a, b)| a.0 != b.0 && a.1 != b.1)
        .sorted_by_key(|pair| area_manhattan(*pair))
        .rev()
        .collect_vec();
    let part_1 = square_like_ordered
        .first()
        .map(|pair| area_manhattan(*pair));

    // Had the right idea originally but adapted implementation from
    // https://www.reddit.com/r/adventofcode/comments/1phywvn/comment/nt85x48/
    let mut verticals = Vec::new();
    let mut horizontals = Vec::new();
    let mut outside_vert = Vec::new();
    let mut outside_horiz = Vec::new();

    // build vectors
    for (a, b) in red_tiles
        .iter()
        .chain([*red_tiles.first().unwrap()].iter())
        .tuple_windows()
    {
        match get_heading(*a, *b) {
            DirectionAxes::Up | DirectionAxes::Down => {
                verticals.push((a.0, b.1.min(a.1)..b.1.max(a.1)));
            }

            DirectionAxes::Left | DirectionAxes::Right => {
                horizontals.push((a.1, b.0.min(a.0)..b.0.max(a.0)));
            }
        }
    }
    verticals.sort_by_key(|(x, _)| *x);
    horizontals.sort_by_key(|(y, _)| *y);

    for (i, (x, vertical)) in verticals.iter().enumerate() {
        let crossed = verticals[i + 1..]
            .iter()
            .filter(|(_, other_vertical)| {
                let mid = (vertical.start + vertical.end) / 2;
                if other_vertical.start <= mid && mid < other_vertical.end {
                    return true;
                }
                false
            })
            .count();
        let offset_x: u64 = if crossed.rem_euclid(2) == 0 {
            x + 1
        } else {
            x - 1
        };
        outside_vert.push((offset_x, (vertical.start + 1..vertical.end - 1)));
    }

    for (i, (y, horizontal)) in horizontals.iter().enumerate() {
        let crossed = horizontals[i + 1..]
            .iter()
            .filter(|(_, other_horizontal)| {
                let mid = (horizontal.start + horizontal.end) / 2;
                if other_horizontal.start <= mid && mid < other_horizontal.end {
                    return true;
                }
                false
            })
            .count();
        let offset_x: u64 = if crossed.rem_euclid(2) == 0 {
            y + 1
        } else {
            y - 1
        };
        outside_horiz.push((offset_x, (horizontal.start + 1..horizontal.end - 1)));
    }

    let largest_inner_rect = square_like_ordered
        .iter()
        .find(|(a, b)| {
            let top = a.1.min(b.1);
            let right = a.0.max(b.0);
            let bottom = a.1.max(b.1);
            let left = a.0.min(b.0);

            if outside_vert.iter().any(|(x, vert)| {
                (left <= *x && *x <= right) && top <= vert.end && vert.start <= bottom
            }) {
                return false;
            }
            if outside_horiz.iter().any(|(y, horiz)| {
                (top <= *y && *y <= bottom) && left <= horiz.end && horiz.start <= right
            }) {
                return false;
            }

            true
        })
        .map(|pair| area_manhattan(*pair));
    (part_1, largest_inner_rect)
}

fn get_heading(from: (u64, u64), to: (u64, u64)) -> DirectionAxes {
    if from.0 == to.0 {
        if from.1 < to.1 {
            DirectionAxes::Down
        } else {
            DirectionAxes::Up
        }
    } else if from.0 < to.0 {
        DirectionAxes::Right
    } else {
        DirectionAxes::Left
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
