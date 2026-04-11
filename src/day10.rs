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
type ButtonEffect = [u16; MAX_BUTTONS];

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    pub target_indicator: u16,
    pub buttons: Vec<u16>,
    pub joltages: ArrayVec<u16, MAX_BUTTONS>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parity(pub u16);

impl Parity {
    fn calculate_parity(buttons: &[u16]) -> Self {
        let mut parity = 0u16;
        for (i, &x) in buttons.iter().enumerate() {
            if x % 2 == 1 {
                parity |= 1 << i;
            }
        }

        Parity(parity)
    }
}

fn fewest_p1(target: u16, buttons: &[u16]) -> usize {
    let output = pathfinding::prelude::bfs(
        &0u16,
        |&state| buttons.iter().map(move |&button_mask| state ^ button_mask),
        |&state| state == target,
    );

    output.unwrap().len() - 1
}

/// Pre-computes all possible increment effects achievable by button combinations.
///
/// Iterates through all 2^n combinations of buttons and computes the resulting
/// increment effect for each. Effects are indexed by their parity for
/// efficient lookup during solving.
///
/// # Returns
///
/// `HashMap<Parity, Vec<(increment_effect, min_button_presses)>>`
///
/// The parity-indexed structure enables divide-and-conquer: when we subtract an
/// effect and divide by 2, the parity changes, so we match against different
/// effect sets at each recursion level.
fn compute_effects(btn_coeffs: &[u16]) -> HashMap<Parity, Vec<(ButtonEffect, usize)>> {
    let mut res: HashMap<_, Vec<_>> = HashMap::new();

    // For each number of pressed buttons (0, 1, 2, ..., num_buttons)
    for num_pressed in 0..=btn_coeffs.len() {
        // For each combination of buttons to press
        for btn_combinations in (0..btn_coeffs.len()).combinations(num_pressed) {
            let (effect, parity) = compute_effect(btn_coeffs, &btn_combinations);

            // Only store if we haven't seen this effect for this parity before
            // This was previously a HashMap of HashMap's but this is slightly better performance.
            let entry = res.entry(parity).or_default();
            if entry.iter().all(|(e, _)| e != &effect) {
                entry.push((effect, num_pressed));
            }
        }
    }

    res
}

/// Computes the increment effect and parity for a button combination.
///
/// For each button pressed, XORs its bitmask into `parity` and uses
/// sparse bit iteration (trailing_zeros + clear-lowest-bit) to increment
/// only the affected counters in `effect`.
///
/// # Returns
///
/// * `effect[i]` - number of times counter `i` is incremented
/// * `Parity` - bitmask where bit `i` = 1 if `effect[i]` is odd
pub fn compute_effect(btn_coeffs: &[u16], btn_combinations: &[usize]) -> (ButtonEffect, Parity) {
    let mut effect = [0u16; MAX_BUTTONS];
    let mut parity = 0u16;

    for &btn_comb_idx in btn_combinations {
        // SAFETY: btn_comb_idx is guaranteed to be within bounds of btn_coeffs slice
        let btn_mask = unsafe { *btn_coeffs.get_unchecked(btn_comb_idx) };

        // Parity is simply XOR of all masks - O(1) per button
        parity ^= btn_mask;

        // Sparse bit iteration: only visit set bits - O(popcount) per button
        let mut remaining = btn_mask;
        while remaining != 0 {
            let joltage_idx = remaining.trailing_zeros() as usize;
            effect[joltage_idx] += 1;
            remaining &= remaining - 1; // Clear lowest set bit
        }
    }

    (effect, Parity(parity))
}

/// Finds minimum button presses to reach a goal state using divide-and-conquer.
///
/// # Algorithm
///
/// 1. Pre-compute all effects indexed by parity via `compute_effects()`
/// 2. Recursively decompose the goal:
///    - Find effects matching the goal's parity (so subtraction yields even values)
///    - For each valid effect: `new_goal = (goal - effect) / 2`
///    - Cost = effect_cost + 2 × recurse(new_goal)
/// 3. Base case: goal is all zeros → cost 0
///
/// Uses memoization to cache subproblem solutions.
fn solve_p2(btn_coeffs: &[u16], goal: &[u16]) -> usize {
    let effect_costs = compute_effects(btn_coeffs);
    let mut cache = Default::default();

    // Convert goal slice to ArrayVec
    let goal_arr = goal.iter().copied().collect();

    fn solve_aux(
        goal: ArrayVec<u16, MAX_BUTTONS>,
        effect_cost_lookup: &HashMap<Parity, Vec<(ButtonEffect, usize)>>,
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

        let parity = Parity::calculate_parity(&goal);
        let mut answer = None;

        // Look up and try each (effect, cost) pair for this parity
        if let Some(effect_cost_pairs) = effect_cost_lookup.get(&parity) {
            for (effect, cost) in effect_cost_pairs {
                // The effect is always MAX_BUTTONS in length, verify unused portion is zero
                let (effect, rest) = effect.split_at(goal.len());
                debug_assert!(rest.iter().all(|&x| x == 0));

                // Check if effect fits within goal
                if goal.iter().zip(effect.iter()).all(|(&g, &e)| g >= e) {
                    // Calculate new goal: (goal - effect) / 2
                    // goal - effect is even because both goal and effect have the same parity
                    let new_goal = goal
                        .iter()
                        .zip(effect.iter())
                        .map(|(&g, &e)| (g - e) / 2)
                        .collect();

                    // Recurse with new goal, multiply cost by 2
                    if let Some(recursed_cost) = solve_aux(new_goal, effect_cost_lookup, cache) {
                        let cost = cost + recursed_cost * 2;
                        answer = Some(answer.map_or(cost, |x| cost.min(x)));
                    }
                }
            }
        }

        cache.insert(goal, answer);
        answer
    }

    solve_aux(goal_arr, &effect_costs, &mut cache).unwrap_or(usize::MAX)
}

fn parse_target_indicator(s: &str) -> IResult<&str, u16> {
    delimited(
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
    .parse(s)
}

fn parse_buttons(s: &str) -> IResult<&str, Vec<u16>> {
    separated_list0(
        tag(" "),
        delimited(
            tag("("),
            fold_separated_list0(tag(","), nom_usize, || 0u16, |acc, idx| acc | (1 << idx)),
            tag(")"),
        ),
    )
    .parse(s)
}

fn parse_joltages(s: &str) -> IResult<&str, ArrayVec<u16, MAX_BUTTONS>> {
    delimited(
        tag("{"),
        fold_separated_list0(tag(","), nom_u16, ArrayVec::new, |mut acc, item| {
            acc.push(item);
            acc
        }),
        tag("}"),
    )
    .parse(s)
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, target_indicator) = parse_target_indicator(s)?;
    let (s, _) = space1(s)?;
    let (s, buttons) = parse_buttons(s)?;
    let (s, _) = space1(s)?;
    let (s, joltages) = parse_joltages(s)?;

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
