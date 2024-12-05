use std::{collections::HashMap, fs};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
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
            rules.entry(nums[0]).or_insert(Vec::new()).push(nums[1]);
        } else {
            let nums = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            prints.push(nums);
        }
    }

    let mut res = 0;
    let mut invalid_prints: Vec<Vec<i32>> = Vec::new();
    for print in prints {
        let mut invalid = false;
        for i in (0..print.len()).rev() {
            let empty = Vec::new();
            let should_be_behind = rules.get(&print[i]).unwrap_or(&empty);
            for j in (0..i + 1).rev() {
                invalid = should_be_behind.iter().any(|x| x == &print[j]);
                if invalid {
                    break;
                }
            }
            if invalid {
                break;
            }
        }
        if invalid {
            invalid_prints.push(print.clone());
        }
    }

    for print in invalid_prints {
        let mut mut_print = print.clone();
        mut_print.sort_by(|a, b| {
            let empty = Vec::new();
            let a_rules = rules.get(a).unwrap_or(&empty);
            if a_rules.contains(b) {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
        res += mut_print[mut_print.len() / 2];
    }

    println!("{}", res);
}
