use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn solve<const N: usize>(lines: &[Vec<u8>]) -> u64 {
    let mut total = 0;

    for line in lines {
        let mut idx = 0;
        let mut skips_remaining = line.len() - N;
        let mut num = 0;
        let mut digits_left = N;

        while digits_left > 0 {
            // skip_remaining indicates how many digits we can skip ahead.
            // This allows us to build a slice of possible next digits to choose from.
            // idx..idx+1 + skips_remaining is the largest we can look at.
            // We take the maximum digit from that slice to maximize the resulting number.
            // We also need to track how many digits we skip to get there, so we can update idx and skips_remaining.

            // We enumerate + rev + max_by_key to find the position of left-most the maximum digit.
            // max_by_key returns the last maximum it finds, not the first, so we can fix this by reversing the iterator
            // to return the left-most maximum rather than the right-most maximum.
            let (pos, next_digit) = line[idx..idx + 1 + skips_remaining]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, y)| **y)
                .map(|(pos, b)| (pos, *b))
                .unwrap();

            skips_remaining -= pos;
            idx += pos + 1;
            digits_left -= 1;
            num = num * 10 + u64::from(next_digit - b'0');

            // no more skips left, take the rest of the digits because line[idx..idx+1+0] is just 1 digit at this point
            // It's pointless to use max_by_key when there's only one digit left to choose from.
            // So we can just append the rest of the digits directly.
            if skips_remaining == 0 {
                for &b in &line[idx..] {
                    num = num * 10 + u64::from(b - b'0');
                }
                break;
            }
        }

        total += num;
    }

    total
}

#[aoc(day3, part1)]
pub fn part1(lines: &[Vec<u8>]) -> u64 {
    solve::<2>(lines)
}

#[aoc(day3, part2)]
pub fn part2(lines: &[Vec<u8>]) -> u64 {
    solve::<12>(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 357);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 3121910778619);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day3.txt");
        const ANSWERS: (u64, u64) = (17613, 175304218462560);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
