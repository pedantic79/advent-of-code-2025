use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::parse::parse_lines_fn;

const DIAL_SIZE: i32 = 100;
const DIAL_START: i32 = 50;

fn parse_input(line: &str) -> i32 {
    let (b, num) = line.split_at(1);
    let num: i32 = num.parse().unwrap();

    if "L" == b { -num } else { num }
}

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    parse_lines_fn(input, parse_input)
}

#[aoc(day1, part1)]
pub fn part1(spins: &[i32]) -> usize {
    let mut dial = DIAL_START;
    let mut count = 0;

    for x in spins {
        dial = (dial + x).rem_euclid(DIAL_SIZE);

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn part2(spins: &[i32]) -> usize {
    let mut dial = DIAL_START;
    let mut count = 0;

    for signed_amount in spins {
        let temp = dial + signed_amount;
        let new_dial = temp.rem_euclid(DIAL_SIZE);

        count += (temp / DIAL_SIZE).abs();
        if dial != 0 && temp <= 0 {
            count += 1;
        }
        dial = new_dial
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
