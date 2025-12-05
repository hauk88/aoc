use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut intervals: Vec<Interval> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();
    let mut mode = "intervalparsing";
    cont.lines().for_each(|line| {
        if line.trim().is_empty() {
            mode = "idparsing";
            return;
        }
        if mode == "intervalparsing" {
            let parts: Vec<&str> = line.split('-').collect();
            let interval = Interval {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            };

            intervals.push(interval);
        }
        if mode == "idparsing" {
            ids.push(line.parse::<i64>().unwrap());
        }
    });
    let count = ids
        .iter()
        .filter(|id| intervals.iter().any(|p| p.contains(**id)))
        .count();
    println!("{}", count);
}
#[derive(Debug)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn contains(&self, id: i64) -> bool {
        return self.start <= id && id <= self.end;
    }
}
