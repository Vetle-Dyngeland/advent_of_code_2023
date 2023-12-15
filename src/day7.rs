const INPUT: &str = "input_files/day7.txt";

type Hand = [Card; 5];

pub fn run() {
    println!("--- Day 7 ---");
    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn part1() -> String {
    let hands = get_input();
    let ordered = order_hands(hands);
    let mut sum = 0;
    for (i, (_, bet)) in ordered.iter().enumerate() {
        sum += bet * (i + 1)
    }
    sum.to_string()
}

fn part2() -> String {
    let hands = get_input();
    let ordered = order_hands_p2(hands);
    let mut sum = 0;
    for (i, (_, bet)) in ordered.iter().enumerate() {
        sum += bet * (i + 1)
    }
    sum.to_string()
}

fn get_input() -> Vec<(Hand, usize)> {
    let mut input = std::fs::read_to_string(INPUT)
        .unwrap()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    {
        let len = input.len();
        input.remove(len - 1);
    }

    input
        .iter()
        .map(|l| {
            let cards_base = l[0..5].trim().to_string();
            let mut hand = Hand::default();
            for (i, c) in cards_base.chars().enumerate() {
                if i >= 5 {
                    break;
                }
                hand[i] = char_to_card(c)
            }

            let bet = l[5..l.len()].trim().to_string().parse::<usize>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<(Hand, usize)>>()
}

fn order_hands(mut hands: Vec<(Hand, usize)>) -> Vec<(Hand, usize)> {
    use std::cmp::Ordering::*;
    hands.sort_by(|a, b| {
        if is_higher(&a.0, &b.0).is_none() {
            return Equal;
        }
        let higher = is_higher(&a.0, &b.0).unwrap();
        if higher {
            Greater
        } else {
            Less
        }
    });
    hands
}

fn order_hands_p2(mut hands: Vec<(Hand, usize)>) -> Vec<(Hand, usize)> {
    use std::cmp::Ordering::*;
    hands.sort_by(|a, b| {
        if is_higher_p2(&a.0, &b.0).is_none() {
            return Equal;
        }
        let higher = is_higher_p2(&a.0, &b.0).unwrap();
        if higher {
            Greater
        } else {
            Less
        }
    });
    hands
}

fn hand_type(type_counts: Vec<(Card, usize)>) -> HandType {
    match type_counts.len() {
        1 => HandType::FiveKind,
        2 => {
            if type_counts[0].1 == 4 || type_counts[1].1 == 4 {
                HandType::FourKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            for (_, count) in type_counts {
                if count == 3 {
                    return HandType::ThreeKind;
                }
            }
            HandType::TwoPair
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn type_counts(hand: &Hand) -> Vec<(Card, usize)> {
    let mut bases = Vec::<(Card, usize)>::new();
    let add_to_base = |card: Card, bases: &Vec<(Card, usize)>| {
        for i in 0..bases.len() {
            if card == bases[i].0 {
                return Some(i);
            }
        }
        None
    };

    for &card in hand {
        if let Some(i) = add_to_base(card, &bases) {
            bases[i].1 += 1;
            continue;
        }
        bases.push((card, 1));
    }

    bases
}

fn type_counts_p2(hand: &Hand) -> Vec<(Card, usize)> {
    let mut bases = Vec::<(Card, usize)>::new();
    let mut jokers = 0usize;

    let add_to_base = |card: Card, bases: &mut Vec<(Card, usize)>| {
        for i in 0..bases.len() {
            if card == bases[i].0 {
                bases[i].1 += 1;
                return true;
            }
        }
        false
    };

    for &card in hand {
        if card == Card::J {
            jokers += 1;
            continue;
        }
        if add_to_base(card, &mut bases) {
            continue;
        }
        bases.push((card, 1));
    }

    if bases.len() == 0 {
        return vec![(Card::J, 5)];
    }

    bases.sort_by(|(_, a), (_, b)| {
        if a == b {
            std::cmp::Ordering::Equal
        } else if a > b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    
    bases[0].1 += jokers;

    bases
}

fn char_to_card(c: char) -> Card {
    if let Ok(n) = c.to_string().parse::<usize>() {
        return Card::Number(n);
    }

    match c {
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => panic!("char {c} was not parsable into card"),
    }
}

/// Returns None if equal, else returns true or false
fn is_higher(hand: &Hand, other: &Hand) -> Option<bool> {
    let (this_type, other_type) = (hand_type(type_counts(hand)), hand_type(type_counts(other)));
    if this_type != other_type {
        return Some(this_type > other_type);
    }

    for i in 0..hand.len() {
        if hand[i] != other[i] {
            return Some(hand[i] > other[i]);
        }
    }
    return None;
}

fn is_higher_p2(hand: &Hand, other: &Hand) -> Option<bool> {
    let (this_type, other_type) = (
        hand_type(type_counts_p2(hand)),
        hand_type(type_counts_p2(other)),
    );
    if this_type != other_type {
        return Some(this_type > other_type);
    }

    for i in 0..hand.len() {
        if hand[i] != other[i] {
            return Some(hand[i] > other[i]);
        }
    }
    return None;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone, Default)]
enum Card {
    #[default]
    J,
    Number(usize),
    T,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
