#[derive(Debug)]
struct Registry {
    a: u32,
    b: u32,
    c: u32,
}

impl Registry {
    fn new() -> Registry {
        Registry { a: 0, b: 0, c: 0 }
    }
}

pub fn part1() {
    let content = std::fs::read_to_string("largep1.txt").expect("Failed to read the file");

    let mut r = Registry::new();
    let mut program: Vec<u8> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts[0] == "Register A" {
            r.a = parts[1].trim().parse().unwrap();
        } else if parts[0] == "Register B" {
            r.b = parts[1].trim().parse().unwrap();
        } else if parts[0] == "Register C" {
            r.c = parts[1].trim().parse().unwrap();
        } else if parts[0] == "Program" {
            parts[1]
                .trim()
                .split(",")
                .for_each(|x| program.push(x.trim().parse().unwrap()));
        }
    }

    let mut i: usize = 0;
    let mut output: Vec<u32> = Vec::new();
    loop {
        if i >= program.len() - 1 {
            break;
        }
        let mut new_i = i + 2;
        let opcode = program[i];
        let operand = program[i + 1];
        match opcode {
            0 => {
                let op = get_combo_operand(&r, operand);
                let base: u32 = 2;
                r.a = r.a / base.pow(op);
            }
            1 => {
                let op = operand as u32;
                r.b = r.b ^ op;
            }
            2 => {
                let op = get_combo_operand(&r, operand);
                r.b = op % 8;
            }
            3 => {
                if r.a != 0 {
                    new_i = operand as usize;
                }
            }
            4 => {
                r.b = r.c ^ r.b;
            }
            5 => {
                let op = get_combo_operand(&r, operand);
                let res = op % 8;
                output.push(res);
                print!("{},", res);
            }
            6 => {
                let op = get_combo_operand(&r, operand);
                let base: u32 = 2;
                r.b = r.a / base.pow(op);
            }
            7 => {
                let op = get_combo_operand(&r, operand);
                let base: u32 = 2;
                r.c = r.a / base.pow(op);
            }
            _ => panic!("Unknown opcode"),
        }

        i = new_i;
    }
    println!();
}

fn get_combo_operand(r: &Registry, operand: u8) -> u32 {
    match operand {
        0 | 1 | 2 | 3 => operand as u32,
        4 => r.a,
        5 => r.b,
        6 => r.c,
        _ => panic!("Unknown operand"),
    }
}
