use std::fs;

pub fn part1() {
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
    let n = num_str.len();
    if n % 2 != 0 {
        return true;
    }

    let (p1, p2) = num_str.split_at(n / 2);
    return p1 != p2;
}
