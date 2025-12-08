use std::cmp::Reverse;

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{connected_components, kruskal};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

fn euclidean_distance(coord_a: &Coords, coord_b: &Coords) -> usize {
    let dx = coord_a.x.abs_diff(coord_b.x);
    let dy = coord_a.y.abs_diff(coord_b.y);
    let dz = coord_a.z.abs_diff(coord_b.z);

    (dx * dx) + (dy * dy) + (dz * dz)
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> (Vec<Coords>, Vec<(Coords, Coords, usize)>) {
    let inputs: Vec<Coords> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|num| num.parse::<usize>().unwrap());
            Coords {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            }
        })
        .collect();

    // find the pairs coordinates that are the closest to another coordinate
    let mut pairs = Vec::new();

    for (i, coord_a) in inputs.iter().enumerate() {
        for coord_b in inputs.iter().skip(i + 1) {
            let dist = euclidean_distance(coord_a, coord_b);
            pairs.push((*coord_a, *coord_b, dist));
        }
    }
    pairs.sort_unstable_by_key(|(_, _, dist)| *dist);

    (inputs, pairs)
}

fn part1_solve<const COUNT: usize>(inputs: &[Coords], pairs: &[(Coords, Coords, usize)]) -> usize {
    let mut sizes: Vec<_> = connected_components(inputs, |&n| {
        pairs.iter().take(COUNT).filter_map(move |elem| {
            if elem.0 == n {
                Some(elem.1)
            } else if elem.1 == n {
                Some(elem.0)
            } else {
                None
            }
        })
    })
    .into_iter()
    .map(|comp| comp.len())
    .collect();

    sizes.sort_unstable_by_key(|&a| Reverse(a));
    sizes.iter().take(3).product()
}

#[aoc(day8, part1)]
pub fn part1((inputs, pairs): &(Vec<Coords>, Vec<(Coords, Coords, usize)>)) -> usize {
    part1_solve::<1000>(inputs, pairs)
}

#[aoc(day8, part2)]
pub fn part2((_inputs, pairs): &(Vec<Coords>, Vec<(Coords, Coords, usize)>)) -> usize {
    let last_edge = kruskal(pairs).last().unwrap();

    last_edge.0.x * last_edge.1.x
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        let (a, b) = &generator(SAMPLE);
        assert_eq!(part1_solve::<10>(a, b), 40);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 25272);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day8.txt");
        const ANSWERS: (usize, usize) = (98696, 2245203960);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
