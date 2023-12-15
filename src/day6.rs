const INPUT: &str = "input_files/day6.txt";

pub fn run() {
    println!("--- DAY 6 ---");

    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn part1() -> String {
    let races = get_races()
        .iter()
        .map(|r| get_winning_times(*r))
        .collect::<Vec<usize>>();
    let mut multi = 1;
    for i in 0..races.len() {
        multi *= races[i]
    }
    multi.to_string()
}

fn get_races() -> Vec<(usize, usize)> {
    let input = std::fs::read_to_string(INPUT)
        .unwrap()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    let get_numbers = |s: &str| {
        if s.len() <= 1 {
            return Err("Vec was none".to_string());
        }
        let mut base = s[s.find(':').unwrap() + 1..s.len()]
            .trim_start()
            .to_string();

        let mut ret = Vec::<usize>::new();
        while let Some(n) = base.find(" ") {
            ret.push(base[0..n].parse().unwrap());
            let clone = base.clone();
            base = clone[n..clone.len()].trim_start().to_string();
        }

        Ok(ret)
    };

    let (times, records) = (
        get_numbers(input.get(0).unwrap()).unwrap(),
        get_numbers(input.get(1).unwrap()).unwrap(),
    );

    let mut ret = Vec::new();
    for i in 0..times.len() {
        ret.push((times[i], records[i]))
    }

    ret
}

fn get_winning_times((time, record): (usize, usize)) -> usize {
    let mut winning_count = 0;
    for i in 0..time + 1 {
        winning_count += (i * (time - i) > record) as usize;
    }
    winning_count
}

fn part2() -> String {
    get_winning_times(get_race()).to_string()
}

fn get_race() -> (usize, usize) {
    let race = std::fs::read_to_string(INPUT)
        .unwrap()
        .replace(' ', "")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    let get_number = |s: String| {
        s[s.find(":").unwrap() + 1..s.len()]
            .parse::<usize>()
            .unwrap()
    };
    (get_number(race[0].clone()), get_number(race[1].clone()))
}
