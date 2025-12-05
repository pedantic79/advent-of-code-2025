use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser, bytes::complete::tag, combinator::all_consuming};

use crate::common::nom::{nom_lines, nom_u64};

#[derive(Debug, PartialEq, Eq)]
pub struct IngredientInfo {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Range {
    start: u64,
    end: u64,
}

fn parse_range(s: &str) -> IResult<&str, Range> {
    let (s, start) = nom_u64(s)?;
    let (s, _) = tag("-").parse(s)?;
    let (s, end) = nom_u64(s)?;
    Ok((s, Range { start, end }))
}

fn parse_input(s: &str) -> IResult<&str, (Vec<Range>, Vec<u64>)> {
    let (s, ranges) = nom_lines(parse_range).parse(s)?;
    let (s, _) = tag("\n\n").parse(s)?;
    let (s, ids) = nom_lines(nom_u64).parse(s)?;

    Ok((s, (ranges, ids)))
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> IngredientInfo {
    let (ranges, ids) = all_consuming(parse_input).parse(input).unwrap().1;

    IngredientInfo {
        ranges: find_non_overlapping_ranges(&ranges),
        ids,
    }
}

#[aoc(day5, part1)]
pub fn part1(inputs: &IngredientInfo) -> usize {
    inputs
        .ids
        .iter()
        .filter(|&id| {
            inputs
                .ranges
                .iter()
                .any(|Range { start, end }| id >= start && id <= end)
        })
        .count()
}

fn merge_overlapping_ranges(a: Range, b: Range) -> Option<Range> {
    if a.end < b.start || b.end < a.start {
        return None;
    }

    Some(Range {
        start: a.start.min(b.start),
        end: a.end.max(b.end),
    })
}

fn find_non_overlapping_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut non_overlapping_ranges: Vec<Range> = Vec::with_capacity(ranges.len());

    // Iterate through each range and try to merge it with existing non-overlapping ranges
    for current in ranges {
        let mut current = *current;
        let mut i = 0;

        while i < non_overlapping_ranges.len() {
            if let Some(merged) = merge_overlapping_ranges(current, non_overlapping_ranges[i]) {
                // If they overlap, update current to the merged range and remove the existing range
                current = merged;
                non_overlapping_ranges.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // After attempting to merge with all existing ranges, add the (possibly merged) current range
        non_overlapping_ranges.push(current);
    }

    non_overlapping_ranges
}

#[aoc(day5, part2)]
pub fn part2(inputs: &IngredientInfo) -> u64 {
    inputs
        .ranges
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

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
        assert_eq!(part2(&generator(SAMPLE)), 14);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day5.txt");
        const ANSWERS: (usize, u64) = (509, 336790092076620);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
