use std::{cmp::Ordering, collections::HashMap};

fn get_char_rank(c: char) -> i64 {
    let mut map: HashMap<char, i64> = HashMap::new();
    map.insert('A', 1);
    map.insert('K', 2);
    map.insert('Q', 3);
    map.insert('J', 4);
    map.insert('T', 5);

    return map[&c];
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Bid {
    cards: Vec<char>,
    bid_value: String,
    rank: i64,
}

impl Bid {
    /// Creates a new [`Bid`].
    fn new(cards: Vec<char>, bid_value: String) -> Bid {
        let rank = Bid::get_rank(cards.clone());
        return Self {
            cards,
            bid_value,
            rank,
        };
    }

    fn get_cards_as_string(&self) -> String {
        return self
            .cards
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("");
    }

    fn get_rank(cards: Vec<char>) -> i64 {
        let mut frequency: HashMap<char, u32> = HashMap::new();
        for word in cards.clone() {
            *frequency.entry(word).or_insert(0) += 1;
        }

        let five_of_a_kind: Vec<u32> = vec![5];
        let four_of_a_kind: Vec<u32> = vec![4, 1];
        let full_house: Vec<u32> = vec![3, 2];
        let three_of_a_kind: Vec<u32> = vec![3, 1, 1];
        let two_pair: Vec<u32> = vec![2, 2, 1];
        let one_pair: Vec<u32> = vec![2, 1, 1, 1];
        let high_card: Vec<u32> = vec![1, 1, 1, 1, 1];

        let mut vals: Vec<u32> = frequency
            .clone()
            .values()
            .into_iter()
            .map(|s| s.clone())
            .collect::<Vec<u32>>();

        vals.sort_by(|a, b| b.cmp(a));

        if vals == five_of_a_kind {
            return 1;
        }

        if vals == four_of_a_kind {
            return 2;
        }

        if vals == full_house {
            return 3;
        }

        if vals == three_of_a_kind {
            return 4;
        }

        if vals == two_pair {
            return 5;
        }

        if vals == one_pair {
            return 6;
        }

        if vals == high_card {
            return 7;
        }

        return 8;
    }
}

pub fn solve_part_one(lines: Vec<String>) {
    let mut bids: Vec<Bid> = lines
        .iter()
        .map(|line| {
            let str = line.split(" ").collect::<Vec<&str>>();
            let cards = str[0].chars().collect::<Vec<char>>();
            let bid_value = str[1].to_string();
            return Bid::new(cards, bid_value);
        })
        .collect();

    bids.sort_by(|a, b| {
        let cmp = a.rank.cmp(&b.rank);
        if cmp == Ordering::Equal {
            for i in 0..a.cards.len() {
                if a.cards[i] != b.cards[i] {
                    if a.cards[i].is_digit(10) && b.cards[i].is_digit(10) {
                        return b.cards[i].cmp(&a.cards[i]);
                    };

                    if a.cards[i].is_digit(10) {
                        return Ordering::Less;
                    }

                    if b.cards[i].is_digit(10) {
                        return Ordering::Greater;
                    }

                    return get_char_rank(a.cards[i]).cmp(&get_char_rank(b.cards[i]));
                }
            }
        }
        return cmp;
    });

    let mut result: i64 = 0;
    for i in 0..bids.len() {
        let bid_val = bids[i].bid_value.parse::<i64>().unwrap();
        println!(
            "{} {bid_val} {} {}",
            bids.len() - i,
            bid_val * (bids.len() - i) as i64,
            bids[i].get_cards_as_string()
        );
        result += bid_val * (bids.len() - i) as i64;
    }

    println!("day7_1 {}", result)
}

pub fn solve_part_two(lines: Vec<String>) {
    println!("day7_2 {}", 1);
}
