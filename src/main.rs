mod leaderboard;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::leaderboard::leader_board_http;

fn add_to_leaderboard(request: String) -> String {
    if let Some(start) = request.find("\r\n\r\n") {
        let body = &request[start + 4..];
        let lines: Vec<&str> = body.split("\r\n").collect();

        if let Some(data_line) = lines.last() {
            let data: Vec<&str> = data_line.split(",").collect();

            if data.len() == 2 {
                return "HTTP/1.1 200 OK\r\n\r\nData received successfully".to_string();
            }
        }
    }
    println!("ERROR, POST request doesn't match");
    "HTTP/1.1 400 Bad Request\r\n\r\nGiven leaderboard entry is not correct".to_string()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    let response = if request.starts_with("POST") {
        add_to_leaderboard(request.to_string())
    } else if request.starts_with("GET") {
        leader_board_http()
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\nOnly POST and GET requests are supported".to_string()
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
