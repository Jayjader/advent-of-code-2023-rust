use std::collections::VecDeque;

trait VecAndVecDequeExtForDay9 {
    fn compute_diffs(&self) -> VecDeque<i64>;
}
#[macro_export]
macro_rules! extend_for_day9 {
        ($($t:ty),+) => {
            $(
            impl VecAndVecDequeExtForDay9 for $t {
                fn compute_diffs(&self) -> VecDeque<i64> {
                    let diff_length = self.len() - 1;
                    self.iter().enumerate().take(diff_length).fold(
                        VecDeque::with_capacity(diff_length),
                        |mut accum, (index, next)| {
                            accum.push_back(self[index + 1] - next);
                            accum
                        },
                    )
                }
            })+
        };
    }
extend_for_day9!(Vec<i64>, VecDeque<i64>);

#[macro_export]
macro_rules! compute_diffs {
        ($($record:ident)?) => {
            $(loop {
                $record.push($record.last().unwrap().compute_diffs());
                if $record.last().unwrap().iter().all(|&d| d == 0) {
                    break;
                }
            })?
        };
    }

#[macro_export]
macro_rules! complete_numbers {
        ($record:ident, $accessor:ident, $mutator:ident, $completer:ident) => {
            {
                for index_in_record in (0..($record.len() - 1)).rev() {
                    // get the immediately higher level's last value
                    let &higher_levels_last = $accessor(&$record[index_in_record + 1]).unwrap();
                    // derive missing value for current diff level from the immediately higher level's boundary value
                    let current_diff_level = &mut $record[index_in_record];
                    let missing_value = $completer(*$accessor(current_diff_level).unwrap() , higher_levels_last);
                    // extend current diff level's record with missing value
                    $mutator(current_diff_level, missing_value);
                }
                $accessor(&$record.first().unwrap()).unwrap()
            }
        };
    }

fn parse_numbers(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.trim().split('\n').map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    })
}
pub fn part1(input: &str) -> String {
    use std::ops::Add;
    format!(
        "{}",
        parse_numbers(input)
            .map(|numbers| {
                let mut diff_record = vec![numbers.compute_diffs()];
                compute_diffs!(diff_record);
                let accessor = VecDeque::back;
                let mutator = VecDeque::push_back;
                let completer: fn(i64, i64) -> <i64 as Add<i64>>::Output = i64::add;
                let missing_diff_val = complete_numbers!(diff_record, accessor, mutator, completer);
                numbers.last().unwrap() + missing_diff_val
            })
            .sum::<i64>()
    )
}

#[test]
fn part1_on_first_sample_line() {
    let input = "0 3 6 9 12 15\n";
    assert_eq!(part1(input), "18");
}
#[test]
fn part1_on_second_sample_line() {
    let input = "1 3 6 10 15 21\n";
    assert_eq!(part1(input), "28");
}
#[test]
fn part1_on_sample() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part1(input), "114");
}

pub fn part2(input: &str) -> String {
    use std::ops::Sub;
    format!(
        "{}",
        parse_numbers(input)
            .map(|numbers| {
                let mut diff_record = vec![numbers.compute_diffs()];
                compute_diffs!(diff_record);
                let accessor = VecDeque::front;
                let mutator = VecDeque::push_front;
                let completer: fn(i64, i64) -> <i64 as Sub<i64>>::Output = i64::sub;
                let missing_diff_val = complete_numbers!(diff_record, accessor, mutator, completer);
                numbers.first().unwrap() - missing_diff_val
            })
            .sum::<i64>()
    )
}

#[test]
fn part2_on_first_sample_line() {
    let input = "0 3 6 9 12 15\n";
    assert_eq!(part2(input), "-3");
}
#[test]
fn part2_on_second_sample_line() {
    let input = "1 3 6 10 15 21\n";
    assert_eq!(part2(input), "0");
}
#[test]
fn part2_on_third_sample_line() {
    let input = "10 13 16 21 30 45\n";
    assert_eq!(part2(input), "5");
}
#[test]
fn part2_on_sample() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part2(input), "2");
}
