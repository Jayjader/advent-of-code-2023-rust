#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Start,
}
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct PipeOpenings {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}
impl Pipe {
    fn get_openings(&self) -> PipeOpenings {
        match self {
            Pipe::Vertical => PipeOpenings {
                north: true,
                south: true,
                east: false,
                west: false,
            },
            Pipe::Horizontal => PipeOpenings {
                north: false,
                south: false,
                east: true,
                west: true,
            },
            Pipe::NorthEastBend => PipeOpenings {
                north: true,
                south: false,
                east: true,
                west: false,
            },
            Pipe::NorthWestBend => PipeOpenings {
                north: true,
                south: false,
                east: false,
                west: true,
            },
            Pipe::SouthWestBend => PipeOpenings {
                north: false,
                south: true,
                east: false,
                west: true,
            },
            Pipe::SouthEastBend => PipeOpenings {
                north: false,
                south: true,
                east: true,
                west: false,
            },
            Pipe::Start => PipeOpenings {
                north: true,
                south: true,
                east: true,
                west: true,
            },
        }
    }
}
impl PipeOpenings {
    fn can_connect(&self, compass_direction: &Direction, to_: &PipeOpenings) -> bool {
        matches!(
            (self, compass_direction, to_),
            (
                PipeOpenings { north: true, .. },
                Direction::North,
                PipeOpenings { south: true, .. }
            ) | (
                PipeOpenings { south: true, .. },
                Direction::South,
                PipeOpenings { north: true, .. }
            ) | (
                PipeOpenings { east: true, .. },
                Direction::East,
                PipeOpenings { west: true, .. }
            ) | (
                PipeOpenings { west: true, .. },
                Direction::West,
                PipeOpenings { east: true, .. }
            )
        )
    }
}
#[derive(Debug, Eq, PartialEq)]
enum GridCell {
    Ground,
    Pipe(PipeOpenings),
}

impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEastBend),
            'J' => Ok(Pipe::NorthWestBend),
            '7' => Ok(Pipe::SouthWestBend),
            'F' => Ok(Pipe::SouthEastBend),
            'S' => Ok(Pipe::Start),
            _ => Err(()),
        }
    }
}
impl TryFrom<char> for GridCell {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(GridCell::Ground),
            _ => match <char as TryInto<Pipe>>::try_into(value) {
                Ok(pipe) => Ok(GridCell::Pipe(pipe.get_openings())),
                Err(_) => Err(()),
            },
        }
    }
}
type Position = (usize, usize);
fn get_neighbors(
    p: &Position,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (Position, Direction)> + '_ {
    let &(p_x, p_y) = p;
    [
        ((p_x.saturating_sub(1), p_y), Direction::West),
        ((p_x, (p_y).saturating_sub(1)), Direction::North),
        (((p_x + 1).min(width - 1), p_y), Direction::East),
        ((p_x, (p_y + 1).min(height - 1)), Direction::South),
    ]
    .into_iter()
    .filter(|&((x, y), _)| (x, y) != *p)
}
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<GridCell>>,
    start: Position,
}
impl Grid {
    fn parse(input: &str) -> Grid {
        let mut start = Position::default();
        let cells = input
            .trim()
            .split('\n')
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .flat_map(<char as TryInto<GridCell>>::try_into)
                    .enumerate()
                    .map(|(x, cell)| {
                        if let GridCell::Pipe(openings) = cell {
                            if openings == Pipe::Start.get_openings() {
                                start = (x, y);
                            }
                        }
                        cell
                    })
                    .collect::<Vec<GridCell>>()
            })
            .collect::<Vec<_>>();
        let (width, height) = (cells[0].len(), cells.len());
        Grid {
            width,
            height,
            cells,
            start,
        }
    }
    fn find_loop(&self, start: &Position) -> Vec<(Position, PipeOpenings)> {
        let mut loop_segment: Vec<(Position, PipeOpenings)> =
            vec![(*start, Pipe::Start.get_openings())];
        let mut found_head = false;
        while !found_head {
            let &((tail_x, tail_y), tail_openings) = loop_segment.last().unwrap();
            for ((x, y), direction_from_tail) in
                get_neighbors(&(tail_x, tail_y), self.width, self.height)
            {
                if let GridCell::Pipe(neighbor_openings) = &self.cells[y][x] {
                    if tail_openings.can_connect(&direction_from_tail, neighbor_openings) {
                        // if we can connect, then this is one of:
                        let &(start, _) = loop_segment.first().unwrap();
                        let (previous, _) = loop_segment[loop_segment.len().saturating_sub(2)];
                        if (x, y) == previous {
                            // 1. the previous cell in the loop segment
                            continue;
                        }
                        if (x, y) == start {
                            // 2. the start of the loop segment
                            found_head = true;
                        } else {
                            // 3. simply the next cell in the loop segment
                            loop_segment.push(((x, y), *neighbor_openings));
                        }
                        // we've either extended or completed the loop segment,
                        // so we can skip considering the remaining neighbor cells
                        break;
                    }
                }
            }
        }
        loop_segment
    }
}
pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    format!("{}", grid.find_loop(&grid.start).len() / 2)
}
#[test]
fn part1_on_first_sample() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
    assert_eq!(part1(input), "4");
}
#[test]
fn part1_on_second_sample() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
    assert_eq!(part1(input), "8");
}
