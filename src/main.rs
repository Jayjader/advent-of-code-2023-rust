fn main() {
    let day8_input = include_str!("../input/day8");
    println!("day 8, part 1: {}", day8::part1(day8_input));
    println!("day 8, part 2: {}", day8::part2(day8_input));
}

mod day1 {

    const DIGITS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    const DIGIT_NAMES: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn part1(input: &str) -> usize {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                find_first_digit_value_with_index(line).unwrap().1 * 10
                    + find_last_digit_value_with_index(line).unwrap().1
            })
            .sum()
    }
    #[test]
    fn part1_on_sample_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 12 + 38 + 15 + 77);
    }

    fn find_first_digit_value_with_index(line: &str) -> Option<(usize, usize)> {
        line.find(|c| DIGITS.contains(&c)).map(|line_index| {
            (
                line_index,
                line.chars().nth(line_index).unwrap().to_digit(10).unwrap() as usize,
            )
        })
    }
    #[test]
    fn test_find_first_digit_value_with_index() {
        let line = "two1nine";
        assert_eq!(Some((3, 1)), find_first_digit_value_with_index(line))
    }
    fn find_last_digit_value_with_index(line: &str) -> Option<(usize, usize)> {
        line.rfind(|c| DIGITS.contains(&c)).map(|line_index| {
            (
                line_index,
                line.chars().nth(line_index).unwrap().to_digit(10).unwrap() as usize,
            )
        })
    }
    #[test]
    fn test_find_last_digit_value_with_index() {
        let line = "two1nine";
        assert_eq!(Some((3, 1)), find_last_digit_value_with_index(line))
    }
    fn find_first_named_digit_value_with_index(line: &str) -> Option<(usize, usize)> {
        DIGIT_NAMES
            .iter()
            .enumerate()
            .filter_map(|(digit_name_index, name)| {
                line.find(name)
                    .map(|line_index| (line_index, digit_name_index + 1))
            })
            .min_by(|(a_index, _a_val), (b_index, _b_val)| a_index.cmp(b_index))
    }
    #[test]
    fn test_find_first_named_digit_with_index() {
        let line = "two1nine";
        assert_eq!(Some((0, 2)), find_first_named_digit_value_with_index(line))
    }
    fn find_last_named_digit_value_with_index(line: &str) -> Option<(usize, usize)> {
        DIGIT_NAMES
            .iter()
            .enumerate()
            .filter_map(|(digit_name_index, name)| {
                line.rfind(name)
                    .map(|line_index| (line_index, digit_name_index + 1))
            })
            .max_by(|(a_index, _a_val), (b_index, _b_val)| a_index.cmp(b_index))
    }
    #[test]
    fn test_find_last_named_digit_with_index() {
        let line = "two1nine";
        assert_eq!(Some((4, 9)), find_last_named_digit_value_with_index(line))
    }

    // single-pass parser?
    enum One {
        O,
        On,
    }
    enum TwoOrThree {
        T,
        Tw,
        Th,
        Thr,
        Thre,
    }
    enum FourOrFive {
        F,
        Fo,
        Fou,
        Fi,
        Fiv,
    }
    enum SixOrSeven {
        S,
        Si,
        Se,
        Sev,
        Seve,
    }
    enum Eight {
        E,
        Ei,
        Eig,
        Eigh,
    }
    enum Nine {
        N,
        Ni,
        Nin,
    }
    enum Parsing {
        Nothing,
        One(One),
        TwoOrThree(TwoOrThree),
        FourOrFive(FourOrFive),
        SixOrSeven(SixOrSeven),
        Eight(Eight),
        Nine(Nine),
        Parsed(u8),
    }

    fn parse_char(c: &char) -> Parsing {
        match c {
            'o' => Parsing::One(One::O),
            't' => Parsing::TwoOrThree(TwoOrThree::T),
            'f' => Parsing::FourOrFive(FourOrFive::F),
            's' => Parsing::SixOrSeven(SixOrSeven::S),
            'e' => Parsing::Eight(Eight::E),
            'n' => Parsing::Nine(Nine::N),
            _ => Parsing::Nothing,
        }
    }
    fn parse_first_digit(line: &str) -> u8 {
        if let Parsing::Parsed(digit) = line.chars().fold(Parsing::Nothing, |accum, next| {
            if let Parsing::Parsed(_) = accum {
                accum
            } else if next.is_numeric() {
                Parsing::Parsed(next.to_digit(10).unwrap() as u8)
            } else {
                match accum {
                    Parsing::Nothing => parse_char(&next),
                    Parsing::One(one) => match (one, next) {
                        (One::O, 'n') => Parsing::One(One::On),
                        (One::On, 'e') => Parsing::Parsed(1),
                        _ => parse_char(&next),
                    },
                    Parsing::TwoOrThree(two_or_three) => match (two_or_three, next) {
                        (TwoOrThree::T, 'w') => Parsing::TwoOrThree(TwoOrThree::Tw),
                        (TwoOrThree::Tw, 'o') => Parsing::Parsed(2),
                        (TwoOrThree::T, 'h') => Parsing::TwoOrThree(TwoOrThree::Th),
                        (TwoOrThree::Th, 'r') => Parsing::TwoOrThree(TwoOrThree::Thr),
                        (TwoOrThree::Thr, 'e') => Parsing::TwoOrThree(TwoOrThree::Thre),
                        (TwoOrThree::Thre, 'e') => Parsing::Parsed(3),
                        _ => parse_char(&next),
                    },
                    Parsing::FourOrFive(four_or_five) => match (four_or_five, next) {
                        (FourOrFive::F, 'o') => Parsing::FourOrFive(FourOrFive::Fo),
                        (FourOrFive::Fo, 'u') => Parsing::FourOrFive(FourOrFive::Fou),
                        (FourOrFive::Fou, 'r') => Parsing::Parsed(4),
                        (FourOrFive::F, 'i') => Parsing::FourOrFive(FourOrFive::Fi),
                        (FourOrFive::Fi, 'v') => Parsing::FourOrFive(FourOrFive::Fiv),
                        (FourOrFive::Fiv, 'e') => Parsing::Parsed(5),
                        _ => parse_char(&next),
                    },
                    Parsing::SixOrSeven(six_or_seven) => match (six_or_seven, next) {
                        (SixOrSeven::S, 'i') => Parsing::SixOrSeven(SixOrSeven::Si),
                        (SixOrSeven::Si, 'x') => Parsing::Parsed(6),
                        (SixOrSeven::S, 'e') => Parsing::SixOrSeven(SixOrSeven::Se),
                        (SixOrSeven::Se, 'v') => Parsing::SixOrSeven(SixOrSeven::Sev),
                        (SixOrSeven::Sev, 'e') => Parsing::SixOrSeven(SixOrSeven::Seve),
                        (SixOrSeven::Seve, 'n') => Parsing::Parsed(7),
                        _ => parse_char(&next),
                    },
                    Parsing::Eight(eight) => match (eight, next) {
                        (Eight::E, 'i') => Parsing::Eight(Eight::Ei),
                        (Eight::Ei, 'g') => Parsing::Eight(Eight::Eig),
                        (Eight::Eig, 'h') => Parsing::Eight(Eight::Eigh),
                        (Eight::Eigh, 't') => Parsing::Parsed(8),
                        _ => parse_char(&next),
                    },
                    Parsing::Nine(nine) => match (nine, next) {
                        (Nine::N, 'i') => Parsing::Nine(Nine::Ni),
                        (Nine::Ni, 'n') => Parsing::Nine(Nine::Nin),
                        (Nine::Nin, 'e') => Parsing::Parsed(9),
                        _ => parse_char(&next),
                    },
                    Parsing::Parsed(_) => {
                        panic!("execution should not have reached this path")
                    }
                }
            }
        }) {
            digit
        } else {
            panic!()
        }
    }
    #[test]
    fn test_parse_first_char() {
        assert_eq!(parse_first_digit("two1nine"), 2);
        assert_eq!(parse_first_digit("oone"), 1);
        assert_eq!(parse_first_digit("ssgfcpxgmtwoeightzmtqlhqfive15"), 2);
    }
    pub fn part2(input: &str) -> usize {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let first = match (
                    find_first_digit_value_with_index(line),
                    find_first_named_digit_value_with_index(line),
                ) {
                    (None, None) => panic!(),
                    (Some((_, digit_value)), None) => digit_value,
                    (None, Some((_, digit_value))) => digit_value,
                    (Some((d_index, d_value)), Some((n_index, n_value))) => {
                        if d_index < n_index {
                            d_value
                        } else {
                            n_value
                        }
                    }
                };

                let last = match (
                    find_last_digit_value_with_index(line),
                    find_last_named_digit_value_with_index(line),
                ) {
                    (None, None) => panic!(),
                    (Some((_, digit_value)), None) => digit_value,
                    (None, Some((_, digit_value))) => digit_value,
                    (Some((d_index, d_value)), Some((n_index, n_value))) => {
                        if d_index > n_index {
                            d_value
                        } else {
                            n_value
                        }
                    }
                };
                first * 10 + last
            })
            .sum()
    }
    #[test]
    fn part2_on_sample_input() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(input), 29 + 83 + 13 + 24 + 42 + 14 + 76);
    }
}

