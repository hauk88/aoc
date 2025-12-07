use std::{collections::HashSet, fs};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();
    let mut splitters: HashSet<(i32, i32)> = HashSet::new();
    let mut start_pos = (0, 0);
    let mut n = 0;

    for (i, line) in cont.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                splitters.insert((i as i32, j as i32));
            }
            if c == 'S' {
                start_pos = (i as i32, j as i32);
            }
            n = i as i32 + 1;
        }
    }
    let mut beam_stack: HashSet<(i32, i32)> = HashSet::new();
    beam_stack.insert(start_pos);

    let mut split_count = 0;
    for _ in 0..n {
        let mut new_beam_stack: HashSet<(i32, i32)> = HashSet::new();
        for beam in beam_stack.iter() {
            let new_pos = (beam.0 + 1, beam.1);
            if splitters.contains(&new_pos) {
                new_beam_stack.insert((new_pos.0, new_pos.1 - 1));
                new_beam_stack.insert((new_pos.0, new_pos.1 + 1));
                split_count += 1;
            } else {
                new_beam_stack.insert(new_pos);
            }
        }
        beam_stack = new_beam_stack;
    }

    println!("{}", split_count);
}
