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
    let cnt = read_file(3, None, run_sample);
    let mut valid = Vec::new();
    let mut _do = true;

    let re = Regex::new(r"((do\(\))|(don\'t\(\))|mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    for cap in re.captures_iter(cnt.as_str()) {
        if let Some(_) = &cap.get(2) {
            _do = true;
        } else if let Some(_) = &cap.get(3) {
            _do = false; 
        } else if _do == true {
            let x = &cap.get(4).unwrap();
            let y = &cap.get(5).unwrap();

            let x = x.as_str().parse::<i32>().unwrap();
            let y = y.as_str().parse::<i32>().unwrap();

            valid.push(x * y);
        }
    }
    
    valid.into_iter().sum()
}


