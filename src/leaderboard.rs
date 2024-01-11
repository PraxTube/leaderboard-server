use std::fs::File;
use std::io::Write;

const FILE_PATH: &str = "/var/www/html/games/insta-kill/leaderboard.csv";

pub fn add_to_leaderboard(data_line: &str) {
    let mut file = match File::options().append(true).create(true).open(FILE_PATH) {
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
