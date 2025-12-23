use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{one_of, space1},
    combinator,
    multi::{many0, separated_list0},
    sequence::delimited,
};

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    pub target_indicator: u16,
    pub buttons: Vec<u16>,
    pub joltages: Vec<usize>,
}

fn fewest_p1(target: u16, buttons: &[u16]) -> usize {
    let output = pathfinding::prelude::bfs(
        &0u16,
        |&state| buttons.iter().map(move |&button_mask| state ^ button_mask),
        |&state| state == target,
    );

    output.unwrap().len() - 1
}
/// Generates all possible patterns of joltage counter increments achievable by button combinations.
///
/// This function pre-computes every possible pattern that can be created by pressing
/// different combinations of buttons, organizing them by their parity signature for
/// efficient lookup during the solving phase.
///
/// # Arguments
///
/// * `coeffs` - Button configuration where `coeffs[i]` is a vector of joltage counter indices
///   that button `i` increments
/// * `num_variables` - Total number of joltage counters in the system
///
/// # Returns
///
/// A nested HashMap with the structure:
/// ```text
/// HashMap<parity_pattern, HashMap<increment_pattern, min_button_presses>>
/// ```
///
/// Where:
/// * **Outer key (parity_pattern)**: `Vec<bool>` indicating odd/even for each counter
/// * **Inner key (increment_pattern)**: `Vec<usize>` showing how much each counter increases
/// * **Inner value**: Minimum number of button presses needed to achieve that pattern
///
/// # Why This Structure?
///
/// The parity-indexed structure enables the divide-and-conquer algorithm in `solve_single`.
/// When we subtract a pattern and divide by 2, the parity flips, allowing us to match
/// against different pattern sets at each recursion level. This creates a binary tree
/// search through the solution space.
///
/// # Example
///
/// If buttons are `[[0, 1], [1, 2]]` and we press both:
/// - Pattern: `[1, 2, 1]` (counter 0 incremented once, counter 1 twice, counter 2 once)
/// - Parity: `[true, false, true]` (odd, even, odd)
/// - Cost: 2 button presses
///
/// This gets stored as: `result[vec![true, false, true]][vec![1, 2, 1]] = 2`
fn patterns(coeffs: &[u16], num_variables: usize) -> HashMap<u16, HashMap<Vec<usize>, usize>> {
    let mut res = HashMap::new();

    // For each number of pressed buttons (0, 1, 2, ..., num_buttons)
    for num_pressed in 0..=coeffs.len() {
        // For each combination of buttons to press
        for buttons in (0..coeffs.len()).combinations(num_pressed) {
            let (pattern, parity_pattern) = build_pattern(coeffs, num_variables, buttons);

            // Only store if we haven't seen this pattern for this parity before
            // (or if this achieves it with fewer button presses)
            res.entry(parity_pattern)
                .or_insert_with(HashMap::new)
                .entry(pattern)
                .or_insert(num_pressed);
        }
    }

    res
}

