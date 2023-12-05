fn main() {
    println!("Hello, world!");
    // read short.txt
    let contents =
        std::fs::read_to_string("long.txt").expect("Something went wrong reading the file");

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
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_numeric() {
                num.push(chars[i]);
                break;
            }
        }

        sum += num.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum)
}
