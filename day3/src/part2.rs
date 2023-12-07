pub fn part2() {
    let mut cogs: std::collections::HashMap<String, Vec<i32>> = std::collections::HashMap::new();

    let contents = std::fs::read_to_string("shortp1.txt").expect("Error");
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
        let mut current_num = String::from("");
        for j in 0..input_matrix[i].len() {
            let c = input_matrix[i][j];
            if c.is_numeric() {
                current_num.push(c);
            } else {
                if current_num != "" {
                    let num_idx = j - current_num.len() - 1..j + 1;
                    let num = current_num.parse::<i32>().unwrap();

                    for ii in [i - 1, i, i + 1].iter() {
                        for jj in num_idx {
                            let c = input_matrix[*ii][jj];
                            if c == '*' {
                                let coord = format!("{} - {}", ii, jj);
                                if !cogs.contains_key(coord) {
                                    cogs.insert(coord, Vec::new());
                                }
                                cogs[&coord].push(num);
                            }
                        }
                    }

                    current_num = String::from("");
                }
            }
        }
    }
    println!("{}", sum);
}
