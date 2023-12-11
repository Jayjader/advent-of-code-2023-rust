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

pub fn part1(input: &str) -> String {
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

    format!("{}", solve_part::<Card>(input))
}

pub fn part2(input: &str) -> String {
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

    format!("{}", solve_part::<Card>(input))
}

#[test]
fn part1_on_sample() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(part1(input), "6440");
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
    assert_eq!(part1(input), "1343");
}

#[test]
fn part2_on_sample() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(part2(input), "5905");
}
