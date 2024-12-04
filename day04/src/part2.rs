use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let input: Vec<Vec<char>> = cont.lines().map(|x| x.chars().collect()).collect();

    let mut res = 0;

    let n = input.len();
    let m = input[0].len();

    for i in 0..n - 2 {
        for j in 0..m - 2 {
            if input[i + 1][j + 1] != 'A' {
                continue;
            }
            let w1_ok = input[i][j] == 'M' && input[i + 2][j + 2] == 'S'
                || input[i][j] == 'S' && input[i + 2][j + 2] == 'M';
            let w2_ok = input[i + 2][j] == 'M' && input[i][j + 2] == 'S'
                || input[i + 2][j] == 'S' && input[i][j + 2] == 'M';
            if w1_ok && w2_ok {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
