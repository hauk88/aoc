// Declare struct with number and used
struct SymbolNumber {
    number: i32,
    used: bool,
    nonum: bool,
}

pub fn part1() {
    let contents = std::fs::read_to_string("shortp1.txt").expect("Error");

    let mut number_matrix: Vec<Vec<SymbolNumber>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<SymbolNumber> = Vec::new();
        let mut current_number = String::from("");
        for c in line.chars() {
            if c.is_numeric() {
                current_number.push(c);
            } else {
                if current_number.len() > 0 {
                    let symb_num = SymbolNumber {
                        number: current_number.parse::<i32>().unwrap(),
                        used: false,
                        nonum: false,
                    };
                    for i in 0..current_number.len() {
                        row.push(symb_num);
                    }
                }
                row.push(SymbolNumber {
                    number: 0,
                    used: false,
                    nonum: true,
                });
                current_number = String::from("");
            }
        }
        number_matrix.push(row);
    }

    for row in number_matrix {
        for symb_num in row {
            print!("{}", symb_num.number);
        }
        println!("");
    }
}
