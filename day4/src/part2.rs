pub fn part2() {
    let content = std::fs::read_to_string("longp1.txt").expect("error");
    let mut number_of_cards: Vec<usize> = Vec::new();

    for (idx, line) in content.lines().enumerate() {
        if idx >= number_of_cards.len() {
            number_of_cards.push(1);
        }

        let card = line.split(":").collect::<Vec<&str>>()[1];
        let ca = card.split("|").collect::<Vec<&str>>();

        let ca_nums = ca[0].trim().split(" ").collect::<Vec<&str>>();
        let winning_nums = ca[1].trim().split(" ").collect::<Vec<&str>>();

        let mut number_of_wins: usize = 0;
        for num in ca_nums {
            if num == "" {
                continue;
            }
            if winning_nums.contains(&num) {
                number_of_wins += 1;
            }
        }

        let start = idx + 1;
        let end = idx + number_of_wins + 1;

        if number_of_wins == 0 {
            continue;
        }

        for c_idx in start..end {
            if c_idx >= number_of_cards.len() {
                number_of_cards.push(1);
            }

            number_of_cards[c_idx] += number_of_cards[idx];
        }
    }
    // sum the values in number_of_cards.
    let sum = number_of_cards.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);
}
