use std::fs::File;
use std::io::{self, BufRead, Write};

use crate::MAX_LEADERBOARD_ENTRIES;

const FILE_PATH: &str = "/var/www/html/games/insta-kill/leaderboard.csv";

struct LeaderboardEntry {
    name: String,
    score: u32,
    kills: u32,
    time: f32,
}

impl LeaderboardEntry {
    fn new(name: String, score: u32, kills: u32, time: f32) -> Self {
        Self {
            name,
            score,
            kills,
            time,
        }
    }

    fn to_string(&self) -> String {
        format!("{},{},{},{}", self.name, self.score, self.kills, self.time)
    }

    fn try_from(entry: &str) -> Option<LeaderboardEntry> {
        let parts: Vec<&str> = entry.split(',').map(str::trim).collect();
        match parts.as_slice() {
            [name, score, kills, time] => {
                let parsed_score = score.parse::<u32>();
                let parsed_kills = kills.parse::<u32>();
                let parsed_time = time.parse::<f32>();

                if let (Ok(score), Ok(kills), Ok(time)) = (parsed_score, parsed_kills, parsed_time)
                {
                    Some(LeaderboardEntry::new(name.to_string(), score, kills, time))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn insert_sorted(vec: &mut Vec<LeaderboardEntry>, new_element: LeaderboardEntry) {
    let key = new_element.score;

    match vec.binary_search_by_key(&key, |entry| entry.score) {
        Ok(index) | Err(index) => {
            vec.insert(index, new_element);
        }
    }
}

fn sorted_leaderboard() -> Vec<LeaderboardEntry> {
    let file = match File::open(FILE_PATH) {
        Ok(r) => r,
        Err(err) => {
            println!("ERROR while trying to fetch file, {}", err);
            return Vec::new();
        }
    };
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    let mut board: Vec<LeaderboardEntry> = line
        .split(';')
        .filter_map(|entry| LeaderboardEntry::try_from(&entry))
        .collect();
    board.pop();

    board.sort_by(|a, b| a.score.cmp(&b.score));
    board
}

pub fn add_to_leaderboard(data_line: &str) {
    let entry = match LeaderboardEntry::try_from(data_line) {
        Some(r) => r,
        None => return,
    };
    let mut leaderboard = sorted_leaderboard();
    insert_sorted(&mut leaderboard, entry);
    leaderboard.reverse();
    leaderboard.truncate(MAX_LEADERBOARD_ENTRIES);

    let data: Vec<String> = leaderboard.iter().map(|l| l.to_string()).collect();
    let data_str = data.join(";") + ";";

    let mut file = match File::options().write(true).create(true).open(FILE_PATH) {
        Ok(r) => r,
        Err(err) => {
            println!(
                "There was an error when opening the leaderboard file, {}",
                err
            );
            return;
        }
    };

    match file.write(data_str.as_bytes()) {
        Ok(_) => {}
        Err(err) => println!(
            "There was an error when writing to the leaderboard file, {}",
            err
        ),
    };
}