mod day2 {
    #[derive(Debug)]
    struct ColorCounts {
        r: u8,
        g: u8,
        b: u8,
    }
    #[derive(Debug)]
    struct Game {
        id: u8,
        queries: Vec<ColorCounts>,
    }
    fn parse_games(input: &str) -> Vec<Game> {
        input
            .trim_matches('\n')
            .split('\n')
            .map(|line| line.split_once(": ").unwrap())
            .map(|(game_id, queries)| Game {
                id: game_id.split_once(' ').unwrap().1.parse::<u8>().unwrap(),
                queries: queries
                    .split("; ")
                    .map(|query| {
                        query.split(", ").fold(
                            ColorCounts { r: 0, b: 0, g: 0 },
                            |mut accum, next| {
                                let (count, color) = next.split_once(' ').unwrap();
                                let count = count.parse::<u8>().unwrap();
                                match color {
                                    "red" => {
                                        accum.r += count;
                                    }
                                    "blue" => {
                                        accum.b += count;
                                    }
                                    "green" => {
                                        accum.g += count;
                                    }
                                    _ => {}
                                }
                                accum
                            },
                        )
                    })
                    .collect(),
            })
            .collect()
    }

    pub fn part1(input: &str) -> usize {
        const MAX_RED: u8 = 12;
        const MAX_GREEN: u8 = 13;
        const MAX_BLUE: u8 = 14;
        let games: Vec<Game> = parse_games(input);
        let allowed_games = games.iter().filter(|game| {
            game.queries
                .iter()
                .all(|q| q.r <= MAX_RED && q.g <= MAX_GREEN && q.b <= MAX_BLUE)
        });
        allowed_games.map(|game| game.id as usize).sum()
    }

