use std::{collections::HashSet, fs};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    let mut n = 0;
    let mut m = 0;
    cont.lines().enumerate().for_each(|(i, line)| {
        n = i as i32;
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                map.insert((i as i32, j as i32));
            }
            if c == '^' {
                current = (i as i32, j as i32);
            }
            m = j as i32;
        });
    });
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut direction = (-1, 0);

    loop {
        visited.insert(current);
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

    println!("{}", visited.len());
}
