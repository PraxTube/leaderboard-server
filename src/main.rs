mod leaderboard;

use warp::Filter;

use crate::leaderboard::add_to_leaderboard;

fn add_leaderboard_response(name: String, score: u32, kills: u32, time: f32) -> String {
    let body = format!("{},{},{},{}", name, score, kills, time);
    add_to_leaderboard(&body);
    "HTTP/1.1 200 OK\r\n\r\nData received successfully".to_string()
}

#[tokio::main]
async fn main() {
    let request = warp::post()
        .and(warp::path("leaderboard"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u32>())
        .and(warp::path::param::<u32>())
        .and(warp::path::param::<f32>())
        .map(|name, score, kills, time| {
            add_leaderboard_response(name, score, kills, time);
            warp::reply()
        });

    warp::serve(request)
        .tls()
        .cert_path("/etc/letsencrypt/live/rancic.org/fullchain.pem")
        .key_path("/etc/letsencrypt/live/rancic.org/privkey.pem")
        .run(([0, 0, 0, 0], 3434))
        .await
}
