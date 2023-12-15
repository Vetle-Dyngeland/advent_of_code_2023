use std::error::Error;

use csv::StringRecord;

// 0 = red, 1 = green, 2 = blue
const LIMITS: [usize; 3] = [12, 13, 14];

pub fn run() {
    println!("\n--- DAY 2 ---");

    super::solve_and_time("Part 1: ", part_1);
    super::solve_and_time("Part 2: ", part_2);
}

pub fn parse_csv() -> Result<Vec<(usize, Vec<[usize; 3]>)>, Box<dyn Error>> {
    let mut ret: Vec<(usize, Vec<[usize; 3]>)> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .flexible(true)
        .from_path("input_files/day2.csv")?;

    for result in rdr.records() {
        let record = result?;

        ret.push((get_index(&record), get_values(&record)));
    }
    Ok(ret)
}

fn get_index(record: &StringRecord) -> usize {
    let index_base = record.get(0).unwrap().replace("Game ", "");
    let index_end_index = index_base.find(":").unwrap();

    index_base[0..index_end_index].parse().unwrap()
}

fn get_values(record: &StringRecord) -> Vec<[usize; 3]> {
    let mut ret = Vec::new();
    for string in record.iter() {
        let start_index = string.find(':').unwrap_or(0);
        let read_base = string[start_index + 1..string.len()].trim().to_string();
        let values = read_base
            .split(",")
            .map(|s| s.replace(" ", ""))
            .collect::<Vec<String>>();

        let mut result = [0usize; 3];

        for value in values.iter() {
            let mut chars = value.chars();
            let mut number_end_index = 0usize;
            while chars.next().unwrap().is_ascii_digit() {
                number_end_index += 1;
            }
            let number: usize = match value[0..number_end_index].parse() {
                Ok(val) => val,
                Err(_) => panic!(
                    "Could not parse int. Value: {}",
                    value[0..number_end_index].to_string()
                ),
            };

            match &value[number_end_index..value.len()] {
                "red" => result[0] += number,
                "green" => result[1] += number,
                "blue" => result[2] += number,
                _ => panic!(
                    "Value did not match pattern! Value {}",
                    &value[number_end_index..value.len()]
                ),
            }
        }
        ret.push(result)
    }

    ret
}

fn part_1() -> String {
    let values = parse_csv().unwrap_or_else(|e| {
        println!("Error occured: \n   {}", e.to_string());
        panic!();
    });
    let mut index_sum = 0usize;
    for (index, value) in values.iter() {
        index_sum += is_game_possible(value) as usize * index;
    }
    index_sum.to_string()
}

fn is_game_possible(game: &Vec<[usize; 3]>) -> bool {
    for values in game.iter() {
        for i in 0..values.len() {
            if values[i] > LIMITS[i] {
                return false;
            }
        }
    }
    true
}

fn part_2() -> String {
    let values = parse_csv().unwrap_or_else(|e| {
        println!("Error occured: \n   {}", e.to_string());
        panic!();
    });
    let mut power_sum = 0usize;
    for (_, value) in values.iter() {
        power_sum += get_power_sum(value)
    }
    power_sum.to_string()
}

fn get_power_sum(game: &Vec<[usize; 3]>) -> usize {
    let (mut a_max, mut b_max, mut c_max) = (0, 0, 0);
    for [a, b, c] in game.iter().map(|v| *v) {
        if a > a_max {
            a_max = a
        }
        if b > b_max {
            b_max = b
        }
        if c > c_max {
            c_max = c
        }
    }

    a_max * b_max * c_max
}
