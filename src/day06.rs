use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6, part1)]
pub fn generator_p1(input: &str) -> (Vec<Vec<u64>>, Vec<u8>) {
    let mut iter = input.lines().rev();

    let ops = iter
        .next()
        .unwrap()
        .bytes()
        .filter(|&b| b != b' ')
        .collect::<Vec<u8>>();

    let mut output: Vec<Vec<u64>> = Vec::new();
    for line in iter {
        let nums = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        output.push(nums);
    }

    (output, ops)
}

#[aoc_generator(day6, part2)]
pub fn generator_p2(input: &str) -> (Vec<Vec<u64>>, Vec<u8>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();

    debug_assert!(lines.iter().all(|&line| line.len() == lines[0].len()));
    let mut output = Vec::new();
    let mut ops = Vec::new();
    let mut group = Vec::new();

    for col in (0..lines[0].len()).rev() {
        let num = lines[..(lines_len - 1)]
            .iter()
            .map(|&line| line.as_bytes()[col] as char)
            .collect::<String>();

        let n_trimmed = num.trim();
        if !n_trimmed.is_empty() {
            group.push(n_trimmed.parse::<u64>().unwrap());
        }

        let op = lines.last().unwrap().as_bytes()[col];
        if op == b'*' || op == b'+' {
            // process group
            output.push(group);
            ops.push(op);
            group = Vec::new();
        }
    }

    (output, ops)
}

#[aoc(day6, part1)]
pub fn part1((nums, ops): &(Vec<Vec<u64>>, Vec<u8>)) -> u64 {
    let mut total = 0;

    for (i, op) in ops.iter().enumerate() {
        let column = nums.iter().map(|row| row[i]);

        let column_total: u64 = match op {
            b'+' => column.sum(),
            b'*' => column.product(),
            _ => panic!("Unknown operator {}", *op as char),
        };

        total += column_total;
    }

    total
}

#[aoc(day6, part2)]
pub fn part2((nums, ops): &(Vec<Vec<u64>>, Vec<u8>)) -> u64 {
    let mut total = 0;

    for (op, group) in ops.iter().zip(nums.iter()) {
        let group_total: u64 = match op {
            b'+' => group.iter().sum(),
            b'*' => group.iter().product(),
            _ => panic!("Unknown operator {}", *op as char),
        };

        total += group_total;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    // rustfmt::skip
    const SAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator_p1(SAMPLE));
        println!("{:?}", generator_p2(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator_p1(SAMPLE)), 4277556);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator_p2(SAMPLE)), 3263827);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day6.txt");
        const ANSWERS: (u64, u64) = (4387670995909, 9625320374409);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output_p1 = generator_p1(input);
            let output_p2 = generator_p2(input);

            assert_eq!(part1(&output_p1), ANSWERS.0);
            assert_eq!(part2(&output_p2), ANSWERS.1);
        }
    }
}
