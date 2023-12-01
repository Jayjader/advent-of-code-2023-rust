fn main() {
    let day1_input = include_str!("../input/day1");
    println!("day 1, part 1: {}", day1::part1(day1_input));
    println!("day 1, part 2: {}", day1::part2(day1_input));
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
                    Parsing::TwoOrThree(twoOrThree) => match (twoOrThree, next) {
                        (TwoOrThree::T, 'w') => Parsing::TwoOrThree(TwoOrThree::Tw),
                        (TwoOrThree::Tw, 'o') => Parsing::Parsed(2),
                        (TwoOrThree::T, 'h') => Parsing::TwoOrThree(TwoOrThree::Th),
                        (TwoOrThree::Th, 'r') => Parsing::TwoOrThree(TwoOrThree::Thr),
                        (TwoOrThree::Thr, 'e') => Parsing::TwoOrThree(TwoOrThree::Thre),
                        (TwoOrThree::Thre, 'e') => Parsing::Parsed(3),
                        _ => parse_char(&next),
                    },
                    Parsing::FourOrFive(fourOrFive) => match (fourOrFive, next) {
                        (FourOrFive::F, 'o') => Parsing::FourOrFive(FourOrFive::Fo),
                        (FourOrFive::Fo, 'u') => Parsing::FourOrFive(FourOrFive::Fou),
                        (FourOrFive::Fou, 'r') => Parsing::Parsed(4),
                        (FourOrFive::F, 'i') => Parsing::FourOrFive(FourOrFive::Fi),
                        (FourOrFive::Fi, 'v') => Parsing::FourOrFive(FourOrFive::Fiv),
                        (FourOrFive::Fiv, 'e') => Parsing::Parsed(5),
                        _ => parse_char(&next),
                    },
                    Parsing::SixOrSeven(sixOrSeven) => match (sixOrSeven, next) {
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
