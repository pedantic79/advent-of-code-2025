use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{one_of, space1},
    multi::{fold_many0, separated_list0},
    sequence::delimited,
};

use crate::common::nom::{fold_separated_list0, nom_lines, nom_u16, nom_usize, process_input};

const MAX_BUTTONS: usize = 16;

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    pub target_indicator: u16,
    pub buttons: Vec<u16>,
    pub joltages: ArrayVec<u16, MAX_BUTTONS>,
}

fn fewest_p1(target: u16, buttons: &[u16]) -> usize {
    let output = pathfinding::prelude::bfs(
        &0u16,
        |&state| buttons.iter().map(move |&button_mask| state ^ button_mask),
        |&state| state == target,
    );

    output.unwrap().len() - 1
}

/// Pre-computes all possible increment patterns achievable by button combinations.
///
/// Iterates through all 2^n combinations of buttons and computes the resulting
/// increment pattern for each. Patterns are indexed by their parity bitmask for
/// efficient lookup during solving.
///
/// # Returns
///
/// `HashMap<parity_bitmask, Vec<(increment_pattern, min_button_presses)>>`
///
/// The parity-indexed structure enables divide-and-conquer: when we subtract a
/// pattern and divide by 2, the parity changes, so we match against different
/// pattern sets at each recursion level.
fn patterns(coeffs: &[u16]) -> HashMap<u16, Vec<([u16; MAX_BUTTONS], usize)>> {
    let mut res: HashMap<u16, Vec<([u16; MAX_BUTTONS], usize)>> = HashMap::new();

    // For each number of pressed buttons (0, 1, 2, ..., num_buttons)
    for num_pressed in 0..=coeffs.len() {
        // For each combination of buttons to press
        for buttons in (0..coeffs.len()).combinations(num_pressed) {
            let (pattern, parity_pattern) = build_pattern(coeffs, &buttons);

            // Only store if we haven't seen this pattern for this parity before
            // This was previously a HashMap of HashMap's but this is slightly better performance.
            let patterns_vec = res.entry(parity_pattern).or_default();
            if !patterns_vec.iter().any(|(p, _)| p == &pattern) {
                patterns_vec.push((pattern, num_pressed));
            }
        }
    }

    res
}

/// Computes the increment pattern and parity for a button combination.
///
/// For each button pressed, XORs its bitmask into `parity_pattern` and uses
/// sparse bit iteration (trailing_zeros + clear-lowest-bit) to increment
/// only the affected counters in `pattern`.
///
/// # Returns
///
/// * `pattern[i]` - number of times counter `i` is incremented
/// * `parity_pattern` - u16 bitmask where bit `i` = 1 if `pattern[i]` is odd
pub fn build_pattern(coeffs: &[u16], buttons: &[usize]) -> ([u16; MAX_BUTTONS], u16) {
    let mut pattern = [0u16; MAX_BUTTONS];
    let mut parity_pattern = 0u16;

    for &button_idx in buttons {
        // SAFETY: button_idx is guaranteed to be within bounds of coeffs slice
        let button_mask = unsafe { *coeffs.get_unchecked(button_idx) };

        // Parity is simply XOR of all masks - O(1) per button
        parity_pattern ^= button_mask;

        // Sparse bit iteration: only visit set bits - O(popcount) per button
        let mut remaining = button_mask;
        while remaining != 0 {
            let joltage_idx = remaining.trailing_zeros() as usize;
            pattern[joltage_idx] += 1;
            remaining &= remaining - 1; // Clear lowest set bit
        }
    }

    (pattern, parity_pattern)
}

fn minimum(a: Option<usize>, b: usize) -> Option<usize> {
    match a {
        Some(x) => Some(x.min(b)),
        None => Some(b),
    }
}

