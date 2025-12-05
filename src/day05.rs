use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Object {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let ranges = top
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = bottom.lines().map(|line| line.parse().unwrap()).collect();
    Object { ranges, ids }
}

#[aoc(day5, part1)]
pub fn part1(inputs: &Object) -> usize {
    let mut count = 0;
    for id in &inputs.ids {
        count += usize::from(
            inputs
                .ranges
                .iter()
                .any(|(start, end)| id >= start && id <= end),
        )
    }
    count
}

fn merge_overlapping_ranges(
    a_start: u64,
    a_end: u64,
    b_start: u64,
    b_end: u64,
) -> Option<(u64, u64)> {
    if a_end < b_start || b_end < a_start {
        return None;
    }

    Some((a_start.min(b_start), a_end.max(b_end)))
}

fn find_non_overlapping_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut non_overlapping_ranges: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());

    // Iterate through each range and try to merge it with existing non-overlapping ranges
    for (start, end) in ranges {
        let mut current_start = *start;
        let mut current_end = *end;

        let mut i = 0;
        while i < non_overlapping_ranges.len() {
            if let Some((new_start, new_end)) = merge_overlapping_ranges(
                current_start,
                current_end,
                non_overlapping_ranges[i].0,
                non_overlapping_ranges[i].1,
            ) {
                current_start = new_start;
                current_end = new_end;
                non_overlapping_ranges.swap_remove(i);
            } else {
                i += 1;
            }
        }

        non_overlapping_ranges.push((current_start, current_end));
    }

    non_overlapping_ranges
}

#[aoc(day5, part2)]
pub fn part2(inputs: &Object) -> u64 {
    let non_overlapping_ranges = find_non_overlapping_ranges(&inputs.ranges);

    non_overlapping_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
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
