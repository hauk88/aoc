pub fn part1() {
    let content = std::fs::read_to_string("longp1.txt").unwrap();
    let mut platform = content
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    'O' => Tile::Bolder,
                    '.' => Tile::Empty,
                    '#' => Tile::Stopper,
                    _ => panic!("Unknown tile"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = platform.len();
    let m = platform[0].len();

    for j in 0..m {
        let mut stop_idx = -1;
        for i in 0..n {
            let t = &platform[i][j];
            let i32 = i as i32;
            match t {
                Tile::Stopper => {
                    stop_idx = i32;
                }
                Tile::Bolder => {
                    assert!(stop_idx < i32);
                    if i32 == stop_idx + 1 {
                        stop_idx = i32;
                    } else {
                        platform[i][j] = Tile::Empty;
                        platform[(stop_idx + 1) as usize][j] = Tile::Bolder;
                        stop_idx += 1;
                    }
                }
                _ => {}
            }
        }
    }
    let bolder_count = platform
        .iter()
        .map(|x| x.iter().filter(|y| **y == Tile::Bolder).count());
    let mut sum = 0;
    for (i, x) in bolder_count.enumerate() {
        sum += x * (n - i);
    }

    println!("sum: {}", sum);
}

#[derive(Debug, PartialEq)]
enum Tile {
    Bolder,
    Stopper,
    Empty,
}
