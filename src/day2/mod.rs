use std::{error, slice::Windows};

use crate::utils::read_file;

pub fn puzzle1(run_sample: bool) -> i32 {
    let cnt = read_file(2, None, run_sample);
    let mut valid = 0;

    for line in cnt.lines() {
        println!("{}", line);
        let mut report = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        if report
            .windows(2)
            .all(|pair| pair[0] > pair[1] && pair[0] - pair[1] < 4)
            || report
                .windows(2)
                .all(|pair| pair[0] < pair[1] && pair[1] - pair[0] < 4)
        {
            valid += 1;
        }
    }

    valid
}
pub fn puzzle2(run_sample: bool) -> i32 {
    let cnt = read_file(2, None, run_sample);
    let mut valid = Vec::new();

    for line in cnt.lines() {
        println!("Line {line}");

        let report = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let mut dir = 0;
        let mut has_error = false;
        let mut exited = false;
        let mut error: usize = 0;

        for i in 0..report.len() {
            let val = report.get(i).unwrap();
            let next = report.get(i + 1);
            if next.is_none() {
                continue;
            }
            let next = next.unwrap();
            if dir == 0 {
                if val == next {
                    has_error = true;
                    error = i;

                    break;
                }
            }
            if val > next {
                dir = -1;
            } else if val < next {
                dir = 1;
            }

            if (dir == 1 && (next - val > 3 || next - val < 1))
                || (dir == -1 && (val - next > 3 || val - next < 1))
            {
                println!("Calc Error {val} {next} {dir}");
                has_error = true;
                error = i;
                break;
            }
        }

        if has_error {
            println!("Has One Error {} {line}", report.get(error).unwrap());
            let mut dir = 0;
            for i in 0..report.len() {
                let val = report.get(if i == error { i + 1 } else { i }).unwrap();
                let next = report.get(i + 1);

                if next.is_none() {
                    continue;
                }
                let next = next.unwrap();
                if dir == 0 {
                    if val == next {
                        exited = true;

                        break;
                    }
                }
                if val > next {
                    dir = -1;
                } else if val < next {
                    dir = 1;
                }

                if (dir == 1 && (next - val > 3 || next - val < 1))
                    || (dir == -1 && (val - next > 3 || val - next < 1))
                {
                    println!("Second Calc Error {val} {next} {dir}");
                    exited = true;
                    break;
                }
            }
            if exited {
                exited = false;
                let mut dir = 0;
                error = error + 1;

                for i in 0..report.len() {
                    let val = report.get(i).unwrap();
                    let next = report.get(if i + 1 == error { i + 2 } else { i + 1 });

                    if next.is_none() {
                        continue;
                    }
                    let next = next.unwrap();
                    if dir == 0 {
                        if val == next {
                            exited = true;

                            break;
                        }
                    }
                    if val > next {
                        dir = -1;
                    } else if val < next {
                        dir = 1;
                    }

                    if (dir == 1 && (next - val > 3 || next - val < 1))
                        || (dir == -1 && (val - next > 3 || val - next < 1))
                    {
                        println!("Second Calc Error {val} {next} {dir}");
                        exited = true;
                        break;
                    }
                }
            }
        }

        if !exited {
            valid.push(report);
        }
    }

    println!("Valid Line {:?}", valid);
    valid.len() as i32
}
