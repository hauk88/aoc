use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in cont.lines() {
        chars.push(line.chars().collect());
    }

    let m = chars[0].len();
    let n = chars.len();

    let mut moving_op = ' ';
    let mut moving_res = 0;
    let mut global_res = 0;

    for j in 0..m {
        if moving_op == ' ' {
            moving_op = chars[n - 1][j];
            moving_res = if moving_op == '*' { 1 } else { 0 };
        }
        let mut digit_list: Vec<char> = Vec::new();
        for i in 0..(n - 1) {
            let c = chars[i][j];
            if c != ' ' {
                digit_list.push(c);
            }
        }
        if digit_list.is_empty() {
            moving_op = ' ';
            global_res += moving_res;
            continue;
        }
        let digit_str = digit_list.iter().collect::<String>();
        let digit = digit_str.parse::<i64>().unwrap();
        match moving_op {
            '*' => {
                moving_res *= digit;
            }
            '+' => {
                moving_res += digit;
            }
            _ => {
                panic!("Unknown operation")
            }
        }
    }
    global_res += moving_res;

    println!("{}", global_res);
}
