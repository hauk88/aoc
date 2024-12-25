use std::{collections::HashSet, fs};

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_tuple(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut current: (i32, i32) = (0, 0);
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut boxes: HashSet<(i32, i32)> = HashSet::new();
    let mut instructions: Vec<Direction> = Vec::new();
    for (y, line) in cont.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((x as i32, y as i32));
                }
                '@' => {
                    current = (x as i32, y as i32);
                }
                'O' => {
                    boxes.insert((x as i32, y as i32));
                }
                '^' => {
                    instructions.push(Direction::North);
                }
                'v' => {
                    instructions.push(Direction::South);
                }
                '>' => {
                    instructions.push(Direction::East);
                }
                '<' => {
                    instructions.push(Direction::West);
                }
                _ => {}
            }
        }
    }

    for dir in instructions {
        let (dx, dy) = dir.to_tuple();
        let (x, y) = current;
        let mut next_available = current.clone();
        loop {
            let next = (next_available.0 + dx, next_available.1 + dy);
            if walls.contains(&next) {
                next_available = current.clone();
                break;
            }
            next_available = next;
            if !boxes.contains(&next) {
                break;
            }
        }
        if next_available == current {
            continue;
        }
        let first = (x + dx, y + dy);
        current = first;
        if next_available != first {
            boxes.remove(&first);
            boxes.insert(next_available);
        }
    }

    let res = boxes.iter().map(|(x, y)| x + 100 * y).sum::<i32>();
    println!("{}", res);
}
