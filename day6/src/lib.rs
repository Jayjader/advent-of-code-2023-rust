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
pub fn part1(input: &str) -> String {
    format!(
        "{}",
        parse_races(input)
            .map(|race| race.ways_to_beat_record())
            .product::<u64>()
    )
}
#[test]
fn part1_on_sample() {
    let input = "Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part1(input), "288");
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

pub fn part2(input: &str) -> String {
    format!("{}", parse_race(input).ways_to_beat_record())
}
#[test]
fn part2_on_sample() {
    let input = "Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part2(input), "71503");
}
