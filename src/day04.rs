use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::utils::neighbors_diag;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<u8>, usize, usize) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut v = Vec::with_capacity(height * width);
    for line in input.lines() {
        v.extend(line.bytes());
    }

    (v, height, width)
}

fn neighbor_check(inputs: &[u8], r: usize, c: usize, r_max: usize, c_max: usize) -> bool {
    neighbors_diag(r, c, r_max, c_max)
        .filter(|&(nr, nc)| inputs[nr * c_max + nc] == b'@')
        .count()
        < 4
}

fn neighbor_check2(inputs: &[u8], r: usize, c: usize, r_max: usize, c_max: usize) -> bool {
    neighbors_diag(r, c, r_max, c_max)
        .filter(|&(nr, nc)| inputs[nr * c_max + nc] == b'@')
        .try_fold(0, |acc, _| if acc + 1 >= 4 { None } else { Some(acc + 1) })
        .is_some()
}

#[aoc(day4, part1)]
pub fn part1((inputs, r_max, c_max): &(Vec<u8>, usize, usize)) -> usize {
    let mut sum = 0;
    for r in 0..*r_max {
        for c in 0..*c_max {
            let idx = r * c_max + c;
            if inputs[idx] != b'@' {
                continue;
            }

            if neighbor_check2(inputs, r, c, *r_max, *c_max) {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2((inputs, r_max, c_max): &(Vec<u8>, usize, usize)) -> usize {
    let mut count = 0;
    let mut inputs = inputs.to_vec();
    let mut to_remove = Vec::with_capacity(2048);

    loop {
        for r in 0..*r_max {
            for c in 0..*c_max {
                let idx = r * c_max + c;
                if inputs[idx] != b'@' {
                    continue;
                }

                if neighbor_check(&inputs, r, c, *r_max, *c_max) {
                    to_remove.push(idx);
                }
            }
        }

        count += to_remove.len();
        if to_remove.is_empty() {
            break count;
        }

        for &idx in &to_remove {
            inputs[idx] = b'.';
        }
        to_remove.clear();
    }
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
