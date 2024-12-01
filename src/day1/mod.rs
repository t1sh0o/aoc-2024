use crate::utils::read_file;

pub fn puzzle1(run_sample: bool) -> i32 {
    let (mut left, mut right) = get_lists(1, run_sample);

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn puzzle2(run_sample: bool) -> i32 {
    let (left, mut right) = get_lists(1, run_sample);

    right.sort();

    left.into_iter()
        .map(|n| n * right.iter().filter(|m| **m == n).count() as i32)
        .sum()
}

fn get_lists(day: u8, run_sample: bool) -> (Vec<i32>, Vec<i32>) {
    let cnt = read_file(day, None, run_sample);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in cnt.lines() {
        let mut parts = line.split_whitespace();

        left.push(
            parts
                .next()
                .expect("At least to numbers on each row")
                .parse()
                .expect("To be a number"),
        );
        right.push(
            parts
                .next()
                .expect("At least to numbers on each row")
                .parse()
                .expect("To be a number"),
        );
    }

    (left, right)
}
