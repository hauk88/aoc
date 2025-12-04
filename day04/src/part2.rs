use std::{collections::hash_map::HashMap, fs};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    cont.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '@' {
                map.insert((i as i32, j as i32), c);
            }
        });
    });

    let total_paper = map.len();
    loop {
        let n = map.len();
        let remaining_keys: Vec<(i32, i32)> = map.keys().cloned().collect();
        remaining_keys.iter().for_each(|(i, j)| {
            let neighbours = get_neighbors(i, j);
            let mut n_count = 0;
            for n in neighbours {
                if map.contains_key(&n) {
                    n_count += 1;
                    if n_count > 3 {
                        break;
                    }
                }
            }
            if n_count <= 3 {
                map.remove(&(*i, *j));
            }
        });

        if n == map.len() {
            break;
        }
    }

    let res = total_paper - map.len();

    println!("{}", res);
}

fn get_neighbors(i: &i32, j: &i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            neighbors.push((i + di, j + dj));
        }
    }
    return neighbors;
}
