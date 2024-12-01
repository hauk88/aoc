use std::fs;

pub fn part1() {
    // read list of numbers from file
    let cont = fs::read_to_string("largep1.txt").expect("dude");

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in cont.lines() {
        let str_numbers: Vec<&str> = line.split_whitespace().collect();
        v1.push(str_numbers[0].parse::<i32>().unwrap());
        v2.push(str_numbers[1].parse::<i32>().unwrap());
    }

    // sort v1 and v2
    v1.sort();
    v2.sort();

    let mut res = 0;
    for (idx, n1) in v1.iter().enumerate() {
        res += (n1 - v2[idx]).abs()
    }

    println!("{}", res);
}
