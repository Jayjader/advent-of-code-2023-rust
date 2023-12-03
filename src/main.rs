fn main() {
    let day3_input = include_str!("../input/day3");
    println!("day 3, part 1: {}", day3::part1(day3_input));
    println!("day 3, part 2: {}", day3::part2(day3_input));
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
    struct Number {
        val: u32,
        start: Position,
        end: Position,
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
                            match accum.current_number {
                                None => match c {
                                    '.' => accum,
                                    d if d.is_numeric() => {
                                        let pos = Position::from_usize(x, y);
                                        accum.current_number = Some(Number {
                                            val: d.to_digit(10).unwrap(),
                                            start: pos,
                                            end: pos,
                                        });
                                        accum
                                    }
                                    d if !d.is_alphanumeric() => {
                                        accum.found_symbols.push(Symbol {
                                            val: c,
                                            pos: Position::from_usize(x, y),
                                        });
                                        accum
                                    }
                                    _ => panic!(),
                                },
                                Some(number) => match c {
                                    '.' => {
                                        accum.found_numbers.push(number);
                                        accum.current_number = None;
                                        accum
                                    }
                                    d if d.is_numeric() => {
                                        accum.current_number = Some(Number {
                                            val: number.val * 10 + d.to_digit(10).unwrap(),
                                            end: Position::from_usize(x, y),
                                            ..number
                                        });
                                        accum
                                    }
                                    d if !d.is_alphanumeric() => {
                                        accum.found_numbers.push(number);
                                        accum.current_number = None;
                                        accum.found_symbols.push(Symbol {
                                            val: c,
                                            pos: Position::from_usize(x, y),
                                        });
                                        accum
                                    }
                                    _ => panic!(),
                                },
                            }
                        });
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
                parsed.found_symbols.iter().any(|symbol| {
                    let min_num_x = number.start.x.saturating_sub(1);
                    let min_num_y = number.start.y.saturating_sub(1);
                    let max_num_x = (line_length as u8).min(number.end.x + 1);
                    let max_num_y = (line_count as u8).min(number.end.y + 1);
                    (min_num_x <= symbol.pos.x)
                        && (symbol.pos.x <= max_num_x)
                        && (min_num_y <= symbol.pos.y)
                        && (symbol.pos.y <= max_num_y)
                })
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
            .filter_map(|symbol| {
                let valid_numbers = parsed
                    .found_numbers
                    .iter()
                    .filter_map(|number| {
                        let min_num_x = number.start.x.saturating_sub(1);
                        let min_num_y = number.start.y.saturating_sub(1);
                        let max_num_x = (line_length as u8).min(number.end.x + 1);
                        let max_num_y = (line_count as u8).min(number.end.y + 1);
                        if (min_num_x <= symbol.pos.x)
                            && (symbol.pos.x <= max_num_x)
                            && (min_num_y <= symbol.pos.y)
                            && (symbol.pos.y <= max_num_y)
                        {
                            Some(number.val as usize)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if valid_numbers.len() == 2 {
                    Some(valid_numbers.iter().product::<usize>())
                } else {
                    None
                }
            })
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
