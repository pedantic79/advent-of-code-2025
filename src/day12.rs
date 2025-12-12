use aoc_runner_derive::{aoc, aoc_generator};
use nom::{IResult, Parser, bytes::complete::tag};

use crate::common::nom::nom_usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Region {
    width: usize,
    height: usize,
    shape_counts: [usize; 6],
}

fn count_shape(shape: &str) -> usize {
    shape.bytes().filter(|b| *b == b'#').count()
}

fn parse_region(line: &str) -> IResult<&str, (usize, usize, [usize; 6])> {
    let mut counts = [0; 6];
    let (line, width) = nom_usize.parse(line)?;
    let (line, _) = tag("x").parse(line)?;
    let (line, height) = nom_usize.parse(line)?;
    let (mut line, _) = tag(":").parse(line)?;

    for i in counts.iter_mut() {
        (line, _) = tag(" ").parse(line)?;
        (line, *i) = nom_usize.parse(line)?;
    }

    Ok((line, (width, height, counts)))
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> (Vec<Region>, [usize; 6]) {
    let mut iter = input.split("\n\n");

    let mut counts = [0; 6];
    for i in counts.iter_mut() {
        let shape = iter.next().unwrap();
        *i = count_shape(shape);
    }

    let regions = iter.next().unwrap();
    (
        regions
            .lines()
            .map(|line| {
                let (width, height, shape_counts) = parse_region(line).unwrap().1;
                Region {
                    width,
                    height,
                    shape_counts,
                }
            })
            .collect(),
        counts,
    )
}

#[aoc(day12, part1)]
pub fn part1(inputs: &(Vec<Region>, [usize; 6])) -> usize {
    inputs
        .0
        .iter()
        .filter(|region| {
            let area = region.width * region.height;

            // calculate the total area minimum area of the shapes to fit
            // in the region, assuming we are able to fit everything perfectly
            let area_of_counts: usize = region
                .shape_counts
                .iter()
                .zip(inputs.1.iter())
                .map(|(count, size)| count * size)
                .sum();

            // area must be at least 1.2x as large as the area of the shapes
            // to actually be able to fit things
            area as f64 > area_of_counts as f64 * 1.2
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn part2_test() {
        // assert_eq!(part2(&generator(SAMPLE)), 336);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day12.txt");
        const ANSWERS: (usize, usize) = (569, 0);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            // assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
