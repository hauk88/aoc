use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut stones: Vec<i64> = cont.split(" ").map(|x| x.parse().unwrap()).collect();

    let steps = 25;
    for _ in 0..steps {
        for i in (0..stones.len()).rev() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let half = stone_str.len() / 2;
                let (left, right) = stone_str.split_at(half);
                stones[i] = left.parse().unwrap();
                stones.insert(i + 1, right.parse().unwrap());
            } else {
                stones[i] = stone * 2024;
            }
        }
    }
    println!("{:?}", stones.len());
}
