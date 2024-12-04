use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let input: Vec<Vec<char>> = cont.lines().map(|x| x.chars().collect()).collect();

    let mut res = 0;

    let n = input.len();
    let m = input[0].len();

    for i in 0..n {
        let row = &input[i];
        row.windows(4).for_each(|x| {
            if x[0] == 'X' && x[1] == 'M' && x[2] == 'A' && x[3] == 'S' {
                res += 1;
            }
            if x[3] == 'X' && x[2] == 'M' && x[1] == 'A' && x[0] == 'S' {
                res += 1;
            }
        });
    }

    for i in 0..m {
        let mut column = Vec::new();
        for j in 0..n {
            column.push(input[j][i]);
        }
        column.windows(4).for_each(|x| {
            if x[0] == 'X' && x[1] == 'M' && x[2] == 'A' && x[3] == 'S' {
                res += 1;
            }
            if x[3] == 'X' && x[2] == 'M' && x[1] == 'A' && x[0] == 'S' {
                res += 1;
            }
        });
    }

    for i in 0..n {
        for j in 0..m {
            if j + 3 >= m || i + 3 >= n {
                continue;
            }
            if input[i][j] == 'X'
                && input[i + 1][j + 1] == 'M'
                && input[i + 2][j + 2] == 'A'
                && input[i + 3][j + 3] == 'S'
            {
                res += 1;
            }
            if input[i][j] == 'S'
                && input[i + 1][j + 1] == 'A'
                && input[i + 2][j + 2] == 'M'
                && input[i + 3][j + 3] == 'X'
            {
                res += 1;
            }
        }
    }

    for i in 3..n {
        for j in 0..m - 3 {
            if input[i][j] == 'X'
                && input[i - 1][j + 1] == 'M'
                && input[i - 2][j + 2] == 'A'
                && input[i - 3][j + 3] == 'S'
            {
                res += 1;
            }
            if input[i][j] == 'S'
                && input[i - 1][j + 1] == 'A'
                && input[i - 2][j + 2] == 'M'
                && input[i - 3][j + 3] == 'X'
            {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
