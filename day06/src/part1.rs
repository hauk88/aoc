use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<&str> = Vec::new();
    for line in cont.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        if tokens[0] == "*" || tokens[0] == "+" {
            ops = tokens;
        } else {
            nums.push(
                tokens
                    .iter()
                    .map(|t| t.parse().unwrap())
                    .collect::<Vec<i64>>(),
            );
        }
    }

    let mut global_res = 0;
    let n = nums[0].len();
    let m = nums.len();
    for i in 0..n {
        let op = ops[i];
        let res = match op {
            "*" => (0..m).map(|j| nums[j][i]).product::<i64>(),
            "+" => (0..m).map(|j| nums[j][i]).sum::<i64>(),
            _ => panic!("Unknown operation"),
        };
        global_res += res;
    }
    println!("{}", global_res);
}
