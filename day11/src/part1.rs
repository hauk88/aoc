pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let mut space: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
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
    for galaxy in galaxies {
        empty_cols.retain(|&x| x != galaxy.1 as usize);
        empty_rows.retain(|&x| x != galaxy.0 as usize);
    }

    empty_rows.sort();
    empty_rows.reverse();

    for er in empty_rows {
        space.insert(er, (0..m).map(|_| '.').collect());
    }

    empty_cols.sort();
    empty_cols.reverse();

    for j in empty_cols {
        for i in 0..space.len() {
            space[i].insert(j, '.');
        }
    }

    let n1 = space.len();
    let m1 = space[0].len();

    let mut galaxies1: Vec<(i32, i32)> = Vec::new();

    for i in 0..n1 {
        for j in 0..m1 {
            if space[i][j] == '#' {
                galaxies1.push((i as i32, j as i32));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxies1.len() {
        let gi = galaxies1[i];
        for j in i + 1..galaxies1.len() {
            let gj = galaxies1[j];
            let d = (gi.0 - gj.0).abs() + (gi.1 - gj.1).abs();
            sum += d;
        }
    }

    println!("{}", sum)
}
