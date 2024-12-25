use std::fs;

struct Robot {
    pos: (i32, i32),
    dir: (i32, i32),
}

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let robots: Vec<Robot> = cont
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

    let n = 100;

    let final_pos: Vec<(i32, i32)> = robots
        .iter()
        .map(|r| {
            let mut pos = (r.pos.0 + n * r.dir.0, r.pos.1 + n * r.dir.1);
            pos.0 = pos.0 % w;
            pos.1 = pos.1 % h;
            if pos.0 < 0 {
                pos.0 += w;
            }
            if pos.1 < 0 {
                pos.1 += h;
            }
            return pos;
        })
        .collect();
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let w_m = w / 2;
    let h_m = h / 2;
    for pos in final_pos {
        if pos.0 < w_m {
            if pos.1 < h_m {
                q3 += 1;
            }
            if pos.1 > h_m {
                q2 += 1;
            }
        }
        if pos.0 > w_m {
            if pos.1 < h_m {
                q4 += 1;
            }
            if pos.1 > h_m {
                q1 += 1;
            }
        }
    }
    let res = q1 * q2 * q3 * q4;
    println!("{}", res);
}
