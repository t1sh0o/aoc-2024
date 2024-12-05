use crate::utils::read_file;
use regex::Regex;

pub fn puzzle1(run_sample: bool) -> i32 {
    let cnt = read_file(3, None, run_sample);
    let mut valid = Vec::new();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for (_, [x, y]) in re.captures_iter(cnt.as_str()).map(|c| c.extract()) {
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();

        valid.push(x * y);
    }

    valid.into_iter().sum()
}
pub fn puzzle2(run_sample: bool) -> i32 {
    let cnt = read_file(2, None, run_sample);

    0
}