    #[test]
    fn test_part1_on_sample() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(part1(sample), 1 + 2 + 5);
    }

    pub fn part2(input: &str) -> usize {
        let games: Vec<Game> = parse_games(input);
        games
            .iter()
            .map(|game| {
                game.queries
                    .iter()
                    .fold(ColorCounts { r: 0, b: 0, g: 0 }, |mut accum, next| {
                        accum.r = accum.r.max(next.r);
                        accum.g = accum.g.max(next.g);
                        accum.b = accum.b.max(next.b);
                        accum
                    })
            })
            .map(|min_counts| min_counts.r as usize * min_counts.g as usize * min_counts.b as usize)
            .sum()
    }
}

mod day3 {
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

    pub fn part1(input: &str) -> usize {
        let line_length = input.find('\n').unwrap();
        let line_count = input.len() / line_length;
        let parsed = parse_schematic(input);
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
            .sum()
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
        assert_eq!(part1(input), 4361)
    }

    pub fn part2(input: &str) -> usize {
        let line_length = input.find('\n').unwrap();
        let line_count = input.len() / line_length;
        let parsed = parse_schematic(input);
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
            .sum()
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
        assert_eq!(part2(input), 467835);
    }
}

mod day4 {
    use std::collections::BinaryHeap;

    fn parse_numbers(winning: &str) -> impl Iterator<Item = u8> + '_ {
        winning
            .trim()
            .split(' ')
            .filter(|chars| !chars.is_empty())
            .map(|n| n.parse::<u8>().unwrap())
    }
    fn parse_line(line: &str) -> (impl Iterator<Item = u8> + '_, impl Iterator<Item = u8> + '_) {
        let (_, rest) = line.split_once(':').unwrap();
        let (winning, drawn) = rest.split_once('|').unwrap();
        let winning = parse_numbers(winning);
        let drawn = parse_numbers(drawn);
        (winning, drawn)
    }

    pub fn part1(input: &str) -> usize {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (winning, drawn) = parse_line(line);
                let winning = winning.collect::<BinaryHeap<u8>>();
                let scoring = drawn
                    .filter(|number| winning.iter().any(|w| w == number))
                    .count();
                if scoring == 0 {
                    0
                } else {
                    2usize.pow(scoring as u32 - 1)
                }
            })
            .sum()
    }
    #[test]
    fn part1_on_sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(part1(input), 13)
    }
    pub fn part2(input: &str) -> usize {
        let line_count = input.trim().split('\n').count();
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .enumerate()
            .fold(
                std::iter::repeat(1).take(line_count).collect::<Vec<_>>(),
                |mut accum, (card_index, line)| {
                    let card_count = accum[card_index];
                    let (winning, drawn) = parse_line(line);
                    let winning = winning.collect::<BinaryHeap<u8>>();
                    let matches = drawn.filter(|d| winning.iter().any(|w| w == d)).count();
                    for offset in 1..=matches {
                        accum[card_index + offset] += card_count;
                    }
                    accum
                },
            )
            .iter()
            .sum()
    }

    #[test]
    fn part2_on_sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(part2(input), 30);
    }
}

