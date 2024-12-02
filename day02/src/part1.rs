use std::fs;

pub fn part1() {
    // read list of numbers from file
    let cont = fs::read_to_string("largep1.txt").expect("dude");

    let mut res = 0;
    for line in cont.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if check_numbers(numbers) {
            res += 1;
        }
    }

    println!("{}", res);
}

fn check_numbers(numbers: Vec<i32>) -> bool {
    let diff: Vec<i32> = numbers.windows(2).map(|x| x[0] - x[1]).collect();
    let sign = diff[0].signum();
    for d in diff {
        if d.signum() != sign {
            return false;
        }
        if d.abs() > 3 {
            return false;
        }
    }
    return true;
}
