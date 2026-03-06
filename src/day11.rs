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

/// Counts paths to "out" with specific visitor requirements using domain-specific optimization.
///
/// This function is much faster than the general-purpose count_paths because it:
/// 1. **Encodes state directly in return values** instead of exploring all state combinations:
///    - count.0: total paths from this node to "out"
///    - count.1: paths that have visited "dac" (node marker)
///    - count.2: paths that have visited both "dac" AND "fft" (final state)
///
/// 2. **Avoids state explosion**: count_paths would require the use of a tuple to keep track
///    of (location, visited_dac, visited_fft) and explores every combination. This creates many
///    duplicate states. count_part2 instead aggregates all paths upward, then marks states at
///    the important nodes ("dac", "fft").
///
/// 3. **Single memoization pass**: Caches only one value per node instead of many (location, bool, bool)
///    combinations, dramatically reducing cache lookups and memory.
///
/// 4. **Linear traversal**: Works bottom-up from "out", accumulating path information, rather
///    than top-down exploration generating all possible state permutations.
fn count_part2(
    inputs: &HashMap<String, Vec<String>>,
    s: &String,
    memo: &mut HashMap<String, (usize, usize, usize)>,
) -> (usize, usize, usize) {
    // Return cached result if already computed to avoid recalculation
    if let Some(&cached) = memo.get(s) {
        return cached;
    }

    // Base case: reaching "out" is 1 valid path with no special visitors yet
    if s == "out" {
        return (1, 0, 0);
    }

    // Recursively sum path counts from all neighbors
    let mut count = (0, 0, 0);
    if let Some(neighbors) = inputs.get(s) {
        for neighbor in neighbors {
            let (c1, c2, c3) = count_part2(inputs, neighbor, memo);
            count.0 += c1; // Accumulate all paths to "out"
            count.1 += c2; // Accumulate paths that visited "dac"
            count.2 += c3; // Accumulate paths that visited both "dac" and "fft"
        }
    }

    // Mark the "dac" node: all paths from here count as having visited "dac"
    if s == "dac" {
        count.1 = count.0;
    }

    // Mark the "fft" node: count.1 (paths that visited "dac") now qualify as visited both
    if s == "fft" {
        count.2 = count.1;
    }

    // Cache this node's result for future lookups
    memo.insert(*s, count);
    count
}

#[aoc(day11, part2)]
pub fn part2(inputs: &HashMap<String, Vec<String>>) -> usize {
    count_part2(inputs, &"svr".into(), &mut Default::default()).2
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
