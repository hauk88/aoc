use std::fs;

pub fn part2() {
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
    let mut nums_c = nums.clone();
    let first = nums_c.remove(0);
    if nums_c.len() == 0 {
        return target == first;
    }

    let mut add_nums = nums_c.clone();
    add_nums[0] = add_nums[0] + first;

    let mut mul_nums = nums_c.clone();
    mul_nums[0] = mul_nums[0] * first;

    let mut cat_nums = nums_c.clone();
    let mut new_num = first.to_string();
    new_num.push_str(&cat_nums[0].to_string());
    cat_nums[0] = new_num.parse().unwrap();

    return can_reach_target(target, add_nums)
        || can_reach_target(target, mul_nums)
        || can_reach_target(target, cat_nums);
}
