use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut dial = 50;

    let nums: Vec<i32> = cont
        .lines()
        .map(|x| x.trim())
        .map(|x| {
            let (dir, num_str) = x.split_at(1);
            let mut num: i32 = num_str.parse().unwrap();
            if dir == "L" {
                num = -num;
            }
            return num;
        })
        .collect();

    let mut count = 0;

    nums.iter().for_each(|x| {
        dial += x;
        dial = dial % 100;
        if dial == 0 {
            count += 1;
        }
    });

    println!("{}", count);
}
