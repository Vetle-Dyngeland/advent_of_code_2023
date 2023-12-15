use std::collections::HashMap;

const INPUT: &str = "input_files/day8.txt";

type Label = [char; 3];
type Node = [Label; 3];

const START: Label = ['A', 'A', 'A'];
const END: Label = ['Z', 'Z', 'Z'];

pub fn run() {
    println!("--- DAY 8 ---");
    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn part1() -> String {
    let instructions = get_instructions();
    let network = get_network();
    solve(&instructions, &network).to_string()
}

fn part2() -> String {
    let instructions = get_instructions();
    let network = get_network();
    let labels = find_start_labels(&network);

    let counts = labels
        .iter()
        .map(|&l| get_node_travel_len(&network, &instructions, l))
        .collect::<Vec<usize>>();

    least_common_multiple(counts).to_string()
}

fn least_common_multiple(counts: Vec<usize>) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let mut multi = 1;
    for count in counts {
        multi = count * multi / gcd(count, multi);
    }
    multi
}

fn get_node_travel_len(
    network: &HashMap<Label, Node>,
    instructions: &Vec<Instruction>,
    mut label: Label,
) -> usize {
    let mut count = 0;
    while label[2] != 'Z' {
        let label_clone = label.clone();
        label = network
            .get(
                &network.get(&label_clone).unwrap()
                    [instructions[count % instructions.len()] as usize],
            )
            .unwrap()[0];
        count += 1;
    }
    count
}

fn get_instructions() -> Vec<Instruction> {
    std::fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>()[0]
        .trim()
        .chars()
        .map(|c| Instruction::from_char(c).unwrap())
        .collect::<Vec<Instruction>>()
}

fn get_network() -> HashMap<Label, Node> {
    let network = std::fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>();

    network[2..network.len() - 1]
        .iter()
        .map(|s| {
            let n = [
                s[0..3].to_string(),
                s[7..10].to_string(),
                s[12..15].to_string(),
            ]
            .map(|s| {
                let mut chars = s.chars();
                [
                    chars.nth(0).unwrap(),
                    chars.nth(0).unwrap(),
                    chars.nth(0).unwrap(),
                ]
            });
            (n[0], n)
        })
        .collect()
}

fn solve(instructions: &Vec<Instruction>, network: &HashMap<Label, Node>) -> usize {
    let mut step_count = 0usize;
    let mut current_label = START;
    while current_label != END {
        let label = solve_one_step(&instructions, &network, current_label, step_count);
        current_label = label;
        step_count += 1
    }

    step_count
}

fn solve_one_step(
    instructions: &Vec<Instruction>,
    network: &HashMap<Label, Node>,
    current_label: Label,
    current_step: usize,
) -> Label {
    network
        .get(
            &network.get(&current_label).unwrap()
                [instructions[current_step % instructions.len()] as usize],
        )
        .unwrap()[0]
}

fn find_start_labels(network: &HashMap<Label, Node>) -> Vec<Label> {
    network
        .keys()
        .filter(|n| n[2] == 'A')
        .map(|v| *v)
        .collect::<Vec<Label>>()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Instruction {
    Left = 1,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => return None,
        })
    }
}
