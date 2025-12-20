use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{IResult, Parser, bytes::complete::tag, combinator, sequence::separated_pair};

use crate::common::nom::{nom_i64, nom_lines, process_input};

fn parse_line(s: &str) -> IResult<&str, [i64; 2]> {
    combinator::map(separated_pair(nom_i64, tag(","), nom_i64), |(a, b)| [a, b]).parse(s)
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<[i64; 2]> {
    process_input(nom_lines(parse_line))(input)
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[[i64; 2]]) -> u64 {
    inputs
        .iter()
        .tuple_combinations()
        .map(|(a, b)| distance(*a, *b))
        .max()
        .unwrap_or(0)
}

fn sort_pairs(x: [i64; 2], y: [i64; 2]) -> [[i64; 2]; 2] {
    let a = [x[0].min(y[0]), x[1].min(y[1])];
    let b = [x[0].max(y[0]), x[1].max(y[1])];

    [a, b]
}

fn wide_less_than(a: [i64; 2], b: [i64; 2]) -> bool {
    a[0] < b[0] && a[1] < b[1]
}

fn distance(a: [i64; 2], b: [i64; 2]) -> u64 {
    (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1)
}

// This is simliar to the Itertools::circular_tuple_windows, but sort them because
// it's faster.
fn pair_list(inputs: &[[i64; 2]]) -> Vec<[[i64; 2]; 2]> {
    let last = inputs.len() - 1;

    let mut inputs_pairs = Vec::with_capacity(inputs.len());
    for i in 0..last {
        let [c, d] = sort_pairs(inputs[i], inputs[i + 1]);
        inputs_pairs.push([c, d]);
    }
    let [c, d] = sort_pairs(inputs[last], inputs[0]);
    inputs_pairs.push([c, d]);

    inputs_pairs.sort_unstable();
    inputs_pairs
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[[i64; 2]]) -> u64 {
    let inputs_pairs = pair_list(inputs);
    inputs
        .iter()
        .tuple_combinations()
        .filter_map(|(x, y)| {
            let [a, b] = sort_pairs(*x, *y);

            for &[c, d] in &inputs_pairs {
                if wide_less_than(a, d) && wide_less_than(c, b) {
                    return None;
                }
            }

            Some(distance(a, b))
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 50);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 24);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day9.txt");
        const ANSWERS: (u64, u64) = (4771532800, 1544362560);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
