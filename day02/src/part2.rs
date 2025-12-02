use std::fs;

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut count = 0;
    cont.split(",").for_each(|x| {
        let id_split = x.split("-").collect::<Vec<&str>>();
        let start_id = id_split[0];
        let start_num = start_id.parse::<i64>().unwrap();
        let end_id = id_split[1];
        let end_num = end_id.parse::<i64>().unwrap();

        for num in start_num..=end_num {
            if !is_valid(num) {
                count += num;
            }
        }
    });
    println!("{}", count);
}

fn is_valid(num: i64) -> bool {
    let num_str = num.to_string();
    let num_chars = num_str.chars().collect::<Vec<char>>();
    let n = num_chars.len();
    if n == 1 {
        return true;
    }
    let mut possible_chunk_size: Vec<usize> = Vec::new();
    possible_chunk_size.push(1);
    for i in 2..n / 2 + 1 {
        let p = n / i;
        if p * i == n {
            possible_chunk_size.push(i);
        }
    }

    for chunk_size in possible_chunk_size {
        let parts: Vec<String> = num_chars
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();

        let first_part = &parts[0];

        let invalid = parts.iter().all(|x| x == first_part);
        if invalid {
            return false;
        }
    }

    return true;
}
