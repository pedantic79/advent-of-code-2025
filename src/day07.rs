use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use pathfinding::prelude::{bfs_reach, count_paths};

// We process every other row because those are the rows with splitters
const ROW_STEP: usize = 2;

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
    // BFS to find all reachable '^' from the starting point
    bfs_reach((ROW_STEP, map.width / 2), |&(row, col)| {
        let mut v = ArrayVec::<_, 2>::new();

        for new_col in [col - 1, col + 1] {
            // search downwards for the next '^' in this column
            for r in (row + ROW_STEP..=map.height).step_by(ROW_STEP) {
                if map
                    .data
                    .get((map.width + 1) * r + new_col)
                    .copied()
                    .unwrap_or(b' ')
                    == b'^'
                {
                    v.push((r, new_col));
                    break;
                }
            }
        }

        v
    })
    .count()
}

#[aoc(day7, part2)]
pub fn part2(map: &Map) -> usize {
    count_paths(
        (ROW_STEP, map.width / 2),
        |&(row, col)| {
            let mut v = ArrayVec::<_, 2>::new();
            if row + ROW_STEP > map.height {
            } else if map.data[(map.width + 1) * row + col] == b'^' {
                v.push((row + ROW_STEP, col - 1));
                v.push((row + ROW_STEP, col + 1));
            } else {
                v.push((row + ROW_STEP, col));
            }

            v
        },
        |&(row, _)| row == map.height,
    )
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
