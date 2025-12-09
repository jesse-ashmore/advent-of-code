use advent_of_code::{Pairs, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let red_tiles = parse_tiles(input);
    let part_1 = red_tiles.pairs().max_by_key(|pair| area_manhattan(*pair));

    // Build a list of extents for each X and Y line across the grid
    // For each pair, check that each line in X and Y is covered by a single continuous extent
    // Could walk the line and build that way

    (dbg!(part_1).map(|pair| area_manhattan(pair)), None)
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
        assert_eq!(result, (Some(50), None));
    }

    #[test]
    fn test_manhattan_area() {
        let a: (u64, u64) = (2, 5);
        let b: (u64, u64) = (11, 1);
        assert_eq!(area_manhattan((&a, &b)), 50)
    }
}
