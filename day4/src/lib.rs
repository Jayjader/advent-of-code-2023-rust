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

pub fn part1(input: &str) -> String {
    format!(
        "{}",
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
            .sum::<usize>()
    )
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
    assert_eq!(part1(input), "13")
}
pub fn part2(input: &str) -> String {
    let line_count = input.trim().split('\n').count();
    format!(
        "{}",
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
            .sum::<usize>()
    )
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
    assert_eq!(part2(input), "30");
}
