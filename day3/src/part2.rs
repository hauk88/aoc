pub fn part2() {
    let contents = std::fs::read_to_string("longp1.txt").expect("Error");
    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        row.push('.');
        for c in line.chars() {
            row.push(c);
        }
        row.push('.');
        input_matrix.push(row);
    }
    let mut first_row: Vec<char> = Vec::new();
    let mut last_row: Vec<char> = Vec::new();
    for _ in 0..input_matrix[0].len() {
        first_row.push('.');
        last_row.push('.');
    }
    input_matrix.insert(0, first_row);
    input_matrix.push(last_row);

    let mut sum = 0;
    for i in 0..input_matrix.len() {
        for j in 0..input_matrix[i].len() {
            let c = input_matrix[i][j];
            if c == '*' {
                let mut numbers: Vec<i32> = Vec::new();
                for ii in i - 1..i + 2 {
                    let row = &input_matrix[ii];
                    let mut col = j - 1;
                    while col < j + 2 {
                        let c = row[col];
                        if c.is_numeric() {
                            let (num, idx) = find_num(row, col);
                            println!("{} {}", num, idx);
                            numbers.push(num.parse::<i32>().unwrap());
                            col = idx + num.len();
                        } else {
                            col += 1;
                        }
                    }
                }
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }
    println!("{}", sum);
}

fn find_num(row: &Vec<char>, start_idx: usize) -> (String, usize) {
    let mut num = String::from("");
    // find first number
    let mut fist_idx = start_idx;
    for i in (0..start_idx).rev() {
        if !row[i].is_numeric() {
            break;
        }
        fist_idx = i;
    }

    for i in fist_idx..row.len() {
        if !row[i].is_numeric() {
            break;
        }
        num.push(row[i]);
    }
    return (num, fist_idx);
}
