use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let nums: Vec<i32> = cont
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let mut new_layout: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        for _ in 0..nums[i] {
            new_layout.push(match i % 2 {
                0 => (i as i32) / 2,
                1 => -1,
                _ => panic!("Error"),
            });
        }
    }

    let mut i = 0;
    let mut j = new_layout.len() - 1;
    loop {
        if i >= j {
            break;
        }
        if new_layout[i] >= 0 {
            i += 1;
            continue;
        }
        if new_layout[j] < 0 {
            j -= 1;
            continue;
        }
        new_layout[i] = new_layout[j];
        new_layout[j] = -1;
    }

    let mut res: i64 = 0;
    for i in 0..new_layout.len() {
        if new_layout[i] < 0 {
            break;
        }
        res += (i as i64) * (new_layout[i] as i64);
    }

    println!("{}", res);
}
