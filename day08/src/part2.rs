use std::{collections::HashSet, fs};

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let points: Vec<Vec3d> = cont
        .lines()
        .map(|line| {
            let coords: Vec<f32> = line.split(',').map(|s| s.parse::<f32>().unwrap()).collect();
            Vec3d::new(coords[0], coords[1], coords[2])
        })
        .collect();

    let mut distances: Vec<(usize, usize, f32)> = Vec::new();
    let n = points.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].sq_dist(&points[j]);
            distances.push((i, j, dist));
        }
    }

    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut clusters: Vec<HashSet<usize>> = Vec::new();
    let mut res: f32 = 0.0;

    for (i, j, _) in distances.iter() {
        let mut i_cluster_idx: Option<usize> = None;
        let mut j_cluster_idx: Option<usize> = None;
        clusters.iter().enumerate().for_each(|(idx, c)| {
            if c.contains(&i) {
                i_cluster_idx = Some(idx);
            }
            if c.contains(&j) {
                j_cluster_idx = Some(idx);
            }
        });

        match (i_cluster_idx, j_cluster_idx) {
            (Some(i_idx), Some(j_idx)) => {
                if i_idx != j_idx {
                    let keep_idx = i_idx.min(j_idx);
                    let remove_idx = i_idx.max(j_idx);

                    let removed_cluster = clusters.remove(remove_idx);
                    clusters[keep_idx].extend(removed_cluster);
                }
            }
            (None, None) => {
                let mut new_cluster: HashSet<usize> = HashSet::new();
                new_cluster.insert(*i);
                new_cluster.insert(*j);
                clusters.push(new_cluster);
            }
            (Some(i_idx), None) => {
                clusters[i_idx].insert(*j);
            }
            (None, Some(j_idx)) => {
                clusters[j_idx].insert(*i);
            }
        }

        if clusters.len() == 1 && clusters[0].len() == n {
            let p1 = &points[*i];
            let p2 = &points[*j];

            res = p1.x * p2.x;
            break;
        }
    }

    println!("part2: {}", res);
}

#[derive(Debug)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3d {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3d { x, y, z }
    }

    fn sq_dist(&self, other: &Vec3d) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        return dx * dx + dy * dy + dz * dz;
    }
}
