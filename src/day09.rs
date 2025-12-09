use aoc_runner_derive::{aoc, aoc_generator};
use geo::{Intersects, LineString, Polygon};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

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
    let mut max = 0;
    for (i, a) in inputs.iter().enumerate() {
        for b in inputs[i + 1..].iter() {
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            if area > max {
                max = area;
            }
        }
    }
    max
}

fn contains_border(polygon: &Polygon<i64>, a: (i64, i64), b: (i64, i64)) -> bool {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    for x in min_x..=max_x {
        if !polygon.intersects(&geo::Point::new(x, min_y)) {
            return false;
        }
        if !polygon.intersects(&geo::Point::new(x, max_y)) {
            return false;
        }
    }
    for y in min_y..=max_y {
        if !polygon.intersects(&geo::Point::new(min_x, y)) {
            return false;
        }
        if !polygon.intersects(&geo::Point::new(max_x, y)) {
            return false;
        }
    }

    true
}

// VERY SLOW
#[aoc(day9, part2)]
pub fn part2(inputs: &[(i64, i64)]) -> u64 {
    let line_string = LineString::from(inputs.to_vec());
    let polygon = Polygon::new(line_string, vec![]);

    inputs
        .iter()
        .combinations(2)
        .par_bridge()
        .filter_map(|v| {
            let a = v[0];
            let b = v[1];

            if contains_border(&polygon, *a, *b) {
                Some((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap()
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
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
