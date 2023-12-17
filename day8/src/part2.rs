use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

pub fn part2() {
    // read input
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let instructions = lines[0]
        .trim()
        .chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect::<Vec<_>>();
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    for i in 2..lines.len() {
        let line = lines[i];
        let parts = line.split("=").collect::<Vec<_>>();
        let key = parts[0].trim().to_string();
        let a = parts[1];
        let b = a.trim();
        let c = b.replace("(", "");
        let d = c.replace(")", "");

        let e = d.split(",");
        let f = e.map(|s| s.trim().to_string()).collect::<Vec<_>>();
        mapping.insert(key, f);
    }

    let mut i = 0;
    let mut sum = 0;
    let mut current_v = mapping
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|s| s.clone())
        .collect::<Vec<_>>();

    println!("{:?}", current_v);

    loop {
        let instruction = &instructions[i];

        for j in 0..current_v.len() {
            let current = &current_v[j];
            let current_mapping = mapping.get(current).unwrap();
            match instruction {
                Direction::Left => {
                    current_v[j] = current_mapping[0].clone();
                }
                Direction::Right => {
                    current_v[j] = current_mapping[1].clone();
                }
            }
        }

        // println!("{:?}", current_v);

        sum += 1;
        let c = current_v.iter().filter(|x| x.ends_with("Z")).count();
        println!("{:?}", c);
        if current_v.iter().all(|x| x.ends_with("Z")) {
            break;
        }

        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    println!("{:?}", sum)
}