mod day5 {
    use std::collections::HashMap;
    use std::ops::RangeInclusive;

    #[derive(Debug)]
    struct Mapping {
        source_start: usize,
        destination_start: usize,
        count: usize,
    }
    impl Mapping {
        fn source_end_inside(&self) -> usize {
            self.source_start + self.count - 1
        }
        fn source_end_outside(&self) -> usize {
            self.source_end_inside() + 1
        }
        fn destination_end_inside(&self) -> usize {
            self.destination_start + self.count - 1
        }
        fn destination_end_outside(&self) -> usize {
            self.destination_end_inside() + 1
        }
    }
    #[derive(PartialEq, Eq, Debug)]
    enum Value {
        Unmapped(RangeInclusive<usize>),
        Mapped(RangeInclusive<usize>),
    }
    impl Mapping {
        pub fn map_range(&self, range: RangeInclusive<usize>) -> Vec<Value> {
            if *range.end() < self.source_start || *range.start() > self.source_end_inside() {
                // range completely outside mapping
                vec![Value::Unmapped(range)]
            } else if *range.start() >= self.source_start {
                if *range.end() <= self.source_end_inside() {
                    // range completely inside mapping
                    let offset_from_source_start =
                        *range.start() as isize - self.source_start as isize;
                    let offset_from_source_end =
                        *range.end() as isize - self.source_end_inside() as isize;
                    vec![Value::Mapped(
                        ((self.destination_start as isize + offset_from_source_start) as usize)
                            ..=((self.destination_end_inside() as isize + offset_from_source_end)
                                as usize),
                    )]
                } else {
                    // range starts inside but ends outside mapping
                    let offset_from_source_start =
                        *range.start() as isize - self.source_start as isize;
                    vec![
                        Value::Mapped(
                            ((self.destination_start as isize + offset_from_source_start) as usize)
                                ..=self.destination_end_inside(),
                        ),
                        Value::Unmapped(self.source_end_outside()..=*range.end()),
                    ]
                }
            } else if *range.end() <= self.source_end_outside() {
                // range starts outside but ends inside
                let offset_from_source_end =
                    *range.end() as isize - self.source_end_inside() as isize;
                vec![
                    Value::Unmapped(*range.start()..=(self.source_start)),
                    Value::Mapped(
                        self.destination_start
                            ..=(self.destination_end_inside() as isize + offset_from_source_end)
                                as usize,
                    ),
                ]
            } else {
                // range starts and ends outside (and completely overlaps mapping)
                vec![
                    Value::Unmapped(*range.start()..=(self.source_start)),
                    Value::Mapped(self.destination_start..=self.destination_end_inside()),
                    Value::Unmapped(self.source_end_outside()..=*range.end()),
                ]
            }
        }
    }
    #[test]
    fn test_map_range() {
        let mapping = Mapping {
            source_start: 50,
            destination_start: 98,
            count: 2,
        };
        assert_eq!(
            mapping.map_range(55..=(55 + 13)),
            vec![Value::Unmapped(55..=68)]
        );
        assert_eq!(
            mapping.map_range(79..=(79 + 14)),
            vec![Value::Unmapped(79..=93)]
        );

        let mapping = Mapping {
            source_start: 52,
            destination_start: 50,
            count: 48,
        };
        assert_eq!(
            mapping.map_range(55..=(55 + 13)),
            vec![Value::Mapped(53..=66)]
        );
        // 52 + 48 = 100
        // 79 + 14 = 93
        assert_eq!(
            mapping.map_range(79..=(79 + 14)),
            vec![Value::Mapped(77..=91)]
        );

        let mapping = Mapping {
            source_start: 0,
            destination_start: 15,
            count: 37,
        };
        assert_eq!(mapping.map_range(53..=66), vec![Value::Unmapped(53..=66)]);
        assert_eq!(mapping.map_range(79..=93), vec![Value::Unmapped(79..=93)]);
        let mapping = Mapping {
            source_start: 37,
            destination_start: 52,
            count: 2,
        };
        assert_eq!(mapping.map_range(53..=66), vec![Value::Unmapped(53..=66)]);
        assert_eq!(mapping.map_range(79..=93), vec![Value::Unmapped(79..=93)]);
        let mapping = Mapping {
            source_start: 39,
            destination_start: 0,
            count: 15,
        };
        // 53-39=14
        assert_eq!(
            mapping.map_range(53..=66),
            vec![Value::Mapped(14..=14), Value::Unmapped(54..=66)]
        );
    }

