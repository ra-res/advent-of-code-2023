// https://github.com/ChristopherBiscardi/advent-of-code/blob/2a69fdb905c8d510c55d7f1e5f834f9e6a281292/2023/rust/day-07/src/part1.rs

use itertools::{Itertools, Position};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn solve_part_one(lines: Vec<String>) {
    let hands = lines
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            return (hand, bid.parse::<u32>().unwrap(), score_hand(hand));
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>();
    println!("day7_1 {}", hands.to_string());
}

fn score_hand(hand: &str) -> (Type, (u32, u32, u32, u32, u32)) {
    use Type::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };

    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

fn score_hand_with_jokers(hand: &str) -> (Type, (u32, u32, u32, u32, u32)) {
    use Type::*;

    let counts = hand.chars().counts();
    let values = if let Some(joker_count) = counts.get(&'J') {
        if *joker_count == 5 {
            "5".to_string()
        } else {
            counts
                .iter()
                .filter_map(|(key, value)| (key != &'J').then_some(value))
                .sorted()
                .with_position()
                .map(|(position, value)| match position {
                    Position::Last | Position::Only => value + joker_count,
                    _ => *value,
                })
                .join("")
        }
    } else {
        counts.values().sorted().join("")
    };

    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    };

    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

pub fn solve_part_two(lines: Vec<String>) {
    let hands = lines
        .iter()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            return (
                hand,
                bid.parse::<u32>().unwrap(),
                score_hand_with_jokers(hand),
            );
        })
        .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
        .enumerate()
        .map(|(index, (_hand, bid, _))| (index as u32 + 1) * bid)
        .sum::<u32>();
    println!("day7_2 {}", hands.to_string());
}
