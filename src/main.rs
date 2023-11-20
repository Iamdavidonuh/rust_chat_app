use chat_app::{match_command_or_message, SocketMessageFormat};
use serde_json::{self};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
    thread::spawn,
};
use tungstenite::accept;

fn main() {
    let messages_db = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let messages_db = Arc::clone(&messages_db);
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let message = websocket.read().unwrap();
                // We do not want to send back ping/pong messages.
                if message.is_text() {
                    let messsage_txt = message.clone().to_string();
                    // serialize
                    let msg_json: SocketMessageFormat =
                        serde_json::from_str(&messsage_txt).unwrap();

                    let command = match_command_or_message(&msg_json.command);

                    command.execute(&mut websocket, &messages_db, msg_json)
                }
            }
        });
    }
}
