pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();

    let secs = content
        .lines()
        .map(|line| {
            let parts = line.trim().split_whitespace();
            return parts
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for sec in secs {
        sum += calc_next(&sec);
    }

    println!("sum: {}", sum);
}

fn calc_next(sec: &Vec<i32>) -> i32 {
    let mut layers: Vec<Vec<i32>> = Vec::new();
    layers.push(sec.to_vec());
    loop {
        let current = layers.last().unwrap();
        if current.iter().all(|&x| x == 0) {
            break;
        }
        let mut next = Vec::new();
        for i in 0..current.len() - 1 {
            next.push(current[i + 1] - current[i]);
        }
        layers.push(next);
    }

    let mut next = 0;
    for i in (0..layers.len()).rev() {
        let layer = &layers[i];
        next = next + layer.last().unwrap();
    }
    return next;
}
