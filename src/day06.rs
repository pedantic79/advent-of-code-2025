use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;

fn parse_op_with_ranges(chunk: &str) -> (Vec<u8>, Vec<Range<usize>>) {
    let mut ops = Vec::new();
    let mut ranges = Vec::new();

    let bytes = chunk.as_bytes();
    let mut start = 0;
    let mut in_op = false;

    for (i, &b) in bytes.iter().enumerate() {
        if b != b' ' && !in_op {
            in_op = true;
            if i != 0 {
                ranges.push(start..(i - 1));
            }
            start = i;
        } else if b == b' ' && in_op {
            in_op = false;
            ops.push(bytes[start]);
        }
    }

    ranges.push(start..bytes.len());

    debug_assert_eq!(ops.len(), ranges.len());
    (ops, ranges)
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> (Vec<ArrayVec<[u8; 4], 4>>, Vec<u8>) {
    let mut iter = input.lines().rev();
    let (ops, ranges) = parse_op_with_ranges(iter.next().unwrap());

    let columns = ops.len();
    let mut nums = vec![ArrayVec::new(); columns];

    for line in iter {
        let bytes = line.as_bytes();

        for (col, range) in ranges.iter().enumerate() {
            let slice = &bytes[range.clone()];

            nums[col].push([
                slice.first().copied().unwrap_or(b' '),
                slice.get(1).copied().unwrap_or(b' '),
                slice.get(2).copied().unwrap_or(b' '),
                slice.get(3).copied().unwrap_or(b' '),
            ]);
        }
    }

    (nums, ops)
}

#[aoc(day6, part1)]
pub fn part1((nums, ops): &(Vec<ArrayVec<[u8; 4], 4>>, Vec<u8>)) -> u64 {
    let mut total = 0;

    for (col_nums, op) in nums.iter().zip(ops.iter()) {
        let iter = col_nums.iter().map(|x| {
            let s = unsafe { str::from_utf8_unchecked(x) }.trim();
            s.parse::<u64>().unwrap()
        });

        let col_total: u64 = match op {
            b'+' => iter.sum(),
            b'*' => iter.product(),
            _ => panic!("Unknown operation"),
        };
        total += col_total;
    }

    total
}

fn rotate_numbers(nums: &[[u8; 4]]) -> Vec<u64> {
    let mut res = Vec::new();

    for amount in 0..4 {
        let n = nums
            .iter()
            .filter_map(|n| Some(n[amount]).filter(|&c| c != b' '))
            .rev()
            .fold(0, |acc, d| acc * 10 + (u64::from(d - b'0')));

        if n != 0 {
            res.push(n);
        }
    }

    res
}

#[aoc(day6, part2)]
pub fn part2((nums, ops): &(Vec<ArrayVec<[u8; 4], 4>>, Vec<u8>)) -> u64 {
    let mut total = 0;
    for (num_col, op) in nums.iter().zip(ops.iter()) {
        let rotated = rotate_numbers(num_col);

        let col_total: u64 = match op {
            b'+' => rotated.iter().sum(),
            b'*' => rotated.iter().product(),
            _ => panic!("Unknown operation"),
        };
        total += col_total;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    // rustfmt::skip
    const SAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 4277556);
    }

    #[test]
    pub fn rotate_numbers_test() {
        assert_eq!(
            rotate_numbers(&[*b"  6 ", *b" 45 ", *b"123 "]),
            vec![1, 24, 356]
        );
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 3263827);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day6.txt");
        const ANSWERS: (u64, u64) = (4387670995909, 9625320374409);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
