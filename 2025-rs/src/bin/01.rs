use itertools::Itertools;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct Inst(Dir, i16);

impl Inst {
    fn apply(&self, dial: i16) -> i16 {
        (match self.0 {
            Dir::Left => dial - self.1,
            Dir::Right => dial + self.1,
        })
        .rem_euclid(100)
    }

    fn from_line(line: &str) -> Self {
        let dir = match line.chars().next() {
            Some('L') => Dir::Left,
            Some('R') => Dir::Right,
            _ => panic!("Expected L or R at the start of instruction"),
        };
        Inst(
            dir,
            line.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i16>()
                .expect("Unable to parse instruction count to i16"),
        )
    }
}

pub fn solve(input: &String) -> (Option<u64>, Option<u64>) {
    let instructions = input.lines().map(Inst::from_line).collect_vec();

    (Some(part_one(&instructions)), Some(part_two(&instructions)))
}

fn part_one(instructions: &[Inst]) -> u64 {
    let mut dial = 50;
    let mut clicks = 0;
    for inst in instructions {
        dial = inst.apply(dial);
        if dial == 0 {
            clicks += 1;
        }
    }

    clicks
}

fn part_two(instructions: &[Inst]) -> u64 {
    let mut dial = 50;
    let mut clicks = 0;
    for inst in instructions {
        if dial != 0
            && ((inst.0 == Dir::Right && inst.1.rem_euclid(100) > (100 - dial))
                || (inst.0 == Dir::Left && inst.1.rem_euclid(100) > (dial)))
        {
            clicks += 1
        }

        dial = inst.apply(dial);
        if dial == 0 {
            clicks += 1;
        }
        clicks += (inst.1 / 100).abs() as u64;
    }

    clicks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(3), Some(6)));
    }
}
