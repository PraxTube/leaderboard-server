use std::fs::File;
use std::io::{self, BufRead, Write};

const FILE: &str = "leaderboard.csv";

fn get_leaderboard() -> Vec<(String, u32, String, String)> {
    let file = match File::open(FILE) {
        Ok(r) => r,
        Err(err) => {
            println!("Error while trying to fetch file, {}", err);
            return Vec::new();
        }
    };
    let reader = io::BufReader::new(file);

    let mut board: Vec<(String, u32, String, String)> = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|l| {
                let parts: Vec<&str> = l.split(',').map(str::trim).collect();
                match parts.as_slice() {
                    [key, score, kills, time] => score
                        .parse()
                        .ok()
                        .map(|v: u32| (key.to_string(), v, kills.to_string(), time.to_string())),
                    _ => None,
                }
            })
        })
        .collect();
    board.sort_by(|a, b| a.1.cmp(&b.1));
    board.reverse();
    board
}

pub fn leaderboard_http() -> String {
    let board = get_leaderboard();

    let mut result = String::new();

    for (name, score, kills, time) in board {
        result.push_str(&format!("{},{},{},{};", name, score, kills, time));
    }

    result.trim_end_matches(";").to_string()
}

pub fn add_to_leaderboard(data_line: &str) {
    let mut file = match File::options().append(true).create(true).open(FILE) {
        Ok(r) => r,
        Err(err) => {
            println!(
                "There was an error when opening the leaderboard file, {}",
                err
            );
            return;
        }
    };

    let data = format!("{}\n", data_line);
    match file.write(data.as_bytes()) {
        Ok(_) => {}
        Err(err) => println!(
            "There was an error when writing to the leaderboard file, {}",
            err
        ),
    };
}
