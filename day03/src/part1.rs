use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut count = 0;

    cont.lines().for_each(|bank| {
        let bateries: Vec<i32> = bank
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        let (b1, _) = bateries.split_at(bateries.len() - 1);
        let (max, index) = first_max(&b1.to_vec());
        let (_, p2) = bateries.split_at(index + 1);
        let (max2, _) = first_max(&p2.to_vec());
        let res = 10 * max + max2;
        count += res;
    });

    println!("Part 1: {}", count);
}

fn first_max(nums: &Vec<i32>) -> (i32, usize) {
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
