use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn find_left_most_max(slice: &[u8]) -> (usize, u8) {
    // There is a bug here, if slice.is_empty(), this will return (usize::MIN, u8::MIN).
    // However, in our usage of this function, slice is guaranteed to be non-empty.
    let mut max = (usize::MIN, u8::MIN);

    for current in slice.iter().copied().enumerate() {
        if current.1 > max.1 {
            max = current;

            // if max.1 is 9, we can't do better, so break early
            if max.1 == b'9' {
                break;
            }
        }
    }

    max
}

fn solve<const N: usize>(lines: &[Vec<u8>]) -> u64 {
    let mut total = 0;

    for line in lines {
        let mut skips_remaining = line.len() - N;
        let mut idx: usize = 0;
        let mut joltage = 0;

        for _ in 0..N {
            // This works by greedily choosing the largest possible digit at each step.
            // We can skip up to skips_remaining digits to find the next largest digit.
            // We start at idx, and look ahead up to idx+1 + skips_remaining.
            // We find the maximum digit in that range, and append it to our number.
            // We then update idx to be just after the chosen digit, and reduce skips_remaining by how many digits we skipped.
            // Repeat until we've chosen N digits.
            let (pos, next_digit) = find_left_most_max(&line[idx..idx + 1 + skips_remaining]);

            skips_remaining -= pos;
            idx += pos + 1;
            joltage = joltage * 10 + u64::from(next_digit - b'0');

            // If we have no more skips left, we can take the rest of the digits directly.
            // This is because line[idx..idx+1+0] is just a single digit at this point.
            // So it's pointless to use find_left_most_max again.
            // Instead, we can just append the rest of the digits directly.
            if skips_remaining == 0 {
                for &b in &line[idx..] {
                    joltage = joltage * 10 + u64::from(b - b'0');
                }
                break;
            }
        }

        total += joltage;
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

#[aoc(day3, part2, stack)]
pub fn part2_stack(lines: &[Vec<u8>]) -> u64 {
    const N: usize = 12;
    let mut total = 0;
    let mut stack = Vec::with_capacity(N);

    for digits in lines {
        let len = digits.len();

        for (i, &d) in digits.iter().enumerate() {
            // while we can pop and replacing gives a better (bigger) result
            while !stack.is_empty() && stack.len() + (len - i) > N && stack.last().unwrap() < &d {
                stack.pop();
            }

            if stack.len() < N {
                stack.push(d);
            }
        }

        total += stack
            .iter()
            .fold(0u64, |acc, &b| acc * 10 + u64::from(b - b'0'));
        stack.clear();
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
