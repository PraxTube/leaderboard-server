mod leaderboard;

use bytes::Bytes;
use warp::Filter;

use crate::leaderboard::add_to_leaderboard;

fn add_leaderboard_response(body: String) -> String {
    if body.chars().filter(|&c| c == ',').count() == 3 {
        add_to_leaderboard(&body);
        return "HTTP/1.1 200 OK\r\n\r\nData received successfully".to_string();
    }

    println!("ERROR, POST request doesn't match, {}", body);
    "HTTP/1.1 400 Bad Request\r\n\r\nGiven leaderboard entry is not correct".to_string()
}

#[tokio::main]
async fn main() {
    let request = warp::post()
        .and(warp::path("board"))
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::bytes())
        .map(|bytes: Bytes| {
            let s = match String::from_utf8(bytes.to_vec()) {
                Ok(r) => r,
                Err(_) => return warp::reply(),
            };
            add_leaderboard_response(s);
            warp::reply()
        });

    warp::serve(request).run(([0, 0, 0, 0], 3434)).await
}
