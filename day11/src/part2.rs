use std::{collections::HashMap, fs};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut stone_to_count: HashMap<i64, i64> = HashMap::new();
    cont.split(" ").map(|x| x.parse().unwrap()).for_each(|x| {
        stone_to_count.insert(x, 1);
    });

    let steps = 75;
    for step in 0..steps {
        println!("Step {}", step);

        let mut new_stones: HashMap<i64, i64> = HashMap::new();
        for (stone, count) in stone_to_count.iter() {
            if *stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let half = stone_str.len() / 2;
                let (left, right) = stone_str.split_at(half);
                *new_stones.entry(left.parse().unwrap()).or_insert(0) += count;
                *new_stones.entry(right.parse().unwrap()).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_to_count = new_stones;
    }
    let res = stone_to_count.iter().fold(0, |acc, (_, count)| acc + count);
    println!("{:?}", res);
}
