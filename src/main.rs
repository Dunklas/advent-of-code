use std::{
    env,
    io::{self, Read},
};

mod y2015;
mod y2023;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = match args.get(1) {
        Some(year) => year,
        None => {
            panic!("Missing year!")
        }
    };
    let day = match args.get(2) {
        Some(day) => day,
        None => {
            panic!("Missing day!")
        }
    };
    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(_b) => {}
        Err(e) => {
            panic!("Failed to read input: {}", e);
        }
    };
    assert!(!input.is_empty());
    match year.as_str() {
        "2015" => match day.as_str() {
            "1" => {
                y2015::day1::solve(&input);
            }
            "2" => {
                y2015::day2::solve(&input);
            }
            "3" => {
                y2015::day3::solve(&input);
            }
            _ => {
                panic!("Solution does not exist for {} {}", year, day);
            }
        },
        "2023" => match day.as_str() {
            "1" => {
                y2023::day1::solve(&input);
            }
            "2" => {
                y2023::day2::solve(&input);
            }
            "3" => {
                y2023::day3::solve(&input);
            }
            "4" => {
                y2023::day4::solve(&input);
            }
            "5" => {
                y2023::day5::solve(&input);
            }
            "6" => {
                y2023::day6::solve(&input);
            }
            "7" => {
                y2023::day7::solve(&input);
            }
            "8" => {
                y2023::day8::solve(&input);
            }
            "9" => {
                y2023::day9::solve(&input);
            }
            "10" => {
                y2023::day10::solve(&input);
            }
            "11" => {
                y2023::day11::solve(&input);
            }
            "14" => {
                y2023::day14::solve(&input);
            }
            "15" => {
                y2023::day15::solve(&input);
            }
            _ => {
                panic!("Solution does not exist for {} {}", year, day);
            }
        },
        _ => {
            panic!("Solution does not exist for {} {}", year, day);
        }
    }
}
