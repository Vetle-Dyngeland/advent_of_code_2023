pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day10;

fn main() {
    day1::run();
    day2::run();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day7::run();
    day8::run();
    day10::run();
}

pub fn solve_and_time(preceding_string: &str, function: fn() -> String) {
    let start = std::time::Instant::now();

    let result = function();
    println!(
        "{preceding_string}{result} and took {} seconds to execute",
        (start.elapsed().as_secs_f32() * 1000f32).round() / 1000f32
    );
}
