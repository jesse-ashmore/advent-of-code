use std::usize;

use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(12);

pub fn solve(input: &str) -> (Option<usize>, Option<u64>) {
    let (tiles, schemas) = parse_input(input);

    let mut tile_areas = vec![0; tiles.len()];
    for (i, tile) in tiles.iter().enumerate() {
        tile_areas[i] = tile.scan().filter(|(_, t)| **t).count();
    }

    // Doesn't work for example, real input contains nice tiles which can all
    // fit together perfectly. This means we just need to sum up the requirements
    // of each tile, and if it exceeds the area, we're done.
    let possible = schemas
        .iter()
        .filter(|schema| {
            let sum = schema
                .requirements
                .iter()
                .enumerate()
                .map(|(i, min)| tile_areas[i] * min)
                .sum::<usize>();
            sum <= schema.height * schema.width
        })
        .collect_vec();

    (Some(possible.len()), None)
}

#[derive(Debug)]
struct Schema {
    width: usize,
    height: usize,
    requirements: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<[[bool; 3]; 3]>, Vec<Schema>) {
    let parts = input.split("\n\n").collect_vec();

    let mut tiles = Vec::new();
    for ele in parts[..=5].iter() {
        let mut tile: [[bool; 3]; 3] = [
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ];
        for (y, row) in ele.lines().skip(1).enumerate() {
            for (x, b) in row.chars().map(|c| c == '#').enumerate() {
                tile[y][x] = b;
            }
        }
        tiles.push(tile);
    }

    let schemas = parts[6]
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let (width, height) = lhs.split_once("x").unwrap();
            let needed = rhs.split(" ").map(|p| p.parse().unwrap()).collect_vec();
            return Schema {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                requirements: needed,
            };
        })
        .collect_vec();

    return (tiles, schemas);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(2), None));
    }
}
