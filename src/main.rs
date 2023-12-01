fn main() {
    let day1_input = include_str!("../input/day1");
    println!("day 1, part 1: {}", day1::part1(day1_input));
    println!("day 1, part 2: {}", day1::part2(day1_input));
}

mod day1 {
    use std::ops::Index;

    const DIGITS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    const DIGIT_NAMES: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    pub fn part1(input: &str) -> usize {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                format!(
                    "{}{}",
                    line.chars()
                        .nth(line.find(|c| DIGITS.contains(&c)).unwrap())
                        .unwrap(),
                    line.chars()
                        .nth(line.rfind(|c| DIGITS.contains(&c)).unwrap())
                        .unwrap(),
                )
                .parse::<usize>()
                .unwrap()
            })
            .sum()
    }
    #[test]
    fn part1_on_sample_input() {
        use crate::day1;
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(day1::part1(input), 12 + 38 + 15 + 77);
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
                format!("{}{}", first, last).parse::<usize>().unwrap()
            })
            .sum()
    }
    #[test]
    fn part2_on_sample_input() {
        use crate::day1;
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(day1::part2(input), 29 + 83 + 13 + 24 + 42 + 14 + 76);
    }
}
