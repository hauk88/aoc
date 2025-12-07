use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut splitters: HashSet<(i64, i64)> = HashSet::new();
    let mut start_pos = (0, 0);
    let mut n = 0;

    for (i, line) in cont.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                splitters.insert((i as i64, j as i64));
            }
            if c == 'S' {
                start_pos = (i as i64, j as i64);
            }
            n = i as i64 + 1;
        }
    }

    let mut beam_stack: HashMap<(i64, i64), i64> = HashMap::new();
    beam_stack.insert(start_pos, 1);
    for _ in 0..n {
        let mut new_beam_stack: HashMap<(i64, i64), i64> = HashMap::new();
        for (beam, count) in beam_stack.iter() {
            let mut new_positions: Vec<(i64, i64)> = Vec::new();

            let potential_new_pos = (beam.0 + 1, beam.1);
            if splitters.contains(&potential_new_pos) {
                new_positions.push((potential_new_pos.0, potential_new_pos.1 - 1));
                new_positions.push((potential_new_pos.0, potential_new_pos.1 + 1));
            } else {
                new_positions.push(potential_new_pos);
            }

            for pos in new_positions {
                new_beam_stack
                    .entry(pos)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        beam_stack = new_beam_stack;
    }

    println!("{}", beam_stack.values().sum::<i64>());
}
