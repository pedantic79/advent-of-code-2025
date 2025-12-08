use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Coords {
    x: usize,
    y: usize,
    z: usize,
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    // This is faster for part1 but not part2
    // this uses path splitting
    fn find_p1(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            (x, self.parent[x]) = (self.parent[x], self.parent[self.parent[x]]);
        }
        x
    }

    fn union_p1(&mut self, x: usize, y: usize) -> bool {
        let px = self.find_p1(x);
        let py = self.find_p1(y);
        if px == py {
            return false;
        }
        self.parent[px] = py;
        true
    }

    // this uses path compression
    fn find_p2(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_p2(self.parent[x]);
        }
        self.parent[x]
    }

    fn union_p2(&mut self, x: usize, y: usize) -> bool {
        let px = self.find_p2(x);
        let py = self.find_p2(y);
        if px == py {
            return false;
        }
        self.parent[px] = py;
        true
    }
}

fn euclidean_distance(coord_a: &Coords, coord_b: &Coords) -> usize {
    let dx = coord_a.x.abs_diff(coord_b.x);
    let dy = coord_a.y.abs_diff(coord_b.y);
    let dz = coord_a.z.abs_diff(coord_b.z);

    (dx * dx) + (dy * dy) + (dz * dz)
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> (Vec<Coords>, Vec<(usize, usize, usize)>) {
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
        for (j, coord_b) in inputs.iter().enumerate().skip(i + 1) {
            let dist = euclidean_distance(coord_a, coord_b);
            pairs.push((dist, i, j));
        }
    }
    pairs.sort_unstable_by_key(|(dist, _, _)| *dist);

    (inputs, pairs)
}

fn part1_solve<const COUNT: usize>(inputs: &[Coords], pairs: &[(usize, usize, usize)]) -> usize {
    let mut uf = UnionFind::new(inputs.len());
    for (_, i, j) in pairs.iter().take(COUNT).copied() {
        uf.union_p1(i, j);
    }

    // build circuit counts from parent groups
    let mut circuits = vec![0; inputs.len()];
    for i in 0..inputs.len() {
        circuits[uf.find_p1(i)] += 1;
    }

    // Get longest 3 circuits
    circuits.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    circuits.into_iter().take(3).product()
}

#[aoc(day8, part1)]
pub fn part1((inputs, pairs): &(Vec<Coords>, Vec<(usize, usize, usize)>)) -> usize {
    part1_solve::<1000>(inputs, pairs)
}

#[aoc(day8, part2)]
pub fn part2((inputs, pairs): &(Vec<Coords>, Vec<(usize, usize, usize)>)) -> usize {
    let mut last_union = (0, 1);
    let mut uf = UnionFind::new(inputs.len());
    for (_, i, j) in pairs.iter().copied() {
        if uf.union_p2(i, j) {
            last_union = (i, j);
        }
    }

    // Multiply x coordinates of the last union
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
