use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut count = 0;

    cont.lines().for_each(|bank| {
        let mut bateries: Vec<i32> = bank
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        let mut res: i64 = 0;

        for i in (1..13).rev() {
            let n = bateries.len();
            let (bateries_to_consider, _) = bateries.split_at_mut(n - i + 1);
            let (digit, digit_idx) = first_max(&bateries_to_consider);
            let digit_64 = digit as i64;
            let i_32 = i as u32;
            res += digit_64 * 10_i64.pow(i_32 - 1);

            let (_, bateries_left) = bateries.split_at_mut(digit_idx + 1);
            bateries = bateries_left.to_vec();
        }
        count += res;
    });

    println!("Part 2: {}", count);
}

fn first_max(nums: &[i32]) -> (i32, usize) {
    let mut max = nums[0];
    let mut index = 0;
    nums.iter().enumerate().for_each(|(i, &n)| {
        if n > max {
            max = n;
            index = i;
        }
    });

    return (max, index);
}
