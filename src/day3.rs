// Couldnt be bothered to get this one myself
//
use std::collections::HashMap;

const INDEX_MAX: usize = 139;

type Key = (usize, usize);
type Map = HashMap<Key, String>;

pub fn run() {
    println!("--- DAY 3 ---");
    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn insert_number(key: Key, num: &str, map: &mut Map) {
    for x in 0..num.len() {
        map.insert((key.0, key.1 - x), num.to_string());
    }
}

fn look_around(key: &Key, map: &Map) -> Option<Vec<u32>> {
    assert!(key.0 > 0);
    assert!(key.0 < INDEX_MAX);
    let (row, index) = *key;
    let mut result = vec![];

    /* 4 3 5 * search order matters, as we store the whole number in
     * 1 _ 2 * adjacent positions, to avoid duplications first check
     * 7 6 8 * the same row, then above and below, then the corners
     */

    // range should be [-1, 1], but of course (sarcasm=on),
    // rustc complains about 'usize: Neg'… same below
    for i in [0, 2] {
        if let Some(x) = map.get(&(row, index + i - 1)) {
            if let Ok(num) = x.parse::<u32>() {
                result.push(num);
            }
        }
    }
    'row: for r in [0, 2] {
        for i in [1, 0, 2] {
            if let Some(x) = map.get(&(row + r - 1, index + i - 1)) {
                if let Ok(num) = x.parse::<u32>() {
                    result.push(num);
                    if i == 1 {
                        // break, corners will be the same number, if any
                        continue 'row;
                    }
                }
            }
        }
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

fn part2() -> String {
    let data = include_str!("../input_files/day3.txt");
    let mut map = Map::new();
    for (row, data) in data.lines().enumerate() {
        let mut num = String::new();
        for (i, val) in data.char_indices() {
            match val {
                '.' => {
                    if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                    }
                }
                n if n.is_ascii_digit() => {
                    num.push(n);
                    if i == INDEX_MAX {
                        // digit is the last char in the row
                        insert_number((row, i), &num, &mut map);
                        num.clear();
                    }
                }
                _ => {
                    if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                    }
                    // insert symbol
                    map.insert((row, i), val.to_string());
                }
            }
        }
    }

    // search the map for symbols, and look if they have some adjacent numbers
    let mut sum = 0;
    let mut gears = 0;
    for (k, v) in map.iter() {
        match v.as_str() {
            // simpler than listing the symbols below
            n if n.parse::<u32>().is_ok() => (),

            // woohooo! that's the only addition for part two :D
            // (and the change in the returned data, of course)
            "*" => {
                if let Some(numbers) = look_around(k, &map) {
                    sum += numbers.iter().sum::<u32>();
                    if numbers.len() == 2 {
                        gears += numbers[0] * numbers[1]
                    }
                }
            }

            _ => {
                if let Some(numbers) = look_around(k, &map) {
                    sum += numbers.iter().sum::<u32>();
                }
            }
        }
    }
    gears.to_string()
}

fn part1() -> String {
    let data = include_str!("../input_files/day3.txt");
    let mut map = Map::new();
    for (row, data) in data.lines().enumerate() {
        let mut num = String::new();
        for (i, val) in data.char_indices() {
            match val {
                '.' => {
                    if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                    }
                }
                n if n.is_ascii_digit() => {
                    num.push(n);
                    if i == INDEX_MAX {
                        // digit is the last char in the row
                        insert_number((row, i), &num, &mut map);
                        num.clear();
                    }
                }
                _ => {
                    if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                    }
                    // insert symbol
                    map.insert((row, i), val.to_string());
                }
            }
        }
    }

    // search the map for symbols, and look if they have some adjacent numbers
    let mut sum = 0;
    for (k, v) in map.iter() {
        match v.as_str() {
            // simpler than listing the symbols below
            n if n.parse::<u32>().is_ok() => (),

            _ => {
                if let Some(numbers) = look_around(k, &map) {
                    sum += numbers.iter().sum::<u32>();
                }
            }
        }
    }
    sum.to_string()
}
