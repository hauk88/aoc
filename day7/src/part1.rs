use std::{cmp::Ordering, collections::HashMap};

// Make enum of types of hands
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Hand {
    Hc,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Bid {
    hand: String,
    value: i32,
    hand_type: Hand,
}

impl Bid {
    fn compare(&self, other: &Bid) -> Ordering {
        let a = self.hand_type.cmp(&other.hand_type);
        let mut card_to_value: HashMap<char, i32> = HashMap::new();
        card_to_value.insert('2', 2);
        card_to_value.insert('3', 3);
        card_to_value.insert('4', 4);
        card_to_value.insert('5', 5);
        card_to_value.insert('6', 6);
        card_to_value.insert('7', 7);
        card_to_value.insert('8', 8);
        card_to_value.insert('9', 9);
        card_to_value.insert('T', 10);
        card_to_value.insert('J', 11);
        card_to_value.insert('Q', 12);
        card_to_value.insert('K', 13);
        card_to_value.insert('A', 14);

        if a.is_eq() {
            let self_cards = self.hand.chars().collect::<Vec<char>>();
            let other_cards = other.hand.chars().collect::<Vec<char>>();
            for i in 0..self_cards.len() {
                let self_value = card_to_value.get(&self_cards[i]).unwrap();
                let other_value = card_to_value.get(&other_cards[i]).unwrap();
                let cmp = self_value.cmp(&other_value);
                if cmp.is_ne() {
                    return cmp;
                }
            }
        }
        return a;
    }
}

fn parse_hand(hand: String) -> Hand {
    let chars = hand.chars().collect::<Vec<char>>();
    let mut current_chars = chars.clone();
    let mut kinds = Vec::new();
    loop {
        let orig_len = current_chars.len();
        if orig_len == 0 {
            break;
        }
        let first_c = current_chars[0].clone();
        current_chars.retain(|&c| c != first_c);
        kinds.push(orig_len - current_chars.len());
    }
    kinds.sort();
    kinds.reverse();
    let res = match kinds.len() {
        1 => Hand::FiveOfAKind,
        2 => {
            if kinds[0] == 4 {
                return Hand::FourOfAKind;
            } else {
                return Hand::FullHouse;
            }
        }
        3 => {
            if kinds[0] == 3 {
                return Hand::ThreeOfAKind;
            } else {
                return Hand::TwoPair;
            }
        }
        4 => Hand::OnePair,
        _ => Hand::Hc,
    };
    return res;
}

pub fn part1() {
    // read file
    let content = std::fs::read_to_string("longp1.txt").expect("Error");
    let mut bids: Vec<Bid> = Vec::new();
    for line in content.lines() {
        let mut bid = Bid {
            hand: String::new(),
            value: 0,
            hand_type: Hand::Hc,
        };
        let parts = line.split(" ").collect::<Vec<&str>>();
        bid.hand = parts[0].to_string();
        bid.value = parts[1].parse::<i32>().unwrap();
        bid.hand_type = parse_hand(bid.hand.clone());
        bids.push(bid);
    }

    // sort bid by hand type
    bids.sort_by(|a, b| a.compare(b));

    let mut res = 0;
    for (idx, bid) in bids.iter().enumerate() {
        res += bid.value * ((idx as i32) + 1);
    }

    println!("{:?}", res);
}
