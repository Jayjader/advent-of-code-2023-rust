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
                    query
                        .split(", ")
                        .fold(ColorCounts { r: 0, b: 0, g: 0 }, |mut accum, next| {
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
                        })
                })
                .collect(),
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    const MAX_RED: u8 = 12;
    const MAX_GREEN: u8 = 13;
    const MAX_BLUE: u8 = 14;
    let games: Vec<Game> = parse_games(input);
    let allowed_games = games.iter().filter(|game| {
        game.queries
            .iter()
            .all(|q| q.r <= MAX_RED && q.g <= MAX_GREEN && q.b <= MAX_BLUE)
    });
    format!(
        "{}",
        allowed_games.map(|game| game.id as usize).sum::<usize>()
    )
}

#[test]
fn test_part1_on_sample() {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(part1(sample), format!("{}", 1 + 2 + 5));
}

pub fn part2(input: &str) -> String {
    let games: Vec<Game> = parse_games(input);
    format!(
        "{}",
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
            .sum::<usize>()
    )
}
