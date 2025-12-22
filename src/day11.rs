use ahash::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::sstr::SStr;

// Using an ArrayVec will increase speed, but unncessary because it's already super fast.
// type Vec<T> = arrayvec::ArrayVec<T, 32>;
type String = SStr<3>;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (key, rest) = line.split_once(": ").unwrap();
            let connections = rest.split(' ').map(|s| s.into()).collect::<Vec<_>>();

            (key.into(), connections)
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(inputs: &HashMap<String, Vec<String>>) -> usize {
    pathfinding::prelude::count_paths(
        "you".into(),
        |current| inputs.get(current).cloned().unwrap_or_default(),
        |current| current == "out",
    )
}

#[aoc(day11, part2)]
pub fn part2(inputs: &HashMap<String, Vec<String>>) -> usize {
    pathfinding::prelude::count_paths(
        ("svr".into(), false, false),
        |current| {
            if let Some(neighbors) = inputs.get(&current.0) {
                neighbors
                    .iter()
                    .map(move |&x| match x.as_str() {
                        "dac" => (x, true, current.2),
                        "fft" => (x, current.1, true),
                        _ => (x, current.1, current.2),
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        },
        |current| current.0 == "out" && current.1 && current.2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const SAMPLE2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE2)), 2);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day11.txt");
        const ANSWERS: (usize, usize) = (607, 506264456238938);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
