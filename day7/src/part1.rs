// Make enum of types of hands
#[derive(Debug, Eq, PartialEq)]
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
    let content = std::fs::read_to_string("shortp1.txt").expect("Error");
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

    bids.sort_by(|a, b| b.hand_type < a.hand_type);
    println!("{:?}", bids);
}
