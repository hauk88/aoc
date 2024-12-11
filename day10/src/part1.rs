use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut que: Vec<(usize, usize)> = Vec::new();
    let mut start_positions: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for (i, line) in cont.lines().enumerate() {
        let mut row: Vec<i32> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let h = c.to_string().parse().unwrap();
            if h == 0 {
                start_positions.insert((i, j), HashSet::new());
            }
            if h == 9 {
                que.push((i, j));
            }
            row.push(h);
        }
        map.push(row);
    }

    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let mut top_pos = (0, 0);

    while que.len() != 0 {
        let pos = que.pop().unwrap();
        if map[pos.0][pos.1] == 9 {
            top_pos = pos;
        }
        if map[pos.0][pos.1] == 0 {
            start_positions
                .get_mut(&(pos.0, pos.1))
                .unwrap()
                .insert(top_pos);
        }
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for dir in dirs {
            let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if new_pos.0 < 0 || new_pos.0 >= n || new_pos.1 < 0 || new_pos.1 >= m {
                continue;
            }
            let new_pos_idx = (new_pos.0 as usize, new_pos.1 as usize);
            if map[pos.0][pos.1] - 1 == map[new_pos_idx.0][new_pos_idx.1] {
                que.push(new_pos_idx.clone());
            }
        }
    }

    let res: i32 = start_positions.values().map(|x| x.len() as i32).sum();

    println!("{}", res);
}
