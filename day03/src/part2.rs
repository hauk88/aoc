use regex::Regex;
use std::fs;

pub fn part2() {
    // read list of numbers from file
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut valid_intervals: Vec<Vec<usize>> = Vec::new();

    let re_do = Regex::new(r"do\(\)").unwrap();
    let dos: Vec<usize> = re_do.find_iter(cont.as_str()).map(|x| x.start()).collect();

    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let donts: Vec<usize> = re_dont
        .find_iter(cont.as_str())
        .map(|x| x.start())
        .collect();

    let mut valid = 0;
    for i in 0..donts.len() {
        let stop = donts[i];
        if stop < valid {
            continue;
        }
        valid_intervals.push(vec![valid, stop]);
        for j in 0..dos.len() {
            let start = dos[j];
            if start > stop {
                valid = start;
                break;
            }
        }
    }
    if dos.last().unwrap() > donts.last().unwrap() {
        let a = dos.last().unwrap();
        valid_intervals.push(vec![a.clone(), usize::MAX]);
    }

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let res: i32 = re
        .find_iter(cont.as_str())
        .filter(|x| {
            let mut valid = false;
            for i in 0..valid_intervals.len() {
                if x.start() >= valid_intervals[i][0] && x.start() <= valid_intervals[i][1] {
                    valid = true;
                    break;
                }
            }
            valid
        })
        .map(|x| x.as_str())
        .map(|x| x.replace("mul(", "").replace(")", ""))
        .map(|x| {
            let mut nums = x.split(",");
            let a = nums.next().unwrap().parse::<i32>().unwrap();
            let b = nums.next().unwrap().parse::<i32>().unwrap();
            a * b
        })
        .sum();

    println!("{}", res);
}
