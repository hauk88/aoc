pub fn part2() {
    // read the file
    let content = std::fs::read_to_string("longp2.txt").expect("error");
    let mut times = Vec::new();
    let mut dists = Vec::new();

    for line in content.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let nums = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if parts[0] == "Time" {
            times = nums;
        } else if parts[0] == "Distance" {
            dists = nums;
        }
    }

    let mut res = 1;
    for i in 0..times.len() {
        let mut wins = 0;
        for s in 1..times[i] {
            if calc_dist(s, times[i]) > dists[i] {
                wins += 1;
            }
        }
        res *= wins;
    }

    println!("{:?} ", res)
}

fn calc_dist(hold_time: i64, end_time: i64) -> i64 {
    let run_time = end_time - hold_time;
    let speed = hold_time;
    return run_time * speed;
}
