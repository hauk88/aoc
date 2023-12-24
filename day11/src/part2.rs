use std::usize;

pub fn part2() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let space: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let n = space.len();
    let m = space[0].len();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if space[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut empty_cols: Vec<usize> = (0..m).collect();
    let mut empty_rows: Vec<usize> = (0..n).collect();
    for galaxy in &galaxies {
        empty_cols.retain(|&x| x != galaxy.1 as usize);
        empty_rows.retain(|&x| x != galaxy.0 as usize);
    }

    empty_rows.sort();
    empty_cols.sort();

    let mut sum: i64 = 0;
    let delta = 1000000;
    for i in 0..galaxies.len() {
        let gi = true_pos(galaxies[i], &empty_rows, &empty_cols, delta);
        for j in i + 1..galaxies.len() {
            let gj = true_pos(galaxies[j], &empty_rows, &empty_cols, delta);
            let d = (gi.0 as i64 - gj.0 as i64).abs() + (gi.1 as i64 - gj.1 as i64).abs();
            sum += d;
        }
    }

    println!("{}", sum)
}

fn true_pos(
    p_i: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    n: usize,
) -> (usize, usize) {
    let mut p = p_i.clone();
    let mut found = false;
    for (i, r) in empty_rows.iter().enumerate() {
        if p.0 < *r {
            p.0 += i * (n - 1);
            found = true;
            break;
        }
    }
    if !found {
        p.0 += empty_rows.len() * (n - 1);
    }

    found = false;
    for (i, c) in empty_cols.iter().enumerate() {
        if p.1 < *c {
            p.1 += i * (n - 1);
            found = true;
            break;
        }
    }
    if !found {
        p.1 += empty_cols.len() * (n - 1);
    }
    return p;
}