/// Builds a pattern vector and its corresponding parity pattern for a given button combination.
///
/// # Arguments
///
/// * `coeffs` - The button configuration: each button is a vector of joltage counter indices it increments
/// * `num_variables` - The total number of joltage counters
/// * `buttons` - The indices of buttons being pressed in this combination
///
/// # Returns
///
/// A tuple containing:
/// * `pattern` - A vector where `pattern[i]` is the number of times joltage counter `i` gets incremented
/// * `parity_pattern` - A vector where `parity_pattern[i]` is true if `pattern[i]` is odd, false if even
///
/// # How it works
///
/// For each selected button, we iterate through all the joltage counters it affects and:
/// 1. Increment the counter in `pattern` (counting how many times that counter is pressed)
/// 2. Toggle the corresponding bit in `parity_pattern` (XOR with true flips even↔odd)
///
/// The parity pattern is crucial for the divide-and-conquer algorithm because it determines
/// which patterns can be subtracted from a given goal state at each recursion level.
pub fn build_pattern(
    coeffs: &[u16],
    num_variables: usize,
    buttons: Vec<usize>,
) -> (Vec<usize>, u16) {
    // Calculate the pattern: how many times each joltage counter is incremented
    let mut pattern = vec![0; num_variables];

    // Calculate parity pattern as u16 bitmask (1 = odd, 0 = even)
    let mut parity_pattern = 0u16;

    for button_mask in buttons
        .into_iter()
        .map(|i| unsafe { coeffs.get_unchecked(i) })
    {
        for (joltage_idx, p) in pattern.iter_mut().enumerate() {
            let offset = 1 << joltage_idx;
            if button_mask & offset != 0 {
                *p += 1;
                parity_pattern ^= offset;
            }
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

/// Solves the minimum button presses needed to reach a goal state using divide-and-conquer.
///
/// # Problem Statement
///
/// Given a set of buttons where each button increments specific joltage counters by 1,
/// find the minimum number of button presses needed to reach the exact goal state
/// (each counter must match its target value exactly).
///
/// # Algorithm: Divide and Conquer with Parity Matching
///
/// The key insight is that we can work backwards from the goal using a binary decomposition:
///
/// 1. **Pattern Generation**: Pre-compute all possible patterns we can create by pressing
///    different combinations of buttons (0 to all buttons). Store patterns indexed by their
///    parity (even/odd for each counter).
///
/// 2. **Recursive Decomposition**: For each goal state:
///    - Find patterns that match the goal's parity
///    - For each matching pattern that's ≤ goal in all dimensions:
///      - Subtract it: `new_goal = (goal - pattern) / 2`
///      - Recurse on the reduced goal
///      - Cost = buttons to create pattern + 2 × cost to reach new_goal
///    - Return the minimum cost across all options
///
/// 3. **Why it works**: After subtracting a pattern with parity P and dividing by 2,
///    the result has complementary parity (not P). This allows us to apply different
///    patterns at each level, effectively searching a binary tree of possibilities.
///
/// 4. **Base case**: When goal is all zeros, we've successfully decomposed the original
///    goal, so the cost is 0 (we've already counted button presses at each level).
///
/// # Arguments
///
/// * `coeffs` - Button configuration where `coeffs[i]` lists which counters button `i` increments
/// * `goal` - Target values for each joltage counter
///
/// # Returns
///
/// The minimum number of button presses needed to exactly reach the goal state.
/// Returns `usize::MAX` (via saturating arithmetic) if the goal is unreachable.
///
/// # Example
///
/// If we can reach pattern [2, 1] with 2 button presses, and the goal is [4, 2]:
/// - Subtract [2, 1]: goal becomes [2, 1]
/// - Divide by 2: new_goal becomes [1, 0] (integer division)
/// - Recursively solve for [1, 0]
/// - Total cost = 2 + 2 × cost([1, 0])
///
/// # Implementation Details
///
/// Uses memoization (caching) to avoid recomputing solutions for the same goal state.
/// The saturating arithmetic ensures that unreachable goals don't cause overflow panics.
fn solve_single(coeffs: &[u16], goal: &[usize]) -> usize {
    let pattern_costs = patterns(coeffs, goal.len());
    let mut cache = Default::default();

    fn solve_aux(
        goal: Vec<usize>,
        pattern_costs: &HashMap<u16, HashMap<Vec<usize>, usize>>,
        cache: &mut HashMap<Vec<usize>, Option<usize>>,
    ) -> Option<usize> {
        // Base case: all zeros
        if goal.iter().all(|&x| x == 0) {
            return Some(0);
        }

        // Check cache
        if let Some(&cached) = cache.get(&goal) {
            return cached;
        }

        // Get parity pattern for current goal as u16 bitmask
        let parity_pattern =
            goal.iter().enumerate().fold(
                0u16,
                |acc, (i, &x)| if x % 2 == 1 { acc | (1 << i) } else { acc },
            );

        let mut answer = None;

        // Try all patterns that match the parity
        if let Some(patterns_for_parity) = pattern_costs.get(&parity_pattern) {
            for (pattern, &pattern_cost) in patterns_for_parity {
                // Check if pattern fits within goal
                if pattern.iter().zip(goal.iter()).all(|(&p, &g)| p <= g) {
                    // Calculate new goal: (goal - pattern) / 2
                    let new_goal: Vec<_> = pattern
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

    solve_aux(goal.to_vec(), &pattern_costs, &mut cache).unwrap_or(usize::MAX)
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, bits) = delimited(
        tag("["),
        many0(combinator::map(one_of("#."), |c| c == '#')),
        tag("]"),
    )
    .parse(s)?;

    // Convert Vec<bool> to u16 by setting bits
    let target_indicator = bits
        .iter()
        .enumerate()
        .fold(0u16, |acc, (i, &b)| if b { acc | (1 << i) } else { acc });

    let (s, _) = space1(s)?;

    let (s, button_vecs) = separated_list0(
        tag(" "),
        delimited(tag("("), separated_list0(tag(","), nom_usize), tag(")")),
    )
    .parse(s)?;

    // Convert each button's Vec<usize> to u16 bitmask
    let buttons: Vec<u16> = button_vecs
        .iter()
        .map(|indices| indices.iter().fold(0u16, |acc, &idx| acc | (1 << idx)))
        .collect();

    let (s, _) = space1(s)?;

    let (s, joltages) =
        delimited(tag("{"), separated_list0(tag(","), nom_usize), tag("}")).parse(s)?;

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
        .map(|m| solve_single(&m.buttons, &m.joltages))
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
