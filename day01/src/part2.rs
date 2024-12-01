use std::{collections::HashMap, fs};

pub fn part2() {
    // read list of numbers from file
    let cont = fs::read_to_string("largep1.txt").expect("dude");

    let mut v1: Vec<i32> = Vec::new();
    let mut m1: HashMap<i32, i32> = HashMap::new();

    for line in cont.lines() {
        let str_numbers: Vec<&str> = line.split_whitespace().collect();
        v1.push(str_numbers[0].parse::<i32>().unwrap());
        let n2 = str_numbers[1].parse::<i32>().unwrap();
        m1.entry(n2).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut res = 0;
    for n1 in v1 {
        let multiplier = m1.get(&n1).unwrap_or(&0);
        res += n1 * multiplier;
    }

    println!("{}", res);
}
