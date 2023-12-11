#[derive(Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}
impl Position {
    pub fn from_usize(x: usize, y: usize) -> Position {
        Position {
            x: x as u8,
            y: y as u8,
        }
    }
}
#[derive(Clone, Copy)]
struct Number {
    val: u32,
    start: Position,
    end: Position,
}
impl Number {
    pub fn is_valid_part_number(
        self: &Number,
        line_length: usize,
        line_count: usize,
        symbol_pos: &Position,
    ) -> bool {
        let min_num_x = self.start.x.saturating_sub(1);
        let min_num_y = self.start.y.saturating_sub(1);
        let max_num_x = (line_length as u8).min(self.end.x + 1);
        let max_num_y = (line_count as u8).min(self.end.y + 1);
        (min_num_x <= symbol_pos.x)
            && (symbol_pos.x <= max_num_x)
            && (min_num_y <= symbol_pos.y)
            && (symbol_pos.y <= max_num_y)
    }
}
struct Symbol {
    val: char,
    pos: Position,
}
struct Parsing {
    current_number: Option<Number>,
    found_numbers: Vec<Number>,
    found_symbols: Vec<Symbol>,
}

fn parse_schematic(input: &str) -> Parsing {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(
            Parsing {
                current_number: None,
                found_numbers: vec![],
                found_symbols: vec![],
            },
            |accum, (y, line)| {
                let mut accum_with_line =
                    line.chars().enumerate().fold(accum, |mut accum, (x, c)| {
                        if c == '.' {
                            if let Some(number) = accum.current_number {
                                accum.found_numbers.push(number);
                            }
                            Parsing {
                                current_number: None,
                                ..accum
                            }
                        } else if c.is_numeric() {
                            let pos = Position::from_usize(x, y);
                            Parsing {
                                current_number: Some(Number {
                                    val: c.to_digit(10).unwrap()
                                        + accum.current_number.map_or(0, |n| n.val * 10),
                                    start: accum.current_number.map_or(pos, |n| n.start),
                                    end: pos,
                                }),
                                ..accum
                            }
                        } else if !c.is_alphanumeric() {
                            accum.found_symbols.push(Symbol {
                                val: c,
                                pos: Position::from_usize(x, y),
                            });
                            if let Some(number) = accum.current_number {
                                accum.found_numbers.push(number);
                            }
                            Parsing {
                                current_number: None,
                                ..accum
                            }
                        } else {
                            panic!()
                        }
                    });
                // terminate current part number at line end
                if let Some(number) = accum_with_line.current_number {
                    accum_with_line.found_numbers.push(number);
                    accum_with_line.current_number = None;
                }
                accum_with_line
            },
        )
}

pub fn part1(input: &str) -> String {
    let line_length = input.find('\n').unwrap();
    let line_count = input.len() / line_length;
    let parsed = parse_schematic(input);
    format!(
        "{}",
        parsed
            .found_numbers
            .iter()
            .filter(|number| {
                parsed
                    .found_symbols
                    .iter()
                    .any(|symbol| number.is_valid_part_number(line_length, line_count, &symbol.pos))
            })
            .map(|n| n.val as usize)
            .sum::<usize>()
    )
}
#[test]
fn part1_on_sample_input() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    assert_eq!(part1(input), "4361")
}

pub fn part2(input: &str) -> String {
    let line_length = input.find('\n').unwrap();
    let line_count = input.len() / line_length;
    let parsed = parse_schematic(input);
    format!(
        "{}",
        parsed
            .found_symbols
            .iter()
            .filter(|s| s.val == '*')
            .map(|symbol| {
                parsed
                    .found_numbers
                    .iter()
                    .filter(|number| {
                        number.is_valid_part_number(line_length, line_count, &symbol.pos)
                    })
                    .map(|number| (number.val as usize))
                    .collect::<Vec<_>>()
            })
            .filter(|valid_numbers| valid_numbers.len() == 2)
            .map(|part_numbers| part_numbers.iter().product::<usize>())
            .sum::<usize>()
    )
}

#[test]
fn part2_on_sample_input() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

";
    assert_eq!(part2(input), "467835");
}
