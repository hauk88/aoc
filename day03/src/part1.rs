use regex::Regex;
use std::fs;

pub fn part1() {
    // read list of numbers from file
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let res: i32 = re
        .find_iter(cont.as_str())
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