    pub fn part1(input: &str) -> usize {
        #[derive(Debug, Default)]
        struct Almanac<'a> {
            seeds: Vec<usize>,
            mappings: HashMap<(&'a str, &'a str), Vec<Mapping>>,
        }
        impl Almanac<'_> {
            pub fn map_value(&self, category: &str, value: usize) -> (&str, usize) {
                let ((_, new_dest), entries) = self
                    .mappings
                    .iter()
                    .find(|((s, _), _)| *s == category)
                    .unwrap();
                (
                    new_dest,
                    entries
                        .iter()
                        .find(|m| m.source_start <= value && value <= m.source_start + m.count)
                        .map_or(value, |m| m.destination_start + (value - m.source_start)),
                )
            }
        }
        fn parse_almanac(input: &str) -> Almanac {
            input.split("\n\n").fold(
                Almanac {
                    seeds: Vec::new(),
                    mappings: HashMap::new(),
                },
                |mut almanac, next| {
                    let (header, rest) = next.split_once(':').unwrap();
                    if almanac.seeds.is_empty() {
                        almanac
                            .seeds
                            .extend(rest.trim().split(' ').map(|s| s.parse::<usize>().unwrap()));
                        almanac
                    } else {
                        let (dest_cat, header_rest) = header.split_once('-').unwrap();
                        // "-to-" is indices 0->2
                        let (_, source_cat) = header_rest.split_at(3);
                        let (source_cat, _) = source_cat.split_once(' ').unwrap();
                        almanac.mappings.entry((dest_cat, source_cat)).or_default();
                        let for_cat_pair =
                            almanac.mappings.get_mut(&(dest_cat, source_cat)).unwrap();
                        for line in rest.trim().split('\n') {
                            let mut split = line.split_whitespace();
                            let destination_start: usize = split.next().unwrap().parse().unwrap();
                            let source_start: usize = split.next().unwrap().parse().unwrap();
                            let count: usize = split.next().unwrap().parse().unwrap();
                            for_cat_pair.push(Mapping {
                                destination_start,
                                source_start,
                                count,
                            });
                        }
                        almanac
                    }
                },
            )
        }
        let almanac = parse_almanac(input);
        almanac
            .seeds
            .iter()
            .map(|seed_number| {
                let mut mapped_value = *seed_number;
                let mut source = "seed";
                while source != "location" {
                    (source, mapped_value) = almanac.map_value(source, mapped_value);
                }
                mapped_value
            })
            .min()
            .unwrap()
    }
    #[test]
    fn part1_on_sample() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part1(input), 35);
    }
    pub fn part2(input: &str) -> usize {
        fn merge_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
            ranges.sort_by(|a, b| a.start().cmp(b.start()));
            let (mut merged, final_considered) =
                ranges
                    .iter()
                    .fold(
                        (Vec::new(), None),
                        |(mut merged, considering), next| match considering {
                            None => (merged, Some(next.clone())),
                            Some(range) => {
                                if next.start() > range.end() {
                                    merged.push(range);
                                    (merged, Some(next.clone()))
                                } else {
                                    (merged, Some(*range.start()..=*range.end().max(next.end())))
                                }
                            }
                        },
                    );
            if let Some(considered) = final_considered {
                merged.push(considered);
            }
            merged
        }
        #[derive(Debug, Default)]
        struct Almanac {
            seeds: Vec<RangeInclusive<usize>>,
            mappings: Vec<Vec<Mapping>>,
        }
        impl Almanac {
            pub fn map_ranges(&self) -> Vec<RangeInclusive<usize>> {
                let mut mapped = Vec::from_iter(
                    self.seeds
                        .iter()
                        .map(|range| Value::Unmapped(range.clone())),
                );
                for category_mapping in self.mappings.iter() {
                    for mapping in category_mapping {
                        for i in 0..mapped.len() {
                            if let Value::Unmapped(range) = &mapped[i] {
                                mapped.splice(i..(i + 1), mapping.map_range(range.clone()));
                            }
                        }
                    }
                    mapped = merge_ranges(
                        mapped
                            .into_iter()
                            .map(|v| match v {
                                Value::Unmapped(range) => range,
                                Value::Mapped(range) => range,
                            })
                            .collect(),
                    )
                    .into_iter()
                    .map(Value::Unmapped)
                    .collect();
                }
                mapped
                    .into_iter()
                    .map(|v| match v {
                        Value::Mapped(range) => range,
                        Value::Unmapped(range) => range,
                    })
                    .collect()
            }
        }
        fn parse_almanac(input: &str) -> Almanac {
            input
                .split("\n\n")
                .fold(Almanac::default(), |mut almanac, next| {
                    let (_, rest) = next.split_once(':').unwrap();
                    if almanac.seeds.is_empty() {
                        let seed_defs: Vec<_> = rest
                            .trim()
                            .split(' ')
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect();
                        almanac.seeds.extend(
                            seed_defs
                                .as_slice()
                                .chunks(2)
                                .map(|chunk| chunk[0]..=(chunk[0] + chunk[1])),
                        );
                        almanac
                    } else {
                        let mut ranges = Vec::new();
                        for line in rest.trim().split('\n') {
                            let mut split = line.split_whitespace();
                            let destination_start: usize = split.next().unwrap().parse().unwrap();
                            let source_start: usize = split.next().unwrap().parse().unwrap();
                            let count: usize = split.next().unwrap().parse().unwrap();
                            ranges.push(Mapping {
                                destination_start,
                                source_start,
                                count,
                            });
                        }
                        almanac.mappings.push(ranges);
                        almanac
                    }
                })
        }
        let almanac = parse_almanac(input);
        almanac
            .map_ranges()
            .iter()
            .map(|range| *range.start())
            .min()
            .unwrap()
    }

    #[test]
    fn part2_on_single() {
        let input = "seeds: 82 1

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part2(input), 46);
    }
    #[test]
    fn part2_on_sample() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part2(input), 46);
    }
}

