use std::{env, fs};
use rouille::Request;
use rouille::Response;

fn main() {
    let in_file = env::var("ENSHROUDED_PLAYER_COUNT_FILE")
        .unwrap_or_else(|_e| "./enshrouded-player-count.txt".to_string());

    let port = env::var("ENSHROUDED_PLAYER_COUNT_SERVER_PORT")
        .unwrap_or_else(|_e| "8080".to_string());


    rouille::start_server(format!("{}{}", "0.0.0.0:", port), move |_request| {
        let player_count = fs::read_to_string(&in_file).expect("Unable to read file");
        Response::text(player_count)
    })
}
