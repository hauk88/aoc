use std::{cmp, fs};

#[derive(Debug)]
struct Game {
    button_a: (i32, i32),
    button_b: (i32, i32),
    target: (i32, i32),
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

pub fn part1() {
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

            game.target = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());
        } else if parts[0] == "Button A" {
            let xy_parts = parts[1].split(",").collect::<Vec<&str>>();
            let x_str = xy_parts[0].trim().split("+").collect::<Vec<&str>>()[1];
            let y_str = xy_parts[1].trim().split("+").collect::<Vec<&str>>()[1];

            game.button_a = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());
        } else if parts[0] == "Button B" {
            let xy_parts = parts[1].split(",").collect::<Vec<&str>>();
            let x_str = xy_parts[0].trim().split("+").collect::<Vec<&str>>()[1];
            let y_str = xy_parts[1].trim().split("+").collect::<Vec<&str>>()[1];

            game.button_b = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());
        }
    }
    games.push(game);

    let res: i32 = games
        .iter()
        .map(|game| tokens(game))
        .map(|res| match res {
            Some(token) => token,
            None => 0,
        })
        .sum();
    println!("{}", res);
}

fn tokens(game: &Game) -> Option<i32> {
    let mut token = i32::MAX;
    for a in 0..100 {
        for b in 0..100 {
            let x_a = game.button_a.0 * a;
            let y_a = game.button_a.1 * a;
            let x_b = game.button_b.0 * b;
            let y_b = game.button_b.1 * b;
            let x = x_a + x_b;
            let y = y_a + y_b;
            if x == game.target.0 && y == game.target.1 {
                token = cmp::min(3 * a + b, token);
            }
        }
    }
    if token == i32::MAX {
        return None;
    }
    return Some(token);
}
