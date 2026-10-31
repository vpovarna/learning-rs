use core::panic;
use std::{collections::HashMap, println};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
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

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid type"),
        }
    }

    fn joker_rank(&self) -> u8 {
        match self {
            Card::Jack => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Queen => 10,
            Card::King => 11,
            Card::Ace => 12,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: i32,
}

impl Hand {
    fn new(cards_str: &str, bid: i32) -> Hand {
        let cards: [Card; 5] = Self::parse_cards(cards_str);
        let hand_type = Hand::classify(&cards);

        Hand {
            hand_type,
            cards,
            bid,
        }
    }

    fn parse_cards(cards_str: &str) -> [Card; 5] {
        let chars: Vec<char> = cards_str.chars().collect();
        std::array::from_fn(|i| Card::from_char(chars[i]))
    }

    fn classify(cards: &[Card; 5]) -> HandType {
        let mut counts: HashMap<Card, i32> = HashMap::new();
        for card in cards {
            *counts.entry(*card).or_insert(0) += 1
        }

        let mut freqs: Vec<i32> = counts.values().copied().collect();
        freqs.sort_by(|a, b| b.cmp(a));

        match freqs.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn classify_with_jokers(cards: &[Card; 5]) -> HandType {
        let joker_count = cards.iter().filter(|c| **c == Card::Jack).count() as i32;

        let mut counts: HashMap<Card, i32> = HashMap::new();
        for card in cards.iter().filter(|c| **c != Card::Jack) {
            *counts.entry(*card).or_insert(0) += 1;
        }

        let mut freqs: Vec<i32> = counts.values().copied().collect();
        freqs.sort_by(|a, b| b.cmp(a));
        if freqs.is_empty() {
            freqs.push(0); // all five were jokers
        }
        freqs[0] += joker_count;

        match freqs.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let bid = parts[1].parse::<i32>().unwrap();
        let hand: Hand = Hand::new(parts[0], bid);
        hands.push(hand);
    }

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid)
        .sum()
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let bid = parts[1].parse::<i32>().unwrap();
        let cards: [Card; 5] = Hand::parse_cards(parts[0]);
        let hand_type = Hand::classify_with_jokers(&cards);
        hands.push(Hand {
            hand_type,
            cards,
            bid,
        });
    }

    hands.sort_by(|a, b| {
        a.hand_type.cmp(&b.hand_type).then_with(|| {
            a.cards
                .map(|c| c.joker_rank())
                .cmp(&b.cards.map(|c| c.joker_rank()))
        })
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid)
        .sum()
}

fn main() {
    let input =
        std::fs::read_to_string("inputs/2023_day07.txt").expect("Unable to read input file");

    println!("Part1 {:?}", part1(&input));
    println!("Part2 {:?}", part2(&input));
}
