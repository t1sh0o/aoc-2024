use std::{env::args, process::exit};

use utils::{prepare_for, GetInputError};

mod day1;
mod day2;
mod utils;

fn main() {
    let day = args().nth(1).expect("2 arguments yo be passed");
    let puzzle = args().nth(2).expect("2 arguments yo be passed");
    let run_sample = args().nth(3).is_some();

    match prepare_for(day.as_str()) {
        Ok(_) => {
            println!("Running Day: {}, Puzzle: {}", day, puzzle);

            let res = match day.as_str() {
                "1" => match puzzle.as_str() {
                    "1" => day1::puzzle1(run_sample),
                    "2" => day1::puzzle2(run_sample),
                    _ => {
                        eprintln!("Puzzle {} for day {} not solved yet", puzzle, day);
                        exit(1)
                    }
                },
                "2" => match puzzle.as_str() {
                    "1" => day2::puzzle1(run_sample),
                    "2" => day2::puzzle2(run_sample),
                    _ => {
                        eprintln!("Puzzle {} for day {} not solved yet", puzzle, day);
                        exit(1)
                    }
                },
                _ => {
                    eprintln!("Puzzles for day {} not solved yet", day);
                    exit(1)
                }
            };

            println!("Result: {}", res);
        }
        Err(e) => match e {
            GetInputError::HttpError(msg) => eprintln!("{}", msg),
            GetInputError::FileIsEmpty(filename) => eprintln!(
                "File {} is empty after fetch. Probably you session ID is invalid.",
                filename
            ),
            GetInputError::IoError(e) => eprintln!("Error: {:?}", e),
            GetInputError::VarError(e) => eprintln!("Error: {:?}", e),
        },
    }
}
