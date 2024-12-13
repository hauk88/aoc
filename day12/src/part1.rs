use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut regions: HashMap<char, Vec<HashSet<(i32, i32)>>> = HashMap::new();
    for (i, line) in cont.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let region_set = regions.entry(c).or_insert(Vec::new());
            // find all connected regions
            let matching_indices: Vec<usize> = region_set
                .iter()
                .enumerate()
                .filter_map(|(idx, region)| {
                    let up = (i as i32 - 1, j as i32);
                    let down = (i as i32 + 1, j as i32);
                    let left = (i as i32, j as i32 - 1);
                    let right = (i as i32, j as i32 + 1);
                    if region.contains(&up)
                        || region.contains(&down)
                        || region.contains(&left)
                        || region.contains(&right)
                    {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();
            let mut new_region = HashSet::new();
            new_region.insert((i as i32, j as i32));
            if matching_indices.is_empty() {
                region_set.push(new_region);
            } else {
                for &idx in matching_indices.iter().rev() {
                    new_region.extend(region_set[idx].drain());
                    region_set.remove(idx);
                }
                region_set.push(new_region);
            }
        }
    }

    let mut res: i32 = 0;
    for (_, regions) in regions.iter() {
        for region in regions.iter() {
            let r_fence = get_region_fences(region);
            res += r_fence * region.len() as i32;
        }
    }
    println!("{}", res);
}

fn get_region_fences(region: &HashSet<(i32, i32)>) -> i32 {
    let mut fences = 0;
    for (i, j) in region.iter() {
        let up = (i - 1, *j);
        let down = (i + 1, *j);
        let left = (*i, j - 1);
        let right = (*i, j + 1);
        let dirs = vec![up, down, left, right];
        for dir in dirs.iter() {
            if !region.contains(dir) {
                fences += 1;
            }
        }
    }
    return fences;
}
