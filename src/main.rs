use std::{
    env,
    io::{self, Read}
};

mod y2015;

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
        Ok(_b) => {},
        Err(e) => {
            panic!("Failed to read input: {}", e);
        }
    };
    assert!(!input.is_empty());
    match year.as_str() {
        "2015" => match day.as_str() {
            "1" => {
                y2015::day1::solve(&input);
            },
            "2" => {
                y2015::day2::solve(&input);
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
