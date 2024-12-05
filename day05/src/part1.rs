use std::fs;

struct Tuple {
    x: i32,
    y: i32,
}

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut pairs: Vec<Tuple> = Vec::new();
    let mut prints: Vec<Vec<i32>> = Vec::new();
    let mut pair_mode = true;
    for line in cont.lines() {
        if line.trim() == "" {
            pair_mode = false;
            continue;
        }
        if pair_mode {
            let nums = line
                .split("|")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            pairs.push(Tuple {
                x: nums[0],
                y: nums[1],
            });
        } else {
            let nums = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            prints.push(nums);
        }
    }

    let mut res = 0;
    for print in prints {
        let mut invalid = false;
        for i in 0..print.len() {
            let potential_pairs = pairs
                .iter()
                .filter(|pair| pair.y == print[i])
                .collect::<Vec<&Tuple>>();
            for j in i + 1..print.len() {
                invalid = potential_pairs.iter().any(|pair| pair.x == print[j]);
                if invalid {
                    break;
                }
            }
            if invalid {
                break;
            }
        }
        if !invalid {
            res += print[print.len() / 2];
        }
    }

    println!("{}", res);
}
