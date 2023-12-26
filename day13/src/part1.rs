use std::cmp::min;

pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let lines = content.lines();
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for line in lines {
        if line.trim().len() == 0 {
            sum += find_reflection(&puzzle);
            puzzle = Vec::new();
            continue;
        }
        puzzle.push(line.chars().collect());
    }
    sum += find_reflection(&puzzle);

    println!("{}", sum);
}

fn find_reflection(puzzle: &Vec<Vec<char>>) -> i32 {
    let n = puzzle.len();
    let m = puzzle[0].len();

    for idx in 0..(n - 1) {
        let mut reflection = true;
        let width = min(idx + 1, n - idx - 1);

        for w_idx in 0..width {
            let left = idx - w_idx;
            let right = idx + 1 + w_idx;

            for j in 0..m {
                if puzzle[left][j] != puzzle[right][j] {
                    reflection = false;
                    break;
                }
            }
            if !reflection {
                break;
            }
        }

        if reflection {
            return 100 * (idx as i32 + 1);
        }
    }

    for idx in 0..m - 1 {
        let mut reflection = true;

        let width = min(idx + 1, m - idx - 1);

        for w_idx in 0..width {
            let left = idx - w_idx;
            let right = idx + 1 + w_idx;

            for i in 0..n {
                if puzzle[i][left] != puzzle[i][right] {
                    reflection = false;
                    break;
                }
            }
            if !reflection {
                break;
            }
        }

        if reflection {
            return idx as i32 + 1;
        }
    }
    panic!("No reflection found")
}
