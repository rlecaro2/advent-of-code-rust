use itertools::Itertools;
use regex::Regex;
use std::{cmp::Ordering, collections::BinaryHeap};

advent_of_code::solution!(7);

fn get_hand_type(cards: &str) -> HandType {
    let mut counts = Vec::new();
    for (_, group) in &cards.chars().sorted().group_by(|c| c.clone()) {
        counts.push(group.collect::<Vec<char>>().len());
    }

    let four_or_five = counts.iter().find(|&&c| c == 5 || c == 4);
    let pairs = counts.iter().filter(|&&c| c == 2).count();
    let threes = counts.iter().filter(|&&c| c == 3).count();

    match four_or_five {
        Some(4) => return HandType::FourOfAKind,
        Some(5) => return HandType::FiveOfAKind,
        _ => (),
    }

    match (pairs, threes) {
        (1, 1) => return HandType::FullHouse,
        (0, 1) => return HandType::ThreeOfAKind,
        (2, 0) => return HandType::TwoPair,
        (1, 0) => return HandType::OnePair,
        _ => return HandType::HighCard,
    }
}

fn get_card_value(card: char) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

// #[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: u64,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            let self_chars = self.cards.chars();
            let other_chars = other.cards.chars();

            for (s, o) in self_chars.zip(other_chars) {
                match get_card_value(s).cmp(&get_card_value(o)) {
                    Ordering::Equal => continue,
                    other_ordering => return other_ordering,
                }
            }
        }

        self.hand_type.cmp(&other.hand_type)
    }
}

fn create_hand(cards: &str, bid: u64) -> Hand {
    Hand {
        cards: cards.to_string(),
        hand_type: get_hand_type(cards),
        bid,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let hand_re = Regex::new(r"(?<cards>\w{5}) (?<bid>\d+)").unwrap();

    let mut heap: BinaryHeap<Hand> = BinaryHeap::new();

    for line in input.lines() {
        let caps = hand_re.captures(line).unwrap();
        let cards = &caps["cards"];
        let bid = caps["bid"].parse::<u64>().unwrap();

        let hand = create_hand(cards, bid);
        heap.push(hand);
    }

    let mut total = 0;
    let mut multiplier = heap.iter().count() as u64;
    while let Some(hand) = heap.pop() {
        total += hand.bid * multiplier;
        multiplier -= 1;
    }

    return Some(total);
}

pub fn part_two(_: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_hand_type() {
        assert_eq!(get_hand_type("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("ABAAA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("JAJAA"), HandType::FullHouse);
        assert_eq!(get_hand_type("62AAA"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("6262A"), HandType::TwoPair);
        assert_eq!(get_hand_type("62334"), HandType::OnePair);
        assert_eq!(get_hand_type("62345"), HandType::HighCard);
    }

    #[test]
    fn test_same_hand_ordering() {
        assert!(
            Hand {
                cards: "T55J5".to_string(),
                bid: 0,
                hand_type: HandType::ThreeOfAKind,
            } < Hand {
                cards: "QQQJA".to_string(),
                bid: 0,
                hand_type: HandType::ThreeOfAKind,
            },
            "ordering isn't working"
        );
    }
}
