pub fn part1() {
    let contents = std::fs::read_to_string("longp1.txt").expect("Error");
    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    let mut near_symbol_matrix: Vec<Vec<bool>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        let mut near_symbol_row: Vec<bool> = Vec::new();
        near_symbol_row.push(false);
        for c in line.chars() {
            row.push(c);
            near_symbol_row.push(false);
        }
        near_symbol_row.push(false);
        near_symbol_matrix.push(near_symbol_row);
        input_matrix.push(row);
    }
    let mut first_row: Vec<bool> = Vec::new();
    let mut last_row: Vec<bool> = Vec::new();
    for _ in 0..input_matrix[0].len() + 2 {
        first_row.push(false);
        last_row.push(false);
    }
    near_symbol_matrix.insert(0, first_row);
    near_symbol_matrix.push(last_row);

    for i in 0..input_matrix.len() {
        for j in 0..input_matrix[i].len() {
            let c = input_matrix[i][j];
            let i_s = i + 1;
            let j_s = j + 1;
            if !c.is_numeric() && c != '.' {
                near_symbol_matrix[i_s - 1][j_s - 1] = true;
                near_symbol_matrix[i_s - 1][j_s] = true;
                near_symbol_matrix[i_s - 1][j_s + 1] = true;
                near_symbol_matrix[i_s][j_s - 1] = true;
                near_symbol_matrix[i_s][j_s + 1] = true;
                near_symbol_matrix[i_s + 1][j_s - 1] = true;
                near_symbol_matrix[i_s + 1][j_s] = true;
                near_symbol_matrix[i_s + 1][j_s + 1] = true;
            }
        }
    }

    // println!("{:?}", near_symbol_matrix);

    let mut sum = 0;
    for i in 0..input_matrix.len() {
        let mut current_num = String::from("");
        for j in 0..input_matrix[i].len() {
            let c = input_matrix[i][j];
            if c.is_numeric() {
                current_num.push(c);
            }
            if j == input_matrix[i].len() - 1 || !c.is_numeric() {
                if current_num != "" {
                    let num_idx = j - current_num.len()..j;

                    let mut is_near_symbol = false;
                    for idx in num_idx {
                        if near_symbol_matrix[i + 1][idx + 1] {
                            is_near_symbol = true;
                            break;
                        }
                    }
                    if is_near_symbol {
                        let num = current_num.parse::<i32>().unwrap();
                        sum += num;
                    }
                    current_num = String::from("");
                }
            }
        }
    }
    println!("{}", sum);
}
