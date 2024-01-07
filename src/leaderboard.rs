use std::fs::File;
use std::io::{self, BufRead};

const FILE: &str = "leaderboard.csv";

fn get_leader_board() -> Vec<(String, u32)> {
    let file = match File::open(FILE) {
        Ok(r) => r,
        Err(err) => {
            println!("ERROR while trying to fetch file, {}", err);
            return Vec::new();
        }
    };
    let reader = io::BufReader::new(file);

    let mut board: Vec<(String, u32)> = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|l| {
                let parts: Vec<&str> = l.split(',').map(str::trim).collect();
                match parts.as_slice() {
                    [key, value] => value.parse().ok().map(|v: u32| (key.to_string(), v)),
                    _ => None,
                }
            })
        })
        .collect();
    board.sort_by(|a, b| a.1.cmp(&b.1));
    board.reverse();
    board
}

pub fn leader_board_http() -> String {
    let board = get_leader_board();

    let mut result = String::new();

    for (name, score) in board {
        result.push_str(&format!("{},{};", name, score));
    }

    result
}
