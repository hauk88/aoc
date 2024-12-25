use std::fs;

#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    target: (i64, i64),
}

impl Game {
    fn new() -> Game {
        Game {
            button_a: (1, 1),
            button_b: (1, 1),
            target: (1, 1),
        }
    }
}

pub fn part2() {
    let cont = fs::read_to_string("largep1.txt").unwrap();

    let mut games: Vec<Game> = Vec::new();
    let mut game = Game::new();
    for line in cont.lines() {
        if !line.contains(":") {
            games.push(game);
            game = Game::new();
        }
        let parts = line.split(":").collect::<Vec<&str>>();
        if parts[0] == "Prize" {
            let xy_parts = parts[1].split(",").collect::<Vec<&str>>();
            let x_str = xy_parts[0].trim().split("=").collect::<Vec<&str>>()[1];
            let y_str = xy_parts[1].trim().split("=").collect::<Vec<&str>>()[1];

            game.target = (
                x_str.parse::<i64>().unwrap() + 10000000000000,
                y_str.parse::<i64>().unwrap() + 10000000000000,
            );
        } else if parts[0] == "Button A" {
            let xy_parts = parts[1].split(",").collect::<Vec<&str>>();
            let x_str = xy_parts[0].trim().split("+").collect::<Vec<&str>>()[1];
            let y_str = xy_parts[1].trim().split("+").collect::<Vec<&str>>()[1];

            game.button_a = (x_str.parse::<i64>().unwrap(), y_str.parse::<i64>().unwrap());
        } else if parts[0] == "Button B" {
            let xy_parts = parts[1].split(",").collect::<Vec<&str>>();
            let x_str = xy_parts[0].trim().split("+").collect::<Vec<&str>>()[1];
            let y_str = xy_parts[1].trim().split("+").collect::<Vec<&str>>()[1];

            game.button_b = (x_str.parse::<i64>().unwrap(), y_str.parse::<i64>().unwrap());
        }
    }
    games.push(game);

    let res: i64 = games
        .iter()
        .map(|game| {
            let t = tokens(game);
            match t {
                Some(token) => token,
                None => 0,
            }
        })
        .sum();
    println!("{}", res);
}

// Ax = b
// [x00, x01;x10,x11]*[a,b] = [c0,c1]

// a*x00 + b*x01 = c0
// a*x10 + b*x11 = c1

// a*x10 - a*x00*x10/x00 + b*x11 - b*x01*x10/x00 = c1 - c0*x10/x00
// b = (c1 - c0*x10/x00) / (x11 - x01*x10/x00)
// a = (c0 - b*x01) / x00

fn tokens(game: &Game) -> Option<i64> {
    let x00 = game.button_a.0 as f64;
    let x01 = game.button_b.0 as f64;
    let x10 = game.button_a.1 as f64;
    let x11 = game.button_b.1 as f64;
    let c0 = game.target.0 as f64;
    let c1 = game.target.1 as f64;

    let b = (c1 - c0 * x10 / x00) / (x11 - x01 * x10 / x00);
    let a = (c0 - b * x01) / x00;

    // Mess with percision until I got the right answer
    if (b.round() - b).abs() > 0.01 || (a.round() - a).abs() > 0.01 {
        return None;
    }
    let a_res = a.round() as i64;
    let b_res = b.round() as i64;

    return Some(3 * a_res + b_res);
}
