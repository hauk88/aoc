use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_tuple(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

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
    let mut direction = Direction::Up;

    loop {
        visited.insert(current);

        let next = (
            current.0 + direction.to_tuple().0,
            current.1 + direction.to_tuple().1,
        );
        if next.0 < 0 || next.1 < 0 || next.0 > n || next.1 > m {
            break;
        }
        if map.contains(&next) {
            direction = direction.turn_right();
        } else {
            current = next;
        }
    }
    println!("{}", visited.len());
}
