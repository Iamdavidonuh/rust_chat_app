use chat_app::{
    fetch_messages, match_command_or_message, send_message, SocketCommands, SocketMessageFormat,
};
use serde_json::{self};
use std::{io::Error, net::TcpListener, thread};
use tungstenite::accept;

fn main() {
    let mut messages: Vec<String> = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let mut websocket = accept(stream.unwrap()).unwrap();

        loop {
            let message = websocket.read().unwrap();
            // We do not want to send back ping/pong messages.
            if message.is_text() {
                println!("Client sent: {}", &message);

                let messsage_txt = message.clone().to_string();
                // serialize
                let msg_json: SocketMessageFormat = serde_json::from_str(&messsage_txt).unwrap();
                println!("Client json: {:?}", &msg_json);

                let command = match_command_or_message(&msg_json.command);

                match command {
                    SocketCommands::FetchMessages => fetch_messages(&mut websocket, &messages),
                    SocketCommands::NewMessage => match msg_json.message {
                        Some(socket_msg) => {
                            println!("msg.... {}", &socket_msg);
                            messages.push(socket_msg.clone());

                            send_message(&mut websocket, socket_msg);
                        }
                        None => {
                            continue;
                        }
                    },
                    SocketCommands::Nothing => continue,
                };
            }
        }
    }
}
