use std::collections::HashMap;
const SHOULD_RUN_PART_2_EVEN_THOUGH_IT_IS_VERY_SLOW: bool = false;
const PART_2_PRECALCULATED: usize = 5667240;

#[test]
fn is_part2_and_precalculated_the_same() {
    let cards = get_cards();
    let copies = get_copies(&cards);
    let card_count = cards
        .iter()
        .map(|c| count_copies(&copies, c))
        .sum::<usize>();

    assert_eq!(card_count, PART_2_PRECALCULATED)
}

pub fn run() {
    println!("---DAY 4---");

    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn part1() -> String {
    let cards = get_cards();
    cards.iter().map(|c| c.points()).sum::<usize>().to_string()
}

fn part2() -> String {
    let cards = get_cards();
    let copies = get_copies(&cards);

    // VERY slow lol, takes like 5 seconds
    if SHOULD_RUN_PART_2_EVEN_THOUGH_IT_IS_VERY_SLOW {
        cards
            .iter()
            .map(|c| count_copies(&copies, c))
            .sum::<usize>()
    } else {
        PART_2_PRECALCULATED
    }
    .to_string()
}

#[derive(Clone, Hash, Debug, PartialEq, Default, Eq)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    held: Vec<usize>,
}

impl Card {
    fn winning_count(&self) -> usize {
        self.held
            .iter()
            .filter(|n| self.winning.contains(n))
            .collect::<Vec<&usize>>()
            .len()
    }

    fn points(&self) -> usize {
        if self.winning_count() == 0 {
            return 0;
        }
        2usize.pow(self.winning_count() as u32 - 1)
    }
}

fn get_cards() -> Vec<Card> {
    std::fs::read_to_string("input_files/day4.txt")
        .unwrap()
        .split('\n')
        .filter_map(|s| {
            if s.len() < 2 {
                return None;
            }
            let idx = s[4..8].trim().parse::<usize>().unwrap();
            let parse = |start, end| -> Vec<usize> {
                s[start..end]
                    .trim()
                    .replace("  ", " ")
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect()
            };
            let winning = parse(9, s.find('|').unwrap());
            let held = parse(s.find('|').unwrap() + 1, s.len());

            Some(Card {
                id: idx,
                winning,
                held,
            })
        })
        .collect::<Vec<Card>>()
}

fn get_copies(cards: &Vec<Card>) -> HashMap<Card, Vec<Card>> {
    let mut map = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        let mut copies = Vec::new();
        for j in 0..card.winning_count() {
            copies.push(match cards.get(i + j + 1) {
                Some(c) => c.clone(),
                None => break,
            });
        }
        map.insert(card.clone(), copies);
    }
    map
}

fn count_copies(cards: &HashMap<Card, Vec<Card>>, current: &Card) -> usize {
    1 + match cards.get(current) {
        Some(v) => v.iter().map(|c| count_copies(&cards, c)).sum::<usize>(),
        None => 0,
    }
}