mod day6 {
    #[derive(Debug)]
    struct Race {
        time: usize,
        distance_record: u64,
    }
    impl Race {
        pub fn ways_to_beat_record(&self) -> u64 {
            let t_min = 0.5
                * (self.time as f32
                    - ((self.time.pow(2) as u64 - 4 * self.distance_record) as f32).sqrt());
            let t_min = if t_min.ceil() == t_min {
                t_min as u64 + 1
            } else {
                t_min.ceil() as u64
            };
            let t_max = 0.5
                * (self.time as f32
                    + ((self.time.pow(2) as u64 - 4 * self.distance_record) as f32).sqrt());
            let t_max = if t_max.floor() == t_max {
                t_max as u64 - 1
            } else {
                t_max.floor() as u64
            };
            t_max - t_min + 1
        }
    }
    fn parse_numbers<T: std::str::FromStr>(line: &str) -> impl Iterator<Item = T> + '_
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<T>().unwrap())
    }
    fn parse_races(input: &str) -> impl Iterator<Item = Race> + '_ {
        let (times, distances) = input.trim().split_once('\n').unwrap();
        let times = parse_numbers(times);
        let distance_records = parse_numbers(distances);
        times
            .zip(distance_records)
            .map(|(time, distance_record)| Race {
                time,
                distance_record,
            })
    }
    pub fn part1(input: &str) -> usize {
        parse_races(input)
            .map(|race| race.ways_to_beat_record())
            .product::<u64>() as usize
    }
    #[test]
    fn part1_on_sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(part1(input), 288);
    }
    fn parse_race(input: &str) -> Race {
        let (times, distances) = input.trim().split_once('\n').unwrap();
        let time = parse_number(times);
        let distance_record = parse_number(distances);
        Race {
            time,
            distance_record,
        }
    }

    fn parse_number<T: std::str::FromStr>(times: &str) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        times
            .split_whitespace()
            .skip(1)
            .flat_map(|s| s.chars())
            .collect::<String>()
            .parse::<T>()
            .unwrap()
    }

    pub fn part2(input: &str) -> u64 {
        parse_race(input).ways_to_beat_record()
    }
    #[test]
    fn part2_on_sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(part2(input), 71503);
    }
}

