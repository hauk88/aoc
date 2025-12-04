use std::{collections::hash_map::HashMap, fs};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut n = 0;
    let mut m = 0;
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    cont.lines().enumerate().for_each(|(i, line)| {
        n = i;
        line.chars().enumerate().for_each(|(j, c)| {
            m = j;
            map.insert((i, j), c);
        });
    });

    let mut count = 0;
    n += 1;
    m += 1;

    for i in 0..n {
        for j in 0..m {
            let c = map.get(&(i, j)).unwrap();
            if c == &'.' {
                continue;
            }
            let neighbors = get_neighbors(i, j);
            let mut n_count = 0;
            for n in neighbors {
                let nc = map.get(&n).unwrap_or(&'.');
                if nc == c {
                    n_count += 1;
                    if n_count > 3 {
                        break;
                    }
                }
            }
            if n_count <= 3 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn get_neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && nj >= 0 {
                neighbors.push((ni as usize, nj as usize));
            }
        }
    }
    return neighbors;
}
