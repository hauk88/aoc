use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut input: Vec<(i64, Vec<i64>)> = Vec::new();
    cont.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let target: i64 = parts[0].parse().unwrap();
        let nums: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        input.push((target, nums));
    });

    let mut res = 0;

    for lines in input {
        if can_reach_target(lines.0, lines.1) {
            res += lines.0;
        }
    }

    println!("{}", res);
}

fn can_reach_target(target: i64, nums: Vec<i64>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let mut add_nums = nums.clone();
    let rm_num = add_nums.remove(0);
    add_nums[0] = add_nums[0] + rm_num;

    let mut mul_nums = nums.clone();
    let mul_rm_num = mul_nums.remove(0);
    mul_nums[0] = mul_nums[0] * mul_rm_num;

    return can_reach_target(target, add_nums) || can_reach_target(target, mul_nums);
}