mod day7 {
    use counter::Counter;

    trait CounterExtDay7 {
        fn top_count(&self) -> usize;
    }
    impl<T: Eq + Ord + Clone + std::hash::Hash> CounterExtDay7 for Counter<T> {
        fn top_count(&self) -> usize {
            self.k_most_common_ordered(1)[0].1
        }
    }

    trait ParseableAsCard {
        fn parse_card(c: char) -> Self;
    }
    fn parse_cards<C: ParseableAsCard>(cards: &str) -> [C; 5] {
        let mut chars = cards.chars();
        [
            C::parse_card(chars.next().unwrap()),
            C::parse_card(chars.next().unwrap()),
            C::parse_card(chars.next().unwrap()),
            C::parse_card(chars.next().unwrap()),
            C::parse_card(chars.next().unwrap()),
        ]
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Hand<C: ParseableAsCard> {
        winning: HandType,
        cards: [C; 5],
    }
    trait ParseableAsHand<C: ParseableAsCard> {
        fn parse_hand(cards: [C; 5]) -> Self;
    }
    fn parse_hands<C: ParseableAsCard>(input: &str) -> Vec<(Hand<C>, u64)>
    where
        Hand<C>: ParseableAsHand<C>,
    {
        input
            .trim()
            .split('\n')
            .map(|line| line.split_once(' ').unwrap())
            .map(|(cards, winnings)| {
                (
                    Hand::parse_hand(parse_cards(cards)),
                    winnings.parse().unwrap(),
                )
            })
            .collect()
    }
    fn solve_part<C: ParseableAsCard + Ord>(input: &str) -> u64
    where
        Hand<C>: ParseableAsHand<C>,
    {
        let mut hands = parse_hands(input);
        hands.sort_by(|(a_hand, _), (b_hand, _)| a_hand.cmp(b_hand));
        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, winnings))| (rank as u64 + 1) * winnings)
            .sum()
    }

    pub fn part1(input: &str) -> u64 {
        #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Copy, Clone)]
        enum Card {
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Jack,
            Queen,
            King,
            Ace,
        }

        impl ParseableAsCard for Card {
            fn parse_card(c: char) -> Card {
                match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("unknown char for card encountered: {}", c),
                }
            }
        }

        impl ParseableAsHand<Card> for Hand<Card> {
            fn parse_hand(cards: [Card; 5]) -> Self {
                let counts = cards.iter().collect::<Counter<_>>();
                Hand {
                    winning: match counts.len() {
                        5 => HandType::HighCard,
                        4 => HandType::OnePair,
                        3 => {
                            if counts.top_count() == 3 {
                                HandType::ThreeOfAKind
                            } else {
                                HandType::TwoPair
                            }
                        }
                        2 => {
                            if counts.top_count() == 3 {
                                HandType::FullHouse
                            } else {
                                HandType::FourOfAKind
                            }
                        }
                        1 => HandType::FiveOfAKind,
                        _ => panic!("cards in hand not between 1 and 5"),
                    },
                    cards,
                }
            }
        }

        solve_part::<Card>(input)
    }

    pub fn part2(input: &str) -> u64 {
        #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Copy, Clone)]
        enum Card {
            Joker,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Queen,
            King,
            Ace,
        }

        impl ParseableAsCard for Card {
            fn parse_card(c: char) -> Card {
                match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    'J' => Card::Joker,
                    _ => panic!("unknown char for card encountered: {}", c),
                }
            }
        }
        impl ParseableAsHand<Card> for Hand<Card> {
            fn parse_hand(cards: [Card; 5]) -> Self {
                let counts = cards.iter().collect::<Counter<_>>();
                Hand {
                    winning: match (counts.len(), cards.contains(&Card::Joker)) {
                        (1, _) => HandType::FiveOfAKind,
                        (2, true) => HandType::FiveOfAKind,
                        (2, false) => {
                            if counts.top_count() == 3 {
                                HandType::FullHouse
                            } else {
                                HandType::FourOfAKind
                            }
                        }
                        (3, true) => {
                            let ordered = counts.k_most_common_ordered(3);
                            if ordered[0].1 == 3 {
                                HandType::FourOfAKind
                            } else if *ordered.last().unwrap().0 == Card::Joker {
                                HandType::FullHouse
                            } else {
                                HandType::FourOfAKind
                            }
                        }
                        (3, false) => {
                            if counts.top_count() == 3 {
                                HandType::ThreeOfAKind
                            } else {
                                HandType::TwoPair
                            }
                        }
                        (4, true) => HandType::ThreeOfAKind,
                        (4, false) => HandType::OnePair,
                        (5, true) => HandType::OnePair,
                        (5, false) => HandType::HighCard,
                        _ => panic!("cards in hand not between 1 and 5"),
                    },
                    cards,
                }
            }
        }

        solve_part::<Card>(input)
    }

    #[test]
    fn part1_on_sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(part1(input), 6440);
    }
    #[test]
    fn part1_on_extra_sample() {
        let input = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
";
        assert_eq!(part1(input), 1343);
    }

    #[test]
    fn part2_on_sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(part2(input), 5905);
    }
}

