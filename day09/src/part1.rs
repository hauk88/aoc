use std::fs;

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let cords: Vec<Vec2d> = cont
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            Vec2d {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect();

    let mut areas: Vec<i64> = Vec::new();
    let n = cords.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let area = cords[i].area(&cords[j]);
            areas.push(area);
        }
    }
    areas.sort();
    println!("{}", areas.last().unwrap());
}

#[derive(Debug)]
struct Vec2d {
    x: i64,
    y: i64,
}

impl Vec2d {
    fn area(&self, other: &Vec2d) -> i64 {
        let dx = (self.x - other.x).abs() + 1;
        let dy = (self.y - other.y).abs() + 1;
        let res = dx * dy;
        return res;
    }
}
