pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").expect("error");

    let mut sum = 0;
    for line in content.lines() {
        let card = line.split(":").collect::<Vec<&str>>()[1];
        let ca = card.split("|").collect::<Vec<&str>>();

        let ca_nums = ca[0].trim().split(" ").collect::<Vec<&str>>();
        let winning_nums = ca[1].trim().split(" ").collect::<Vec<&str>>();

        let mut number_of_wins = 0;
        for num in ca_nums {
            if num == "" {
                continue;
            }
            if winning_nums.contains(&num) {
                number_of_wins += 1;
            }
        }

        if number_of_wins != 0 {
            sum += 2_i32.pow(number_of_wins - 1);
        }
    }
    println!("{}", sum);
}
