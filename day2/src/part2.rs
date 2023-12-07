use std::collections::HashMap;
pub fn part2() {
    let contents =
        std::fs::read_to_string("longp1.txt").expect("Something went wrong reading the file");

    // loop over contents
    let mut sum = 0;
    for line in contents.lines() {
        let mut color_to_maxfreq: HashMap<&str, i32> = HashMap::new();

        let draws = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        for draw in draws {
            let colorfreqs = draw.split(",");
            for colorfreq in colorfreqs {
                let color = colorfreq.trim().split(" ").collect::<Vec<&str>>()[1];
                let freq = colorfreq.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap();

                if color_to_maxfreq.contains_key(color) {
                    let maxfreq = color_to_maxfreq.get(color).unwrap();
                    if freq > *maxfreq {
                        color_to_maxfreq.insert(color, freq);
                    }
                } else {
                    color_to_maxfreq.insert(color, freq);
                }
            }
        }

        let mut pwr = 1;
        color_to_maxfreq.iter().for_each(|(_, v)| pwr *= v);
        sum += pwr;
    }
    println!("sum: {}", sum)
}
