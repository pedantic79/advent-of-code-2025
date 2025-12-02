use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    left: usize,
    right: usize,
}

fn is_invalid_one(s: &str) -> bool {
    let (l, r) = s.split_at(s.len() / 2);
    l == r
}

fn is_invalid_two(s: &str) -> bool {
    let n = s.len();
    if n < 2 {
        return false;
    }

    (1..=n / 2).filter(|&k| n.is_multiple_of(k)).any(|k| {
        let pattern = &s[0..k];
        (k..n).step_by(k).all(|i| &s[i..i + k] == pattern)
    })
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|group| {
            let (l, r) = group.split_once('-').unwrap();

            Range {
                left: l.parse().unwrap(),
                right: r.parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[Range]) -> usize {
    inputs
        .iter()
        .flat_map(|range| range.left..=range.right)
        .filter(|s| is_invalid_one(&s.to_string()))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[Range]) -> usize {
    inputs
        .iter()
        .flat_map(|range| range.left..=range.right)
        .filter(|s| is_invalid_two(&s.to_string()))
        .sum()
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
