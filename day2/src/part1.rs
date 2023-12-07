use std::collections::HashMap;
pub fn part1() {
    // create hash map of colors to frequencies in i32
    let mut freqs: HashMap<&str, i32> = HashMap::new();
    freqs.insert("red", 12);
    freqs.insert("green", 13);
    freqs.insert("blue", 14);

    let contents =
        std::fs::read_to_string("longp1.txt").expect("Something went wrong reading the file");

    // loop over contents
    let mut sum = 0;
    for line in contents.lines() {
        // split on :
        let gameidstr = line.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1];
        let gameid = gameidstr.parse::<i32>().unwrap();

        let draws = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        let mut game_broken = false;
        for draw in draws {
            let colorfreqs = draw.split(",");
            for colorfreq in colorfreqs {
                let color = colorfreq.trim().split(" ").collect::<Vec<&str>>()[1];
                let freq = colorfreq.trim().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap();

                if freqs.contains_key(color) {
                    let maxfreq = freqs.get(color).unwrap();
                    if freq > *maxfreq {
                        game_broken = true;
                        break;
                    }
                }
            }
            if game_broken {
                break;
            }
        }

        if !game_broken {
            sum += gameid;
        }
    }
    println!("sum: {}", sum)
}
