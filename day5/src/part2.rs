use std::collections::HashMap;

struct Interval {
    a: i64,
    b: i64,
}

struct Mapping {
    target: Interval,
    source: Interval,
}

impl Interval{
    fn 
}

pub fn part2() {
    let content = std::fs::read_to_string("longp1.txt").expect("error");

    let mut maps: HashMap<&str, Vec<Mapping>> = HashMap::new();
    let mut seeds: Vec<Interval> = Vec::new();

    let mut current_map_name = "";

    for line in content.lines() {
        if line.starts_with("seeds") {
            let seeds_str = line.split(":").collect::<Vec<&str>>()[1].trim();
            let seed_nums = seeds_str
                .split_white()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            for i in 0..seed_nums.len() / 2 {
                seeds.push(Interval {
                    a: seed_nums[2 * i],
                    b: seed_nums[2 * i + 1],
                });
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
        let mapping = Mapping {
            target: Interval {
                a: row[0],
                b: row[2],
            },
            source: Interval {
                a: row[1],
                b: row[2],
            },
        };
        maps.get_mut(current_map_name).unwrap().push(mapping);
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

    let mut locs: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut current_intervals = vec![seed];
        for map in map_stack.iter().rev() {
            let mappings = maps.get(map).unwrap();
            for mapping in mappings {


                if mapping[1] <= current_value && current_value <= mapping[1] + mapping[2] {
                    current_value = mapping[0] + current_value - mapping[1];
                    break;
                }
            }
        }
        locs.push(current_value);
    }

    println!("{:?}", locs.iter().min());
}
