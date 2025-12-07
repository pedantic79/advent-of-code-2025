use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Map {
    let data = input.as_bytes().to_vec();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    Map {
        data,
        width,
        height,
    }
}

#[aoc(day7, part1)]
pub fn part1(map: &Map) -> usize {
    let mut beams = Vec::with_capacity(map.width);
    let mut new_beams = Vec::with_capacity(map.width);
    new_beams.push(map.width / 2);
    let mut count = 0;

    for row in 1..map.height {
        std::mem::swap(&mut beams, &mut new_beams);
        new_beams.clear();
        beams.sort_unstable();
        beams.dedup();

        for &beam in beams.iter() {
            if map.data[(map.width + 1) * row + beam] == b'^' {
                new_beams.push(beam + 1);
                new_beams.push(beam - 1);
                count += 1;
            } else {
                new_beams.push(beam);
            }
        }
    }

    count
}

fn num_worlds(col: usize, row: usize, map: &Map, memo: &mut Vec<usize>) -> usize {
    if row >= map.height {
        return 1;
    }

    let index = row * (map.width + 1) + col;
    let x = memo[index];
    if x > 0 {
        return x;
    }

    let row = row + 1;
    let count = if map.data[index] == b'^' {
        num_worlds(col + 1, row, map, memo) + num_worlds(col - 1, row, map, memo)
    } else {
        num_worlds(col, row, map, memo)
    };

    memo[index] = count;
    count
}

#[aoc(day7, part2)]
pub fn part2(map: &Map) -> usize {
    num_worlds(map.width / 2, 1, map, &mut vec![0; map.data.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    pub fn input_test() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn part1_test() {
        assert_eq!(part1(&generator(SAMPLE)), 21);
    }

    #[test]
    pub fn part2_test() {
        assert_eq!(part2(&generator(SAMPLE)), 40);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2025/day7.txt");
        const ANSWERS: (usize, usize) = (1717, 231507396180012);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n');
            let output = generator(input);

            assert_eq!(part1(&output), ANSWERS.0);
            assert_eq!(part2(&output), ANSWERS.1);
        }
    }
}
