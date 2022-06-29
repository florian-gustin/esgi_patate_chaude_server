use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::client_message::ClientMessage;
use crate::player::Player;
use crate::server_message::{ServerMessage, SubscribeResult, Welcome};

pub(crate) fn start_listening() {
    let players = &mut Vec::<Player>::new();
    let address = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };

    accept_clients_connection(&listener, players);
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn transform_array_of_u8_to_u32(x:[u8;4]) -> u32 {
    ((x[0] as u32) << 24) | ((x[1] as u32) << 16) | ((x[2] as u32) << 8) | (x[3] as u32)
}

fn read_message(mut stream: &TcpStream) -> String {
    let mut length_buffer = [0; 4];
    let buffer_response = stream.read(&mut length_buffer);
    match buffer_response {
        Err(error) => {
            panic!("{:?}", error)
        }
        _ => {}
    }

    let length = transform_array_of_u8_to_u32(length_buffer);
    let mut buffer = vec![0; length as usize];
    let response = stream.read(&mut buffer);
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }

    let message = String::from_utf8_lossy(&buffer);
    // println!("Received message \"{message}\" of length {message_size}", message = message, message_size = length);
    return message.to_string();
}

fn send_message(mut stream: &TcpStream, message: &str) {
    let message_size: u32 = message.len() as u32;
    let encoded_size = &transform_u32_to_array_of_u8(message_size);

    let response = stream.write(encoded_size);
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }

    let response = stream.write(message.as_bytes());
    match response {
        Err(error) => {
            panic!("{:?}", error);
        }
        _ => {}
    }
}

fn accept_clients_connection(mut listener: &TcpListener, players: &mut Vec<Player>) {
    let should_accept_connections = &mut true;
    let streams = &mut Vec::<TcpStream>::new();
    while *should_accept_connections {
        let mut incoming = listener.incoming();
        let stream = incoming.next().unwrap();
        let stream = stream.unwrap();
        streams.push(stream.try_clone().unwrap());
        let message = read_message(&stream);
        analyse_client_message(&message, &stream, should_accept_connections, players);
    }
    println!("Stop accepting clients connections");
}

fn analyse_client_message(message: &str, stream: &TcpStream, should_accept_players: &mut bool, players: &mut Vec<Player>) {
    println!("{:?}", message);
    let message_json = serde_json::from_str(&message).unwrap();
    match message_json {
        ClientMessage::Hello => {
            println!("Hello");
            let response = ServerMessage::Welcome(Welcome { version: 1 });
            send_message(&stream, &serde_json::to_string(&response).unwrap());
        }
        ClientMessage::Subscribe(subscribe) => {
            println!("Subscribe {:?}", subscribe);
            players.push(Player {
                name: subscribe.name,
                socket: stream.try_clone().unwrap(),
                score: 0,
                steps: 0,
                is_active: true,
                total_used_time: 0.0
            });
            let mut response = ServerMessage::SubscribeResult(SubscribeResult::Ok);
            send_message(&stream, &serde_json::to_string(&response).unwrap());
        }
        ClientMessage::StartGame(start_game) => {
            println!("StartGame {:?}", start_game);
            *should_accept_players = false;
        }
    }
}
