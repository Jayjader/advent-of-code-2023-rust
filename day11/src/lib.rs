use std::collections::{BTreeMap, BTreeSet};

type Position = (usize, usize);
#[derive(Debug)]
enum GridCell {
    EmptySpace,
    Galaxy,
}
impl TryFrom<char> for GridCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(GridCell::EmptySpace),
            '#' => Ok(GridCell::Galaxy),
            _ => Err(()),
        }
    }
}
struct Galaxies(BTreeSet<Position>);

impl Galaxies {
    fn parse(input: &str, expansion_factor: usize) -> Galaxies {
        let lines = input.trim().split('\n');
        let empty_rows: Vec<_> = lines
            .clone()
            .enumerate()
            .filter(|(_, line)| line.chars().all(|c| c.eq(&'.')))
            .map(|(y, _)| y)
            .collect();
        let empty_columns: Vec<_> = lines
            .clone()
            .fold(BTreeMap::new(), |mut accum, line| {
                if accum.is_empty() {
                    for (x, c) in line.chars().enumerate() {
                        accum.insert(
                            x,
                            match c {
                                '.' => true,
                                '#' => false,
                                _ => panic!(),
                            },
                        );
                    }
                } else {
                    for (x, c) in line.chars().enumerate() {
                        if *accum.get(&x).unwrap() {
                            accum.insert(
                                x,
                                match c {
                                    '.' => true,
                                    '#' => false,
                                    _ => panic!(),
                                },
                            );
                        }
                    }
                }

                accum
            })
            .iter()
            .filter(|(_, is_empty)| **is_empty)
            .map(|(&x, _)| x)
            .collect();
        let mut galaxies = BTreeSet::new();
        let mut y_offset = 0;
        for (y, line) in lines.enumerate() {
            if empty_rows.contains(&y) {
                y_offset += (expansion_factor - 1);
            } else {
                let mut x_offset = 0;
                for (x, c) in line.chars().enumerate() {
                    if empty_columns.contains(&x) {
                        x_offset += (expansion_factor - 1);
                    } else {
                        let cell = c.try_into().unwrap();
                        {
                            let cell_pos = (x + x_offset, y + y_offset);
                            if let GridCell::Galaxy = cell {
                                galaxies.insert(cell_pos);
                            }
                        }
                    }
                }
            }
        }
        Galaxies(galaxies)
    }
    pub fn sum_shortest_pairwise_distances(&self) -> usize {
        use itertools::Itertools;
        self.0
            .iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .map(|((from_x, from_y), (to_x, to_y))| from_x.abs_diff(*to_x) + from_y.abs_diff(*to_y))
            .sum()
    }
}
pub fn part1(input: &str) -> String {
    let galaxies = Galaxies::parse(input, 2);
    format!("{}", galaxies.sum_shortest_pairwise_distances())
}

#[test]
fn part1_on_sample() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(part1(input), "374");
}
#[test]
fn bigger_expansions_on_sample() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(
        1030,
        Galaxies::parse(input, 10).sum_shortest_pairwise_distances()
    );
    assert_eq!(
        8410,
        Galaxies::parse(input, 100).sum_shortest_pairwise_distances()
    );
}
pub fn part2(input: &str) -> String {
    format!(
        "{}",
        Galaxies::parse(input, 1_000_000).sum_shortest_pairwise_distances()
    )
}
