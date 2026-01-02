use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{one_of, space1},
    multi::{fold_many0, separated_list0},
    sequence::delimited,
};

use crate::common::nom::{nom_lines, nom_u64, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    pub target_indicator: u16,
    /// buttons[i] contains the list of joltage indices that button i affects
    pub buttons: Vec<Vec<usize>>,
    pub joltages: Vec<u64>,
}

fn fewest_p1(target: u16, buttons: &[Vec<usize>]) -> usize {
    // Convert buttons to bitmasks for BFS
    let button_masks: Vec<u16> = buttons
        .iter()
        .map(|indices| indices.iter().fold(0u16, |acc, &idx| acc | (1 << idx)))
        .collect();

    let output = pathfinding::prelude::bfs(
        &0u16,
        |&state| button_masks.iter().map(move |&mask| state ^ mask),
        |&state| state == target,
    );

    output.unwrap().len() - 1
}

/// Solves part 2 using Z3 SMT solver to minimize button presses.
///
/// Models the problem as: find non-negative integer multipliers x_i for each button
/// such that sum(x_i * button_presses_j) = target_j for all joltage counters j,
/// minimizing sum(x_i).
fn solve_p2_z3(buttons: &[Vec<usize>], joltages: &[u64]) -> u64 {
    use z3::ast::Int;
    use z3::{Optimize, SatResult};

    let opt = Optimize::new();

    // Pre-build reverse mapping: joltage index -> list of button indices that affect it
    let mut joltage_to_buttons = vec![Vec::new(); joltages.len()];
    for (button_idx, indices) in buttons.iter().enumerate() {
        for &joltage_idx in indices {
            joltage_to_buttons[joltage_idx].push(button_idx);
        }
    }

    // Create integer variables for number of times each button is pressed
    // and assert each is non-negative
    let button_presses: Vec<_> = (0..buttons.len())
        .map(|i| {
            let var = Int::new_const(i as u32);
            opt.assert(&var.ge(0));
            var
        })
        .collect();

    // For each joltage counter, sum of button contributions must equal target
    for (&target, contributing_buttons) in joltages.iter().zip(&joltage_to_buttons) {
        if contributing_buttons.is_empty() {
            // No button affects this joltage - target must be 0
            opt.assert(&Int::from_u64(target).eq(0));
        } else {
            let sum: Int = contributing_buttons
                .iter()
                .map(|&i| &button_presses[i])
                .sum();
            opt.assert(&sum.eq(target));
        }
    }

    // Minimize total button presses
    let total: Int = button_presses.into_iter().sum();
    opt.minimize(&total);

    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .and_then(|model| model.eval(&total, true))
            .and_then(|res| res.as_u64())
            .unwrap(),
        _ => panic!("No solution found"),
    }
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
        delimited(tag("("), separated_list0(tag(","), nom_usize), tag(")")),
    )
    .parse(s)?;

    let (s, _) = space1(s)?;

    let (s, joltages) =
        delimited(tag("{"), separated_list0(tag(","), nom_u64), tag("}")).parse(s)?;

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
pub fn part2(inputs: &[Machine]) -> u64 {
    inputs
        .iter()
        .map(|m| solve_p2_z3(&m.buttons, &m.joltages))
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
        const ANSWERS: (usize, u64) = (452, 17424);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
