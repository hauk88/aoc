use std::collections::HashMap;

pub fn part2() {
    let content = std::fs::read_to_string("longp1.txt").expect("error");

    let mut maps: HashMap<&str, Vec<Vec<i64>>> = HashMap::new();
    let mut seeds: Vec<i64> = Vec::new();

    let mut current_map_name = "";

    for line in content.lines() {
        if line.starts_with("seeds") {
            let seeds_str = line.split(":").collect::<Vec<&str>>()[1].trim();
            for seed in seeds_str.split(" ") {
                if seed == "" {
                    continue;
                }
                seeds.push(seed.trim().parse::<i64>().unwrap());
            }
            continue;
        }
        if line.contains("map") {
            current_map_name = line.trim().split(" ").collect::<Vec<&str>>()[0];
            maps.insert(current_map_name, Vec::new());
            continue;
        }
        if line.trim() == "" {
            continue;
        }

        let row = line
            .trim()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        maps.get_mut(current_map_name).unwrap().push(row);
    }

    let mut map_stack: Vec<&str> = Vec::new();
    let mut target = "location";
    let start = "seed";

    loop {
        let tm = maps.keys().find(|x| x.ends_with(target)).unwrap();
        target = tm.split("-").collect::<Vec<&str>>()[0];
        map_stack.push(tm);
        if target == start {
            break;
        }
    }

    let mut location = 0;
    loop {
        if location % 100000 == 0 {
            println!("{}", location);
        }

        let mut current_value = location;
        for map in map_stack.iter() {
            let mappings = maps.get(map).unwrap();
            for mapping in mappings {
                if mapping[0] <= current_value && current_value <= mapping[0] + mapping[2] {
                    current_value = mapping[1] + current_value - mapping[0];
                    break;
                }
            }
        }
        let mut seed_found = false;
        for seed_range in seeds.chunks(2) {
            if seed_range[0] <= current_value && current_value <= seed_range[0] + seed_range[1] {
                seed_found = true;
                break;
            }
        }
        if seed_found {
            break;
        }
        location += 1;
    }

    println!("Res: {:?}", location)
}
