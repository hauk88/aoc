pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let lines = content.lines();

    let mut sum = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let records: Vec<_> = parts[0].chars().collect();
        let groupings = parts[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        sum += find_all_arrangements(records, groupings);
    }
    println!("{}", sum);
}

fn find_all_arrangements(records: Vec<char>, groupings: Vec<i32>) -> i32 {
    let unknowns: Vec<usize> = records
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == '?')
        .map(|(i, _)| i)
        .collect();

    let mut arrangements = 0;
    let mut r = records.clone();
    for i in 0..2_i32.pow(unknowns.len() as u32) {
        for j in 0..unknowns.len() {
            let bit = (i >> j) & 1;
            r[unknowns[j]] = if bit == 0 { '.' } else { '#' };
        }
        if check_arrangement(&r, &groupings) {
            arrangements += 1;
        }
    }
    return arrangements;
}

fn check_arrangement(records: &Vec<char>, groupings: &Vec<i32>) -> bool {
    let mut groups: Vec<i32> = Vec::new();
    let mut idx = 0;
    for i in 0..records.len() {
        if records[i] == '#' {
            if groups.len() == idx {
                groups.push(0);
            }
            groups[idx] += 1;
        } else {
            idx = groups.len();
        }
    }
    if groups.len() != groupings.len() {
        return false;
    }
    for i in 0..groups.len() {
        if groups[i] != groupings[i] {
            return false;
        }
    }
    return true;
}
