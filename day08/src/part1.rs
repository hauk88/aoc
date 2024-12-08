use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut freq_to_antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut n = 0;
    let mut m = 0;
    cont.lines().enumerate().for_each(|(i, line)| {
        n = i as i32;
        m = 0;
        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                freq_to_antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
            m = j as i32;
        });
    });
    n += 1;
    m += 1;

    let mut antinode: HashSet<(i32, i32)> = HashSet::new();
    for (_, antennas) in freq_to_antennas.iter() {
        find_antinodes(antennas, n, m, &mut antinode);
    }

    println!("{}", antinode.len());
}

fn find_antinodes(
    antennas: &Vec<(i32, i32)>,
    n: i32,
    m: i32,
    antinode_set: &mut HashSet<(i32, i32)>,
) {
    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j {
                continue;
            }
            let res1 = intrp_2d(antennas[i], antennas[j], 2);
            let res2 = intrp_2d(antennas[i], antennas[j], -1);
            if res1.0 >= 0 && res1.1 >= 0 && res1.0 < n && res1.1 < m {
                antinode_set.insert(res1);
            }
            if res2.0 >= 0 && res2.1 >= 0 && res2.0 < n && res2.1 < m {
                antinode_set.insert(res2);
            }
        }
    }
}

fn intrp_2d(p1: (i32, i32), p2: (i32, i32), t: i32) -> (i32, i32) {
    return (intrp_1d(p1.0, p2.0, t), intrp_1d(p1.1, p2.1, t));
}

fn intrp_1d(x1: i32, x2: i32, t: i32) -> i32 {
    return x1 * (1 - t) + x2 * t;
}
