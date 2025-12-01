use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::parse::parse_lines;

#[derive(Debug, PartialEq, Eq)]
pub enum Dir {
    Left(i32),
    Right(i32),
}

impl FromStr for Dir {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = &s[0..1];
        let num = s[1..].parse().unwrap();
        Ok(match b {
            "L" => Self::Left(num),
            "R" => Self::Right(num),
            _ => unreachable!(),
        })
    }
}

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Dir> {
    parse_lines(input)
}

#[aoc(day1, part1)]
pub fn part1(inputs: &[Dir]) -> usize {
    let mut pos: i32 = 50;
    let mut count = 0;

    for d in inputs {
        pos = match d {
            Dir::Left(x) => (pos - x).rem_euclid(100),
            Dir::Right(x) => (pos + x).rem_euclid(100),
        };

        if pos == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[Dir]) -> usize {
    let mut pos: i32 = 50;
    let mut count = 0;

    for d in inputs {
        let x = match d {
            Dir::Left(x) => x,
            Dir::Right(x) => x,
        };

        count += x / 100;

        pos = match d {
            Dir::Left(x) => {
                let temp = (pos - x).rem_euclid(100);
                if pos != 0 && (temp >= pos || temp == 0) {
                    count += 1;
                }
                temp
            }
            Dir::Right(x) => {
                let temp = (pos + x).rem_euclid(100);
                if temp < pos {
                    count += 1;
                }
                temp
            }
        };
    }

    count as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 3);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 6);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day1.txt");
        const ANSWERS: (usize, usize) = (1029, 5892);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
