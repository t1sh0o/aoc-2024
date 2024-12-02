use crate::utils::read_file;

pub fn puzzle1(run_sample: bool) -> i32 {
    let cnt = read_file(2, None, run_sample);
    let mut valid = Vec::new();

    for line in cnt.lines() {
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        if is_ok(&report) {
            valid.push(report);
        }
    }

    valid.len() as i32
}
pub fn puzzle2(run_sample: bool) -> i32 {
    let cnt = read_file(2, None, run_sample);
    let mut valid = Vec::new();

    for line in cnt.lines() {
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        if is_ok(&report) {
            valid.push(report);
        } else {
            for i in 0..report.len() {
                let mut report_slice = report.clone();
                let _ = report_slice.remove(i);

                if is_ok(&report_slice) {
                    valid.push(report);
                    break;
                }
            }
        }
    }

    valid.len() as i32
}

fn is_ok(report: &Vec<isize>) -> bool {
    report
        .windows(2)
        .all(|pair| pair[0] > pair[1] && pair[0] - pair[1] < 4)
        || report
            .windows(2)
            .all(|pair| pair[0] < pair[1] && pair[1] - pair[0] < 4)
}
