use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let nums: Vec<i32> = cont
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let mut new_layout: Vec<(i32, i32)> = Vec::new();

    for i in 0..nums.len() {
        new_layout.push((
            match i % 2 {
                0 => (i as i32) / 2,
                1 => -1,
                _ => panic!("Error"),
            },
            nums[i],
        ));
    }

    for j in (0..new_layout.len()).rev() {
        if new_layout[j].0 < 0 {
            continue;
        }
        for i in 0..j {
            if new_layout[i].0 >= 0 {
                continue;
            }
            if new_layout[i].1 < new_layout[j].1 {
                continue;
            }

            let move_item = new_layout[j].clone();
            new_layout[j] = (-1, move_item.1);
            new_layout.insert(i, move_item);
            let rest_space = new_layout[i + 1].1 - move_item.1;
            new_layout[i + 1] = (-1, rest_space);

            break;
        }
    }

    let mut res: i64 = 0;
    let mut idx = 0;
    for i in 0..new_layout.len() {
        if new_layout[i].0 < 0 {
            idx += new_layout[i].1;
            continue;
        }
        for _j in 0..new_layout[i].1 {
            res += (idx as i64) * (new_layout[i].0 as i64);
            idx += 1;
        }
    }

    println!("{}", res);
}

fn print_layout(layout: &Vec<(i32, i32)>) {
    for i in 0..layout.len() {
        for _j in 0..layout[i].1 {
            if layout[i].0 < 0 {
                print!(".");
            } else {
                print!("{}", layout[i].0);
            }
        }
    }
    println!();
}
