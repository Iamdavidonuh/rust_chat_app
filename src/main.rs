use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{
    io::Error,
    net::{TcpListener, TcpStream},
    thread,
};
use tungstenite::{accept, Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
struct SocketMessageFormat {
    command: String,
    message: Option<String>,
}

#[derive(Serialize, Debug)]
struct SendMessage {
    command: String,
    message: String,
}

enum SocketCommands {
    FetchMessages,
    NewMessage,
    Nothing,
}


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
                let msg_json: SocketMessageFormat  = serde_json::from_str(&messsage_txt).unwrap();
                println!("Client json: {:?}", &msg_json);

                let command = match_command_or_message(&msg_json.command);

                match command {
                    SocketCommands::FetchMessages => fetch_messages(&mut websocket, &messages),
                    SocketCommands::NewMessage => {
                        match msg_json.message {
                            Some(socket_msg) => {
                                println!("msg.... {}", &socket_msg);
                                messages.push(socket_msg.clone());

                                let send_message = SendMessage {
                                    command: String::from("new_message"),
                                    message: socket_msg,
                                };

                                let serialized = serde_json::to_string(&send_message).unwrap();
                                websocket.send(Message::Text(serialized)).unwrap()
                            },
                            None => {
                                continue;
                            }
                            
                        }
                    },
                    SocketCommands::Nothing => continue,
                };
            }
        }
    }
}

fn match_command_or_message(input: &str) -> SocketCommands {
    println!("Command got, {}", input);
    match input {
        "fetch_messages" => SocketCommands::FetchMessages,
        "new_message" => SocketCommands::NewMessage,
        _ => SocketCommands::Nothing,
    }
}


fn send_message(websocket: &mut WebSocket<TcpStream>, message: &str) {
    let message = Message::Text(message.to_string());
    let _ = websocket.send(message).map_err(|err| {
        eprintln!("cannot reload messages, {}", err);
    });
}

fn fetch_messages(websocket: &mut WebSocket<TcpStream>, messages: &Vec<String>) {
    for msg in messages {
        send_message(websocket, &msg)
    }
}
