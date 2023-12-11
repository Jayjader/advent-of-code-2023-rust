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
                let offset_from_source_start = *range.start() as isize - self.source_start as isize;
                let offset_from_source_end =
                    *range.end() as isize - self.source_end_inside() as isize;
                vec![Value::Mapped(
                    ((self.destination_start as isize + offset_from_source_start) as usize)
                        ..=((self.destination_end_inside() as isize + offset_from_source_end)
                            as usize),
                )]
            } else {
                // range starts inside but ends outside mapping
                let offset_from_source_start = *range.start() as isize - self.source_start as isize;
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
            let offset_from_source_end = *range.end() as isize - self.source_end_inside() as isize;
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

pub fn part1(input: &str) -> String {
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
                    let for_cat_pair = almanac.mappings.get_mut(&(dest_cat, source_cat)).unwrap();
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
    format!(
        "{}",
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
    )
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
    assert_eq!(part1(input), "35");
}
pub fn part2(input: &str) -> String {
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
    format!(
        "{}",
        almanac
            .map_ranges()
            .iter()
            .map(|range| *range.start())
            .min()
            .unwrap()
    )
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
    assert_eq!(part2(input), "46");
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
    assert_eq!(part2(input), "46");
}
