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

impl Range {
    fn merge_overlapping(&self, b: Self) -> Option<Self> {
        if self.end < b.start || b.end < self.start {
            return None;
        }

        Some(Self {
            start: self.start.min(b.start),
            end: self.end.max(b.end),
        })
    }
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
        ranges: merge_ranges(ranges),
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

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    // We require sorted ranges to make merging easier
    ranges.sort_unstable_by_key(|r| r.start);

    let mut disjoint_ranges = Vec::with_capacity(ranges.len());
    disjoint_ranges.push(ranges[0]);

    for current in ranges[1..].iter().copied() {
        eprintln!("Current range: {:?}", current);
        eprintln!("Disjoint ranges: {:?}", disjoint_ranges);
        // We only need to check the last range in disjoint_ranges for overlap
        let last = disjoint_ranges.last_mut().unwrap();

        if let Some(merged_range) = last.merge_overlapping(current) {
            *last = merged_range;
        } else {
            // No overlap, simply add the current range to the end of disjoint_ranges
            // next time through, we will be comparing against this one
            disjoint_ranges.push(current);
        }
    }

    disjoint_ranges
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

    // mod regression {
    //     use super::*;

    //     const INPUT: &str = include_str!("../input/2025/day5.txt");
    //     const ANSWERS: (usize, u64) = (509, 336790092076620);

    //     #[test]
    //     pub fn test() {
    //         let input = INPUT.trim_end_matches('\n');
    //         let output = generator(input);

    //         assert_eq!(part1(&output), ANSWERS.0);
    //         assert_eq!(part2(&output), ANSWERS.1);
    //     }
    // }
}
