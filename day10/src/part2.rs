use std::fs;

#[derive(Debug, Clone, Copy)]
struct TrailPoint {
    h: i32,
    tops_reachable: i32,
}

impl TrailPoint {
    fn new(h: i32) -> TrailPoint {
        TrailPoint {
            h,
            tops_reachable: 0,
        }
    }
}

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut map: Vec<Vec<TrailPoint>> = Vec::new();
    let mut que: Vec<(usize, usize)> = Vec::new();
    let mut start_positions: Vec<(usize, usize)> = Vec::new();
    for (i, line) in cont.lines().enumerate() {
        let mut row: Vec<TrailPoint> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let tp = TrailPoint::new(c.to_string().parse().unwrap());
            if tp.h == 0 {
                start_positions.push((i, j));
            }
            if tp.h == 9 {
                que.push((i, j));
            }
            row.push(tp);
        }
        map.push(row);
    }

    let n = map.len() as i32;
    let m = map[0].len() as i32;

    while que.len() != 0 {
        let pos = que.remove(0);
        map[pos.0][pos.1].tops_reachable += 1;
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for dir in dirs {
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if new_pos.0 < 0 || new_pos.0 >= n || new_pos.1 < 0 || new_pos.1 >= m {
                continue;
            }
            let new_pos_idx = (new_pos.0 as usize, new_pos.1 as usize);
            if map[pos.0][pos.1].h - 1 == map[new_pos_idx.0][new_pos_idx.1].h {
                que.push(new_pos_idx.clone());
            }
        }
    }

    let res: i32 = start_positions
        .iter()
        .map(|x| map[x.0][x.1].tops_reachable)
        .sum();

    println!("{}", res);
}
