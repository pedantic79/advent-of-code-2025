use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::neighbors_diag;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();
    inputs
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|&(c, &cell)| {
                    if cell != b'@' {
                        return false;
                    }
                    neighbors_diag(r, c, r_max, c_max)
                        .filter(|&(nr, nc)| inputs[nr][nc] == b'@')
                        .count()
                        < 4
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Vec<u8>]) -> usize {
    let r_max = inputs.len();
    let c_max = inputs[0].len();

    let mut count = 0;
    let mut inputs = inputs.to_vec();

    loop {
        let mut to_remove = Vec::new();

        for r in 0..r_max {
            for c in 0..c_max {
                if inputs[r][c] != b'@' {
                    continue;
                }

                let neighbor_count = neighbors_diag(r, c, r_max, c_max)
                    .filter(|&(nr, nc)| inputs[nr][nc] == b'@')
                    .count();

                if neighbor_count < 4 {
                    to_remove.push((c, r));
                }
            }
        }

        count += to_remove.len();
        if to_remove.is_empty() {
            break;
        }

        for (c, r) in to_remove {
            inputs[r][c] = b'.';
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 13);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 43);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day4.txt");
        const ANSWERS: (usize, usize) = (1435, 8623);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
