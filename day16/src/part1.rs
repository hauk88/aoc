use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
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
    let content = std::fs::read_to_string("largep1.txt").expect("Failed to read the file");

    let mut maze: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    for (j, line) in content.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i as i32, j as i32);
            } else if c == 'E' {
                end = (i as i32, j as i32);
            } else if c == '#' {
                maze.insert((i as i32, j as i32));
            }
        }
    }

    let mut queue: Vec<(i32, i32, Direction, u32)> = Vec::new();
    queue.push((start.0, start.1, Direction::East, 0));
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
    let res = loop {
        if queue.is_empty() {
            break 0;
        }
        let (i, j, dir, c) = queue.remove(0);
        if visited.contains(&(i, j, dir.clone())) {
            continue;
        }
        visited.insert((i, j, dir.clone()));
        if (i, j) == end {
            break c;
        }
        let (current_di, current_dj) = dir.to_tuple();
        for try_dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        {
            let (di, dj) = try_dir.clone().to_tuple();
            if current_di + di == 0 && current_dj + dj == 0 {
                continue;
            }
            let (ni, nj) = (i + di, j + dj);
            if !maze.contains(&(ni, nj)) {
                if di == current_di && dj == current_dj {
                    sorted_insert(&mut queue, (ni, nj, try_dir.clone(), c + 1));
                } else {
                    sorted_insert(&mut queue, (ni, nj, try_dir.clone(), c + 1001));
                }
            }
        }
    };
    println!("{}", res);
}

fn sorted_insert(queue: &mut Vec<(i32, i32, Direction, u32)>, element: (i32, i32, Direction, u32)) {
    let mut i = 0;
    while i < queue.len() {
        if queue[i].3 > element.3 {
            break;
        }
        i += 1;
    }
    queue.insert(i, element);
}
