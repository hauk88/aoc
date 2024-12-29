use std::collections::HashSet;

use pathfinding::prelude::astar_bag;

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

pub fn part2() {
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

    let res = astar_bag(
        &(start.0, start.1, Direction::East),
        |(i, j, dir)| {
            let mut res = Vec::new();
            let (current_di, current_dj) = dir.clone().to_tuple();
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
                        res.push(((ni, nj, try_dir.clone()), 1));
                    } else {
                        res.push(((ni, nj, try_dir.clone()), 1001));
                    }
                }
            }
            res
        },
        |_| 0,
        |(i, j, _)| *i == end.0 && *j == end.1,
    );
    let (paths, _cost) = res.unwrap();
    let mut unique_positions: HashSet<(i32, i32)> = HashSet::new();
    paths.for_each(|path| {
        path.iter().for_each(|(i, j, _)| {
            let p = (*i, *j);
            unique_positions.insert(p);
        })
    });

    println!("{}", unique_positions.len());
}
