use std::fs;

struct Robot {
    pos: (i32, i32),
    dir: (i32, i32),
}

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut robots: Vec<Robot> = cont
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let p_str = parts[0].split("=").collect::<Vec<&str>>()[1];
            let d_str = parts[1].split("=").collect::<Vec<&str>>()[1];

            let pos = p_str
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let dir = d_str
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Robot {
                pos: (pos[0], pos[1]),
                dir: (dir[0], dir[1]),
            }
        })
        .collect();
    let w = 101;
    let h = 103;

    // let w = 11;
    // let h = 7;
    let mut i = 0;
    loop {
        // // read stdin
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        // if input.trim() == "q" {
        //     break;
        // }
        let no_robots_in_top_row = robots.iter().all(|r| r.pos.1 != 0);
        if (no_robots_in_top_row) {
            println!("i: {}", i);
            print_map(&robots, w, h);
        }

        i += 1;
        robots = robots
            .iter()
            .map(|r| {
                let mut pos = (r.pos.0 + r.dir.0, r.pos.1 + r.dir.1);
                pos.0 = pos.0 % w;
                pos.1 = pos.1 % h;
                if pos.0 < 0 {
                    pos.0 += w;
                }
                if pos.1 < 0 {
                    pos.1 += h;
                }
                Robot {
                    pos: pos,
                    dir: r.dir,
                }
            })
            .collect();
    }
}

fn print_map(robots: &Vec<Robot>, w: i32, h: i32) {
    let mut map = vec![vec!['.'; w as usize]; h as usize];
    for r in robots {
        map[r.pos.1 as usize][r.pos.0 as usize] = '#';
    }
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}