/// Finds minimum button presses to reach a goal state using divide-and-conquer.
///
/// # Algorithm
///
/// 1. Pre-compute all patterns indexed by parity via `patterns()`
/// 2. Recursively decompose the goal:
///    - Find patterns matching the goal's parity (so subtraction yields even values)
///    - For each valid pattern: `new_goal = (goal - pattern) / 2`
///    - Cost = pattern_cost + 2 × recurse(new_goal)
/// 3. Base case: goal is all zeros → cost 0
///
/// Uses memoization to cache subproblem solutions.
fn solve_p2(coeffs: &[u16], goal: &[u16]) -> usize {
    let pattern_costs = patterns(coeffs);
    let mut cache = Default::default();

    // Convert goal slice to ArrayVec
    let goal_arr = goal.iter().copied().collect();

    fn solve_aux(
        goal: ArrayVec<u16, MAX_BUTTONS>,
        pattern_costs: &HashMap<u16, Vec<([u16; MAX_BUTTONS], usize)>>,
        cache: &mut HashMap<ArrayVec<u16, MAX_BUTTONS>, Option<usize>>,
    ) -> Option<usize> {
        // Base case: all zeros
        if goal.iter().all(|&x| x == 0) {
            return Some(0);
        }

        // Check cache
        if let Some(&cached) = cache.get(&goal) {
            return cached;
        }

        let num_vars = goal.len();

        // Get parity pattern for current goal as u16 bitmask
        let parity_pattern =
            goal.iter().enumerate().fold(
                0u16,
                |acc, (i, &x)| if x % 2 == 1 { acc | (1 << i) } else { acc },
            );

        let mut answer = None;

        // Try all patterns that match the parity
        if let Some(patterns_for_parity) = pattern_costs.get(&parity_pattern) {
            for (pattern, pattern_cost) in patterns_for_parity {
                // Check if pattern fits within goal
                if pattern[..num_vars]
                    .iter()
                    .zip(goal.iter())
                    .all(|(&p, &g)| p <= g)
                {
                    // Calculate new goal: (goal - pattern) / 2
                    let new_goal = pattern[..num_vars]
                        .iter()
                        .zip(goal.iter())
                        .map(|(&p, &g)| (g - p) / 2)
                        .collect();

                    // Recurse with new goal, multiply cost by 2
                    if let Some(recursed_cost) = solve_aux(new_goal, pattern_costs, cache) {
                        answer = minimum(answer, pattern_cost + recursed_cost * 2);
                    }
                }
            }
        }

        cache.insert(goal, answer);
        answer
    }

    solve_aux(goal_arr, &pattern_costs, &mut cache).unwrap_or(usize::MAX)
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, target_indicator) = delimited(
        tag("["),
        fold_many0(
            one_of("#."),
            || (0u16, 0usize),
            |(acc, idx), c| {
                let new_acc = if c == '#' { acc | (1 << idx) } else { acc };
                (new_acc, idx + 1)
            },
        ),
        tag("]"),
    )
    .map(|(acc, _)| acc)
    .parse(s)?;

    let (s, _) = space1(s)?;

    let (s, buttons) = separated_list0(
        tag(" "),
        delimited(
            tag("("),
            fold_separated_list0(tag(","), nom_usize, || 0u16, |acc, idx| acc | (1 << idx)),
            tag(")"),
        ),
    )
    .parse(s)?;

    let (s, _) = space1(s)?;

    let (s, joltages) = delimited(
        tag("{"),
        fold_separated_list0(tag(","), nom_u16, ArrayVec::new, |mut acc, item| {
            acc.push(item);
            acc
        }),
        tag("}"),
    )
    .parse(s)?;

    Ok((
        s,
        Machine {
            target_indicator,
            buttons,
            joltages,
        },
    ))
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Machine> {
    process_input(nom_lines(parse_machine))(input)
}

#[aoc(day10, part1)]
pub fn part1(inputs: &[Machine]) -> usize {
    inputs
        .iter()
        .map(|m| fewest_p1(m.target_indicator, &m.buttons))
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Machine]) -> usize {
    inputs
        .iter()
        .map(|m| solve_p2(&m.buttons, &m.joltages))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 7);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 33);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day10.txt");
        const ANSWERS: (usize, usize) = (452, 17424);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
