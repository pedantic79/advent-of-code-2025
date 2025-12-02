use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    left: String,
    right: String,
}

// precomputed multiples for lengths 0 to 10
// (1..=n / 2).filter(|&k| n.is_multiple_of(k))
const MULTIPLES: [&[usize]; 11] = [
    &[],
    &[],
    &[1],
    &[1],
    &[1, 2],
    &[1],
    &[1, 2, 3],
    &[1],
    &[1, 2, 4],
    &[1, 3],
    &[1, 2, 5],
];

fn parse_int_wrapper(f: impl Fn(&[u8]) -> bool, s: &[u8]) -> usize {
    if f(s) {
        unsafe { str::from_utf8_unchecked(s) }
            .parse::<usize>()
            .unwrap()
    } else {
        0
    }
}

fn is_invalid_one(s: &[u8]) -> bool {
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let (l, r) = s.split_at(s.len() / 2);
    l == r
}

fn is_invalid_two(s: &[u8]) -> bool {
    let n = s.len();
    if n < 2 {
        return false;
    }

    MULTIPLES[n].iter().any(|&k| {
        let pattern = &s[0..k];

        // the cycle version is much faster than the chunks, repeat, or step_by versions
        // s.chunks(k).all(|chunk| chunk == pattern)
        // pattern.repeat(n / k).into_iter().eq(s.iter().copied())
        // (k..n).step_by(k).all(|i| &s[i..i + k] == pattern)

        pattern.iter().cycle().take(s.len()).eq(s.iter())
    })
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|group| {
            let (l, r) = group.split_once('-').unwrap();

            Range {
                left: l.to_string(),
                right: r.to_string(),
            }
        })
        .collect()
}

fn solve(inputs: &[Range], f: impl Fn(&[u8]) -> bool + Sync) -> usize {
    inputs
        .par_iter()
        .map(|range| {
            let start = range.left.parse::<usize>().unwrap();
            let end = range.right.parse::<usize>().unwrap();

            // Do math on the strings to loop through the range
            let mut s = range.left.to_string().into_bytes();
            let mut local_total = 0;
            for x in start..=end {
                local_total += parse_int_wrapper(&f, &s);
                if x == end {
                    break;
                }

                // loop through the positions in the sting from back to front
                // to increment the number by 1
                let len = s.len();
                for i in (0..len).rev() {
                    if s[i] == b'9' {
                        s[i] = b'0';
                        // We don't break here since we need to carry over
                        if i == 0 {
                            // Handle overflow
                            s.insert(0, b'1');
                        }
                    } else {
                        s[i] += 1;
                        break;
                    }
                }
            }
            local_total
        })
        .sum()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[Range]) -> usize {
    solve(inputs, is_invalid_one)
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[Range]) -> usize {
    solve(inputs, is_invalid_two)
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
        const ANSWERS: (usize, usize) = (21139440284, 38731915928);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
