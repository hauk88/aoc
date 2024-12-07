use std::{collections::HashSet, fs};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut start = (0, 0);
    let mut n = 0;
    let mut m = 0;
    cont.lines().enumerate().for_each(|(i, line)| {
        n = i as i32;
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                map.insert((i as i32, j as i32));
            }
            if c == '^' {
                start = (i as i32, j as i32);
            }
            m = j as i32;
        });
    });

    let mut res = 0;
    for i in 0..(n + 1) {
        for j in 0..(m + 1) {
            let new_obstical = (i as i32, j as i32);
            if map.contains(&new_obstical) {
                continue;
            }
            if new_obstical.0 == start.0 && new_obstical.1 == start.1 {
                continue;
            }
            map.insert(new_obstical);
            if check_loop(&map, start, n, m) {
                res += 1;
            }

            map.remove(&new_obstical);
        }
    }
    println!("{}", res);
}

fn check_loop(map: &HashSet<(i32, i32)>, start: (i32, i32), n: i32, m: i32) -> bool {
    let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let mut current = start.clone();
    let mut direction = (-1, 0);

    loop {
        let poshash = (current.0, current.1, direction.0, direction.1);
        if visited.contains(&poshash) {
            return true;
        }
        visited.insert(poshash);

        let next = (current.0 + direction.0, current.1 + direction.1);
        if next.0 < 0 || next.1 < 0 || next.0 > n || next.1 > m {
            break;
        }
        if map.contains(&next) {
            direction = match direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("no dir"),
            }
        } else {
            current = next;
        }
    }

    return false;
}
