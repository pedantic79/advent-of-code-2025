use ahash::{HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

fn eucldean_distance(coord_a: &Coords, coord_b: &Coords) -> usize {
    let dx = coord_a.x.abs_diff(coord_b.x);
    let dy = coord_a.y.abs_diff(coord_b.y);
    let dz = coord_a.z.abs_diff(coord_b.z);

    (dx * dx) + (dy * dy) + (dz * dz)
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|num| num.parse::<usize>().unwrap());
            Coords {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            }
        })
        .collect()
}

fn part1_solve<const MAX: usize>(inputs: &[Coords]) -> usize {
    // find the pairs coordinates that are the closest to another coordinate
    let mut pairs = Vec::new();

    for (i, coord_a) in inputs.iter().enumerate() {
        for (j, coord_b) in inputs.iter().enumerate().skip(i + 1) {
            let dist = eucldean_distance(coord_a, coord_b);
            pairs.push((dist, i, j));
        }
    }
    pairs.sort_by_key(|(dist, _, _)| *dist);

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for (_, i, j) in pairs[0..MAX].iter().copied() {
        let found_i = circuits.iter().position(|circuit| circuit.contains(&i));
        let found_j = circuits.iter().position(|circuit| circuit.contains(&j));

        match (found_i, found_j) {
            (Some(ci), Some(cj)) if ci != cj => {
                let to_merge = circuits[cj].clone();
                circuits[ci].extend(to_merge.into_iter());
                circuits.remove(cj);
            }
            (Some(ci), None) => {
                circuits[ci].insert(j);
            }
            (None, Some(cj)) => {
                circuits[cj].insert(i);
            }
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(i);
                new_circuit.insert(j);
                circuits.push(new_circuit);
            }
            _ => {}
        }
    }

    circuits.sort_by_key(|x| std::cmp::Reverse(x.len()));
    circuits[0..3].iter().map(|c| c.len()).product()
}

#[aoc(day8, part1)]
pub fn part1(inputs: &[Coords]) -> usize {
    part1_solve::<1000>(inputs)
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[Coords]) -> usize {
    let mut pairs = Vec::new();

    for (i, coord_a) in inputs.iter().enumerate() {
        for (j, coord_b) in inputs.iter().enumerate().skip(i + 1) {
            let dist = eucldean_distance(coord_a, coord_b);
            pairs.push((dist, i, j));
        }
    }
    pairs.sort_by_key(|(dist, _, _)| *dist);

    let mut last_union = (0, 1);
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for (_, i, j) in pairs.iter().copied() {
        let found_i = circuits.iter().position(|circuit| circuit.contains(&i));
        let found_j = circuits.iter().position(|circuit| circuit.contains(&j));

        match (found_i, found_j) {
            (Some(ci), Some(cj)) if ci != cj => {
                let to_merge = circuits[cj].clone();
                circuits[ci].extend(to_merge.into_iter());
                circuits.remove(cj);
            }
            (Some(ci), None) => {
                circuits[ci].insert(j);
            }
            (None, Some(cj)) => {
                circuits[cj].insert(i);
            }
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(i);
                new_circuit.insert(j);
                circuits.push(new_circuit);
            }
            _ => {}
        }

        if circuits[0].len() == inputs.len() {
            last_union = (i, j);
            break;
        }
    }

    inputs[last_union.0].x * inputs[last_union.1].x
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
        assert_eq!(part1_solve::<10>(&generator(SAMPLE)), 40);
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
