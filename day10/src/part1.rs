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
    for i in 0..n {
        for j in 0..m {
            if maze[i][j] == 'S' {
                start = (i, j);
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
        println!("current: {:?}", current);
        let up_d = (current.0, current.1 - 1);
        let down_d = (current.0, current.1 + 1);
        let left_d = (current.0 - 1, current.1);
        let right_d = (current.0 + 1, current.1);

        let up = maze[up_d.0][up_d.1];
        let down = maze[down_d.0][down_d.1];
        let left = maze[left_d.0][left_d.1];
        let right = maze[right_d.0][right_d.1];

        let up_con = up == '|' || up == '7' || up == 'F';
        let down_con = down == '|' || down == 'L' || down == 'J';
        let left_con = left == '-' || left == 'L' || left == 'F';
        let right_con = right == '-' || right == '7' || right == 'J';

        if up_con && prev != up_d {
            prev = current;
            current = up_d;
        } else if down_con && prev != down_d {
            prev = current;
            current = down_d;
        } else if left_con && prev != left_d {
            prev = current;
            current = left_d;
        } else if right_con && prev != right_d {
            prev = current;
            current = right_d;
        } else {
            break;
        }

        count += 1;
        if current == start {
            break;
        }
    }

    println!("count: {:?}", count);
}
