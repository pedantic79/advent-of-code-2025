use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Object {}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let mut beams = HashSet::new();
    let mut new_beams = HashSet::new();
    new_beams.insert(inputs[0].len() / 2);
    let mut count = 0;

    for row in inputs.iter().skip(1) {
        std::mem::swap(&mut beams, &mut new_beams);
        new_beams.clear();

        for &beam in beams.iter() {
            if row[beam] == b'^' {
                new_beams.insert(beam + 1);
                new_beams.insert(beam - 1);
                count += 1;
            } else {
                new_beams.insert(beam);
            }
        }
    }

    count
}

fn num_worlds(
    beam_col: usize,
    row_num: usize,
    inputs: &[Vec<u8>],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row_num >= inputs.len() {
        return 1;
    }

    if let Some(&res) = memo.get(&(beam_col, row_num)) {
        return res;
    }

    let row = &inputs[row_num];
    let count = if row[beam_col] == b'^' {
        num_worlds(beam_col + 1, row_num + 1, inputs, memo)
            + num_worlds(beam_col - 1, row_num + 1, inputs, memo)
    } else {
        num_worlds(beam_col, row_num + 1, inputs, memo)
    };

    memo.insert((beam_col, row_num), count);
    count
}

#[aoc(day7, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    num_worlds(inputs[0].len() / 2, 1, inputs, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 40);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day7.txt");
        const ANSWERS: (usize, usize) = (1717, 231507396180012);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
