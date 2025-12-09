use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[(i64, i64)]) -> u64 {
    inputs
        .iter()
        .tuple_combinations()
        .map(|(a, b)| distance(*a, *b))
        .max()
        .unwrap_or(0)
}

fn sort_tuples(x: (i64, i64), y: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let a = (x.0.min(y.0), x.1.min(y.1));
    let b = (x.0.max(y.0), x.1.max(y.1));

    (a, b)
}

fn less_than(x: (i64, i64), y: (i64, i64)) -> bool {
    x.1 < y.1 && x.0 < y.0
}

fn distance(a: (i64, i64), b: (i64, i64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[(i64, i64)]) -> u64 {
    inputs
        .iter()
        .tuple_combinations()
        .filter_map(|(x, y)| {
            let (a, b) = sort_tuples(*x, *y);

            for (c, d) in inputs
                .iter()
                .circular_tuple_windows()
                .map(|(x, y)| sort_tuples(*x, *y))
            {
                if less_than(a, d) && less_than(c, b) {
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
