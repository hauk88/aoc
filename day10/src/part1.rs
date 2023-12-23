pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
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
    for i in 0..n {
        for j in 0..m {
            if maze[i][j] == 'S' {
                start = (i, j);

                let up_d = (start.0 - 1, start.1);
                let down_d = (start.0 + 1, start.1);
                let left_d = (start.0, start.1 - 1);
                let right_d = (start.0, start.1 + 1);

                let up_c = parse_char(&maze[up_d.0][up_d.1])[3];
                let down_c = parse_char(&maze[down_d.0][down_d.1])[1];
                let left_c = parse_char(&maze[left_d.0][left_d.1])[2];
                let right_c = parse_char(&maze[right_d.0][right_d.1])[0];

                let mut c = '.';

                if up_c {
                    if down_c {
                        c = '|';
                    } else if left_c {
                        c = 'J';
                    } else if right_c {
                        c = 'L';
                    }
                }
                if down_c {
                    if left_c {
                        c = '7';
                    }
                    if right_c {
                        c = 'F';
                    }
                }
                if left_c && right_c {
                    c = '-';
                }
                maze[i][j] = c;

                break;
            }
        }
        if start != (0, 0) {
            break;
        }
    }

    let mut prev = (0, 0);
    let mut current = start;
    let mut count = 1;
    loop {
        let up_d = (current.0 - 1, current.1);
        let down_d = (current.0 + 1, current.1);
        let left_d = (current.0, current.1 - 1);
        let right_d = (current.0, current.1 + 1);
        let c = &maze[current.0][current.1];

        let conns = parse_char(c);

        if conns[0] && prev != left_d {
            prev = current;
            current = left_d;
        } else if conns[1] && prev != up_d {
            prev = current;
            current = up_d;
        } else if conns[2] && prev != right_d {
            prev = current;
            current = right_d;
        } else if conns[3] && prev != down_d {
            prev = current;
            current = down_d;
        } else {
            break;
        }

        count += 1;
        if current == start {
            break;
        }
    }

    println!("steps: {:?}", (count - 1) / 2);
}

fn parse_char(c: &char) -> Vec<bool> {
    let mut connections = vec![false, false, false, false];

    match c {
        '|' => {
            connections[1] = true;
            connections[3] = true;
        }
        '-' => {
            connections[0] = true;
            connections[2] = true;
        }
        '7' => {
            connections[0] = true;
            connections[3] = true;
        }
        'L' => {
            connections[1] = true;
            connections[2] = true;
        }
        'F' => {
            connections[2] = true;
            connections[3] = true;
        }
        'J' => {
            connections[0] = true;
            connections[1] = true;
        }
        _ => {}
    }

    return connections;
}