mod day8 {
    use std::collections::BTreeMap;

    fn parse_mappings(lines: &str) -> BTreeMap<&str, (&str, &str)> {
        lines
            .trim_start()
            .split_terminator('\n')
            .map(|line| line.split_once(" = ").unwrap())
            .map(|(current_node, left_right)| {
                (
                    current_node,
                    left_right
                        .trim_matches(&['(', ')'][..])
                        .split_once(", ")
                        .unwrap(),
                )
            })
            .collect()
    }
    enum Instruction {
        Left,
        Right,
    }
    impl TryFrom<char> for Instruction {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'L' => Ok(Instruction::Left),
                'R' => Ok(Instruction::Right),
                _ => Err(()),
            }
        }
    }
    pub fn part1(input: &str) -> usize {
        use itertools::{
            FoldWhile::{Continue, Done},
            Itertools,
        };
        let (pattern, mappings) = input.split_once("\n\n").unwrap();
        let mappings = parse_mappings(mappings);
        let (_, count) = pattern
            .chars()
            .map(|c| c.try_into().unwrap())
            .cycle()
            .fold_while(("AAA", 0), |(accum, count), instruction| {
                match *mappings
                    .get(accum)
                    .map(|(left, right)| match instruction {
                        Instruction::Right => right,
                        Instruction::Left => left,
                    })
                    .unwrap()
                {
                    "ZZZ" => Done(("ZZZ", count)),
                    next_node => Continue((next_node, count + 1)),
                }
            })
            .into_inner();
        count
    }

    pub fn part2(input: &str) -> usize {
        let (pattern, mappings) = input.split_once("\n\n").unwrap();
        let mappings = parse_mappings(mappings);
        #[derive(Copy, Clone)]
        enum NodeState {
            EndsWithZAfter(usize),
            // nodes traversed until now
            SearchingForEndState(usize),
        }
        let mut current_nodes = mappings
            .iter()
            .filter_map(|(node, (_, _))| {
                node.ends_with('A')
                    .then_some((*node, NodeState::SearchingForEndState(0)))
            })
            .collect::<Vec<_>>();
        for instruction in pattern.chars().map(|c| c.try_into().unwrap()).cycle() {
            if current_nodes
                .iter()
                .all(|(_, state)| matches!(state, NodeState::EndsWithZAfter(_)))
            {
                break;
            }
            for (node_def, state) in current_nodes.iter_mut() {
                if let NodeState::SearchingForEndState(traversed) = state {
                    let (left, right) = mappings.get(node_def).unwrap();
                    *node_def = match instruction {
                        Instruction::Right => right,
                        Instruction::Left => left,
                    };
                    *traversed += 1;
                    if node_def.ends_with('Z') {
                        *state = NodeState::EndsWithZAfter(*traversed)
                    }
                }
            }
        }
        current_nodes
            .iter()
            .filter_map(|(_, state)| {
                if let NodeState::EndsWithZAfter(traversed) = state {
                    Some(traversed)
                } else {
                    None
                }
            })
            .fold(1, |accum, next| lcm(accum, *next))
    }
    /// thank you https://rustp.org/number-theory/lcm/
    fn gcd(mut a: usize, mut b: usize) -> usize {
        if a == b {
            return a;
        }
        if b > a {
            std::mem::swap(&mut a, &mut b);
        }
        while b > 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }

    /// thank you https://rustp.org/number-theory/lcm/
    fn lcm(a: usize, b: usize) -> usize {
        a * (b / gcd(a, b))
    }

    #[test]
    fn part2_on_sample() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(part2(input), 6);
    }
}
