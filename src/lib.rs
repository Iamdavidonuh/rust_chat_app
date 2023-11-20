use serde::{Deserialize, Serialize};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};
use tungstenite::{Message, WebSocket};

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

impl SocketCommands {
    pub fn execute(
        &self,
        websocket: &mut WebSocket<TcpStream>,
        messages_db: &Arc<Mutex<Vec<String>>>,
        json_message: SocketMessageFormat,
    ) {
        match self {
            SocketCommands::FetchMessages => self.fetch_messages(websocket, messages_db),
            SocketCommands::NewMessage => match json_message.message {
                Some(socket_msg) => {
                    messages_db.lock().unwrap().push(socket_msg.clone());
                    self.send_message(websocket, socket_msg);
                }
                None => {}
            },
            SocketCommands::Nothing => {}
        };
    }
    fn send_message(&self, websocket: &mut WebSocket<TcpStream>, message: String) {
        let msg_to_send = serde_json::to_string(&SendMessage {
            command: String::from("new_message"),
            message: message,
        })
        .unwrap();

        let _ = websocket.send(Message::Text(msg_to_send)).map_err(|err| {
            eprintln!("cannot Send message, {}", err);
        });
    }

    fn fetch_messages(
        &self,
        websocket: &mut WebSocket<TcpStream>,
        messages: &Arc<Mutex<Vec<String>>>,
    ) {
        for msg in messages.lock().unwrap().iter() {
            self.send_message(websocket, msg.clone())
        }
    }
}

pub fn match_command_or_message(input: &str) -> SocketCommands {
    match input {
        "fetch_messages" => SocketCommands::FetchMessages,
        "new_message" => SocketCommands::NewMessage,
        _ => SocketCommands::Nothing,
    }
}
