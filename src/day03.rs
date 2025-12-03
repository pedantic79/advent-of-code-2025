use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> usize {
    let mut total = 0;
    for input in inputs {
        let mut max = usize::MIN;

        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                let x = (input.as_bytes()[i] - b'0') * 10 + (input.as_bytes()[j] - b'0');
                max = max.max(x as usize);
            }
        }

        total += max;
    }

    total
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> usize {
    let mut total = 0;

    let mut digits = Vec::with_capacity(12);

    for line in inputs {
        let line = line.as_bytes();
        let mut idx = 0;
        let mut num_skips = line.len() - 12;
        digits.clear();

        while digits.len() < 12 {
            // we find the left-most maximum digit in the range of possible next digits
            // out of the possible number we can skip

            // we use enumerate + rev + max_by_key to find the position of left-most the maximum digit
            // rev is important because max_by_key returns the last maximum it finds, not the first, so reversing
            // the iterator makes it return the left-most maximum
            let (pos, next_digit) = line[idx..idx + num_skips + 1]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, y)| **y)
                .map(|(pos, b)| (pos, *b))
                .unwrap();

            num_skips -= pos;
            idx += pos + 1;
            digits.push(next_digit);
        }

        total += digits
            .iter()
            .fold(0usize, |acc, &d| acc * 10 + (d - b'0') as usize);
    }

    total
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
        const ANSWERS: (usize, usize) = (17613, 175304218462560);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
