use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};
use Category::*;

pub fn run() {
    println!("--- DAY 5 ---");
    super::solve_and_time("Part 1: ", p1);
    super::solve_and_time("Part 2: ", p2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone)]
pub struct RangeMap {
    destination: Category,
    source_range: RangeInclusive<u64>,
    destination_range: RangeInclusive<u64>,
}

impl RangeMap {
    fn to_source_ids_range(&self) -> RangeInclusive<u64> {
        *self.source_range.start()..=*self.source_range.end()
    }
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<u64>,
    maps: HashMap<Category, Vec<RangeMap>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.split("\n");
    let seeds = lines.next().unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps = HashMap::new();
    for (header, source, destination) in [
        ("seed-to-soil map:", Seed, Soil),
        ("soil-to-fertilizer map:", Soil, Fertilizer),
        ("fertilizer-to-water map:", Fertilizer, Water),
        ("water-to-light map:", Water, Light),
        ("light-to-temperature map:", Light, Temperature),
        ("temperature-to-humidity map:", Temperature, Humidity),
        ("humidity-to-location map:", Humidity, Location),
    ] {
        parse_range_maps(&mut lines, &mut maps, header, source, destination);
    }

    Input { seeds, maps }
}

fn parse_range_maps<'a, 'b>(
    lines: &'b mut impl Iterator<Item = &'a str>,
    maps: &'b mut HashMap<Category, Vec<RangeMap>>,
    expected_header: &str,
    source: Category,
    destination: Category,
) {
    let mut header = "";
    while header == "" {
        header = lines.next().unwrap().trim();
    }

    let mut range_maps = vec![];
    if header == expected_header {
        loop {
            let line = lines.next().unwrap().trim();
            if line == "" {
                break;
            } else {
                match &line
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    &[destination_range_start, source_range_start, length] => {
                        range_maps.push(RangeMap {
                            destination,
                            source_range: source_range_start..=(source_range_start + length - 1),
                            destination_range: destination_range_start
                                ..=(destination_range_start + length - 1),
                        });
                    }
                    x => {
                        panic!("unexpected map range definition: {:?}", x);
                    }
                }
            }
        }
    }
    // for simplicity, ensure that we always sort maps by their source range starts
    range_maps.sort_by_key(|range_map| *range_map.source_range.start());

    maps.insert(source, range_maps);
}

pub fn p1() -> String {
    let input = parse_input(&std::fs::read_to_string("input_files/day5.txt").unwrap());
    input
        .seeds
        .iter()
        .map(|seed| seed_to_location(&input, *seed))
        .min()
        .unwrap()
        .to_string()
}

fn seed_to_location(input: &Input, seed: u64) -> u64 {
    let mut curr_category = Seed;
    let mut curr_id = seed;
    let mut dest_id = None;
    let mut dest_category = None;

    while let Some(range_maps) = input.maps.get(&curr_category) {
        dest_id = None;
        dest_category = None;

        for range_map in range_maps.iter() {
            dest_category = Some(range_map.destination);

            if curr_id >= *range_map.source_range.start()
                && curr_id <= *range_map.source_range.end()
            {
                dest_id = Some(
                    (curr_id - range_map.source_range.start())
                        + range_map.destination_range.start(),
                );
                break;
            }
        }

        if dest_id.is_none() {
            dest_id = Some(curr_id);
        }

        curr_id = dest_id.unwrap();
        curr_category = dest_category.unwrap();
    }

    if dest_category == Some(Location) {
        dest_id.unwrap()
    } else {
        panic!(
            "couldn't arrive at location, got stuck at {:?} category",
            dest_category
        )
    }
}

pub fn p2() -> String {
    let input = parse_input(&std::fs::read_to_string("input_files/day5.txt").unwrap());
    let mut ranges = input
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..=(chunk[0] + chunk[1] - 1))
        .collect::<Vec<_>>();

    for source_category in [Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity] {
        let maps = input.maps.get(&source_category).unwrap();
        let new_ranges =
            split_ranges_based_on_map_ranges(ranges, maps.iter().map(|m| m.to_source_ids_range()));
        ranges = new_ranges
            .into_iter()
            .map(|range| to_desintation_range(range, maps))
            .collect::<Vec<_>>();
    }

    ranges
        .into_iter()
        .map(|r| *r.start())
        .min()
        .unwrap()
        .to_string()
}

fn intersect_with(
    this: &RangeInclusive<u64>,
    other: &RangeInclusive<u64>,
) -> Vec<RangeInclusive<u64>> {
    // this is outside other => no split
    if (this.start() < other.start() && this.end() < other.start())
        || (this.start() > other.end() && this.end() > other.end())
    {
        vec![this.clone()]
    // this intersects other from the end and this's end is contained inside other
    } else if this.start() < other.start() && this.end() <= other.end() {
        vec![
            *this.start()..=(*other.start() - 1),
            *other.start()..=*this.end(),
        ]
    // ranges overlap [other.start(), this.start(), other.end(), this.end()]
    } else if this.start() > other.start() && this.start() < other.end() && this.end() > other.end()
    {
        vec![
            *this.start()..=*other.end(),
            (*other.end() + 1)..=*this.end(),
        ]
    // this starts in other and continues beyond other
    } else if this.start() >= other.start() && this.end() > other.end() {
        vec![
            *this.start()..=*other.end(),
            (*other.end() + 1)..=*this.end(),
        ]
    // this is fully contained in other
    } else if this.start() >= other.start() && this.end() <= other.end() {
        vec![this.clone()]
    } else {
        panic!(
            "unexpected condition: this = {:?}, other = {:?}",
            this, other
        );
    }
}

fn apply_maps(value: u64, maps: &[RangeMap]) -> u64 {
    match maps
        .iter()
        .find(|m| value >= *m.source_range.start() && value <= *m.source_range.end())
    {
        Some(m) => value + m.destination_range.start() - m.source_range.start(),
        None => value,
    }
}

// split the ranges in such a way that each new range will be mapped to the destination via the same map
// or without using any maps
fn split_ranges_based_on_map_ranges(
    ranges: Vec<RangeInclusive<u64>>,
    map_ranges: impl Iterator<Item = RangeInclusive<u64>>,
) -> Vec<RangeInclusive<u64>> {
    let mut new_ranges = ranges;

    for map_source_ids_range in map_ranges {
        let mut new_ranges_updated = HashSet::new();

        for range in &new_ranges {
            for intersection_range in intersect_with(range, &map_source_ids_range) {
                new_ranges_updated.insert(intersection_range);
            }
        }

        new_ranges = new_ranges_updated.into_iter().collect();
    }

    new_ranges
}

fn to_desintation_range(this: RangeInclusive<u64>, maps: &[RangeMap]) -> RangeInclusive<u64> {
    apply_maps(*this.start(), maps)..=apply_maps(*this.end(), maps)
}
