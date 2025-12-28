use aoc_runner_derive::{aoc, aoc_generator};

struct Range {
    start: u64,
    end: u64,
    step: u64,
}

/// Creates an arithmetic sequence for numbers formed by repeating a `size`-digit pattern
/// to fill `digits` total digits.
///
/// Step = `(10^digits - 1) / (10^size - 1)` (geometric series multiplier for repetition).
///
/// Examples: `range(2,1)` → 11,22,...,99; `range(4,2)` → 1010,1111,...,9999
const fn range(digits: u32, size: u32) -> Range {
    let digits_power = 10u64.pow(digits);
    let size_power = 10u64.pow(size);

    let step = (digits_power - 1) / (size_power - 1);
    let start = step * (size_power / 10);
    let end = step * (size_power - 1);

    Range { start, end, step }
}

/// Patterns repeated exactly 2 times: 11, 1212, 123123, etc.
/// Base set for part 1; combined with SECOND/THIRD via inclusion-exclusion for part 2.
const FIRST: &[Range] = &[
    range(2, 1),
    range(4, 2),
    range(6, 3),
    range(8, 4),
    range(10, 5),
];

/// Patterns repeated 3+ times: 111, 12121, 121212, etc.
/// Added in part 2 to cover numbers not in FIRST.
const SECOND: &[Range] = &[
    range(3, 1),
    range(5, 1),
    range(6, 2),
    range(7, 1),
    range(9, 3),
    range(10, 2),
];

/// Overlap between FIRST and SECOND (6× and 10× repetitions).
/// Subtracted in part 2: `FIRST + SECOND - THIRD`.
const THIRD: &[Range] = &[range(6, 1), range(10, 1)];

type Pair = [u64; 2];

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Pair> {
    input
        .split(',')
        .map(|group| {
            let (l, r) = group.split_once('-').unwrap();

            [l.parse().unwrap(), r.parse().unwrap()]
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Pair]) -> u64 {
    sum(FIRST, input)
}

#[aoc(day2, part2)]
pub fn part2(input: &[Pair]) -> u64 {
    sum(FIRST, input) + sum(SECOND, input) - sum(THIRD, input)
}

fn sum(ranges: &[Range], input: &[Pair]) -> u64 {
    let mut result = 0;

    for &Range { start, end, step } in ranges.iter() {
        for &[from, to] in input {
            let lower = from.next_multiple_of(step).max(start);
            let upper = to.min(end);

            if lower <= upper {
                let n = (upper - lower) / step;
                let triangular = n * (n + 1) / 2;
                result += lower * (n + 1) + step * triangular;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 1227775554);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 4174379265);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day2.txt");
        const ANSWERS: (u64, u64) = (21139440284, 38731915928);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
