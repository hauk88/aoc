pub fn part1() {
    let content = std::fs::read_to_string("shortp11.txt").unwrap();
    let mut maze: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    for i in 0..maze.len() {
        maze[i].insert(0, '.');
        maze[i].push('.');
    }

    let m = maze[0].len();
    maze.insert(0, vec!['.'; m]);
    maze.push(vec!['.'; m]);

    let n = maze.len();

    let mut start = (0, 0);
    let mut current = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if maze[i][j] == 'S' {
                start = (i, j);
                let up = maze[i - 1][j];
                let down = maze[i + 1][j];
                let left = maze[i][j - 1];
                let right = maze[i][j + 1];

                let up_con = up == '|' || up == '7' || up == 'F';
                let down_con = down == '|' || down == 'L' || down == 'J';
                let left_con = left == '-' || left == 'L' || left == 'F';
                let right_con = right == '-' || right == '7' || right == 'J';

                if up_con && down_con {
                    current = (i - 1, j);
                    maze[i][j] = '|';
                } else if left_con && right_con {
                    current = (i, j + 1);
                    maze[i][j] = '-';
                } else if up_con && left_con {
                    current = (i - 1, j);
                    maze[i][j] = 'J';
                } else if up_con && right_con {
                    current = (i - 1, j);
                    maze[i][j] = 'L';
                } else if down_con && left_con {
                    current = (i + 1, j);
                    maze[i][j] = '7';
                } else if down_con && right_con {
                    current = (i, j + 1);
                    maze[i][j] = 'F';
                }

                break;
            }
        }
        if start != (0, 0) {
            break;
        }
    }

    let mut prev = start;
    loop {
        let up = maze[current.0 - 1][current.1];
        let down = maze[current.0 + 1][current.1];
        let left = maze[current.0][current.1 - 1];
        let right = maze[current.0][current.1 + 1];

        if curr == start {
            break;
        }
    }

    println!("start: {:?}", start);
}
