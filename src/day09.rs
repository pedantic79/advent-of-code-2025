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

fn distance(a: [i64; 2], b: [i64; 2]) -> u64 {
    (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1)
}

fn sort_pairs(x: [i64; 2], y: [i64; 2]) -> [[i64; 2]; 2] {
    let a = [x[0].min(y[0]), x[1].min(y[1])];
    let b = [x[0].max(y[0]), x[1].max(y[1])];

    [a, b]
}

/// Edge represented as (fixed_coord, range_min, range_max)
type Edge = (i64, i64, i64);

/// Vertical edge: (x, y_min, y_max)
/// Horizontal edge: (y, x_min, x_max)
fn build_edge_lists(inputs: &[[i64; 2]]) -> (Vec<Edge>, Vec<Edge>) {
    let n = inputs.len();
    let mut vertical = Vec::with_capacity(n / 2);
    let mut horizontal = Vec::with_capacity(n / 2);

    // loop through edge pairs and process each edge into vertical and horizontal lists
    // this works like circular_tuple_windows()
    for i in 0..(n - 1) {
        process_edge(&mut vertical, &mut horizontal, inputs[i], inputs[i + 1]);
    }
    process_edge(&mut vertical, &mut horizontal, inputs[n - 1], inputs[0]);

    debug_assert_eq!(vertical.capacity(), n / 2);
    debug_assert_eq!(horizontal.capacity(), n / 2);
    debug_assert_eq!(vertical.len(), horizontal.len());

    vertical.sort_unstable();
    horizontal.sort_unstable();

    (vertical, horizontal)
}

fn process_edge(vertical: &mut Vec<Edge>, horizontal: &mut Vec<Edge>, p1: [i64; 2], p2: [i64; 2]) {
    if p1[0] == p2[0] {
        // Vertical edge
        vertical.push((p1[0], p1[1].min(p2[1]), p1[1].max(p2[1])));
    } else {
        // Horizontal edge
        horizontal.push((p1[1], p1[0].min(p2[0]), p1[0].max(p2[0])));
    }
}

/// Checks if any edge crosses through the interior of a box.
/// Edges are sorted by fixed coordinate for early termination.
fn any_edge_intersects(
    edges: &[Edge],
    fixed_min: i64,
    fixed_max: i64,
    range_min: i64,
    range_max: i64,
) -> bool {
    for &(fixed, e_min, e_max) in edges {
        if fixed <= fixed_min {
            continue;
        }
        if fixed >= fixed_max {
            break;
        }
        if e_min < range_max && e_max > range_min {
            return true;
        }
    }
    false
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[[i64; 2]]) -> u64 {
    let (vertical, horizontal) = build_edge_lists(inputs);

    inputs
        .iter()
        .tuple_combinations()
        .filter_map(|(p1, p2)| {
            let [a, b] = sort_pairs(*p1, *p2);

            // Check if any edge crosses through the rectangle
            if any_edge_intersects(&vertical, a[0], b[0], a[1], b[1])
                || any_edge_intersects(&horizontal, a[1], b[1], a[0], b[0])
            {
                None
            } else {
                Some(distance(a, b))
            }
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
