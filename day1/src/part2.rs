pub fn part2() {
    let digits_list = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let contents =
        std::fs::read_to_string("longp1.txt").expect("Something went wrong reading the file");

    // loop over contents
    let mut sum = 0;
    for line in contents.lines() {
        // loop over each character
        let chars: Vec<_> = line.chars().collect();
        let mut num = String::from("");

        for i in 0..chars.len() {
            if chars[i].is_numeric() {
                num.push(chars[i]);
                break;
            }
            let line_rest: String = line.chars().skip(i).take(chars.len() - i).collect();
            let mut found = false;
            for j in 1..digits_list.len() + 1 {
                if line_rest.starts_with(digits_list[j - 1]) {
                    num.push_str(&j.to_string());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_numeric() {
                num.push(chars[i]);
                break;
            }
            let line_rest: String = line
                .chars()
                .rev()
                .skip(chars.len() - i - 1)
                .take(i)
                .collect();
            let lint_rev: String = line_rest.chars().rev().collect();
            // print!("line_rest: {}, lint_rev: {} ", line_rest, lint_rev);
            let mut found = false;
            for j in 1..digits_list.len() + 1 {
                if lint_rev.ends_with(digits_list[j - 1]) {
                    num.push_str(&j.to_string());
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        // println!("num: {}", num);
        sum += num.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum)
}
