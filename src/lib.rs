use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use tungstenite::{ Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMessageFormat {
    pub command: String,
    pub message: Option<String>,
}

#[derive(Serialize, Debug)]
struct SendMessage {
    command: String,
    message: String,
}

pub enum SocketCommands {
    FetchMessages,
    NewMessage,
    Nothing,
}



pub fn match_command_or_message(input: &str) -> SocketCommands {
    println!("Command got, {}", input);
    match input {
        "fetch_messages" => SocketCommands::FetchMessages,
        "new_message" => SocketCommands::NewMessage,
        _ => SocketCommands::Nothing,
    }
}


pub fn send_message(websocket: &mut WebSocket<TcpStream>, message: String) {

    let msg_to_send = serde_json::to_string(&SendMessage {
        command: String::from("new_message"),
        message: message,
    }).unwrap();

    let _ = websocket.send(Message::Text(msg_to_send)).map_err(|err| {
        eprintln!("cannot Send message, {}", err);
    });
}

pub fn fetch_messages(websocket: &mut WebSocket<TcpStream>, messages: &Vec<String>) {
    for msg in messages {
        send_message(websocket, msg.clone())
    }
}
