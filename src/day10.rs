use aoc_runner_derive::{aoc, aoc_generator};
use good_lp::{
    Expression, IntoAffineExpression, ProblemVariables, Solution, SolverModel, variable,
};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{one_of, space0, space1},
    combinator,
    multi::{many0, separated_list0},
    sequence::delimited,
};

use crate::common::nom::{nom_lines, nom_usize, process_input};

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    target_indicator: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn fewest_p1(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let s = vec![false; target.len()];
    let output = pathfinding::prelude::bfs(
        &s,
        |state| {
            let mut output = Vec::with_capacity(buttons.len());
            for presses in buttons {
                let mut new_state = state.clone();
                for &pos in presses {
                    new_state[pos] = !state[pos];
                }
                output.push(new_state);
            }

            output
        },
        |state| state == target,
    );

    output.unwrap().len() - 1
}

fn fewest_p2(joltages: &[usize], buttons: &[Vec<usize>]) -> usize {
    let mut vars = ProblemVariables::new();

    // Press Counts represents the number of times each button is pressed
    // Without specifying a floor, the minimum number of presses is
    // negative infinity.
    let press_counts: Vec<_> = (0..(buttons.len()))
        .map(|_| {
            // register each button's press count as a variable with the solver
            // keep track of the variables so we can state the objective in terms
            // of them below
            vars.add(variable().min(0).integer())
        })
        .collect();

    // We state the problem: smallest sum of all button presses
    let mut problem = good_lp::highs(vars.minimise(press_counts.iter().sum::<Expression>()));

    // Available solvers listed at https://docs.rs/crate/good_lp/latest
    // Use of the `highs`` solver as it:
    // - Requires no extra libraries (ruling out coin_cbc)
    // - Supports integers (clarabel doesn't support these, despite the
    //   docstring, highs does)
    // - Gives the right answer (sorry microlp)
    // - Is "fast"
    // - Doesn't require any additional mucking about to get it working
    //   (most of the others)

    // The value of each joltage counter is derived from the buttons pressed
    // we have one expression per counter
    let mut expressions = vec![0.into_expression(); joltages.len()];

    for (button, presses) in buttons.iter().zip(press_counts.iter()) {
        for &x in button.iter() {
            // for each button pressed, add the number of times it is pressed
            // to the total for the joltage counters it increments
            expressions[x] += presses;
        }
    }

    for (e, &j) in expressions.into_iter().zip(joltages.iter()) {
        // for each of the expressions for a given joltage counter's value,
        // add the constraint that the result of the expression must be the desired
        // joltage
        problem.add_constraint(e.eq(j as f64));
    }

    let solution = problem.solve().unwrap();
    press_counts.iter().map(|&v| solution.value(v)).sum::<f64>() as usize
}

fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (s, target_indicator) = delimited(
        tag("["),
        many0(combinator::map(one_of("#."), |c| c == '#')),
        tag("]"),
    )
    .parse(s)?;

    let (s, _) = space1(s)?;

    let (s, buttons) = separated_list0(
        tag(" "),
        delimited(tag("("), separated_list0(tag(","), nom_usize), tag(")")),
    )
    .parse(s)?;

    let (s, _) = space0(s)?;

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
        .map(|m| fewest_p1(&m.target_indicator, &m.buttons))
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(inputs: &[Machine]) -> usize {
    inputs
        .iter()
        .map(|m| fewest_p2(&m.joltages, &m.buttons))
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
