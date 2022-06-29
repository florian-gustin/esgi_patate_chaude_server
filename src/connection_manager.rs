use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use crate::client_message::ClientMessage;
use crate::player::Player;
use crate::server_message::{EndOfGame, PublicPlayer, ServerMessage, SubscribeError, SubscribeResult, Welcome};
use crate::server_message::ServerMessage::PublicLeaderBoard;

pub(crate) fn start_listening<'stream>() {
    let players = &mut Vec::<Player>::new();
    let address = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };

    accept_clients_connection(&listener, players);
    wait_for_game_to_start(&listener);

    start_challenge(players);

    finish_game(players);
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

fn send_message_to_players(players: &mut Vec<Player>, message: &str) {
    for player in players {
        let mut stream = player.stream.lock().unwrap();
        send_message(&mut stream, message);
    }
}

fn accept_clients_connection(listener: &TcpListener, players: &mut Vec<Player>) {
    let should_accept_players = &mut true;
    while *should_accept_players {
        println!("Players: {:?}", players);
        let mut incoming = listener.incoming();
        let stream = incoming.next().unwrap();
        let stream = stream.unwrap();
        let message = read_message(&stream);
        analyse_client_message(&message, &stream, should_accept_players, players);
    }
    println!("Stop accepting clients connections");
}

fn wait_for_game_to_start(listener: &TcpListener) {
    let wait_start_order = &mut true;
    while *wait_start_order {
        let mut incoming = listener.incoming();
        let stream = incoming.next().unwrap();
        let stream = stream.unwrap();
        let message = read_message(&stream);
        let message_json = serde_json::from_str(&message).unwrap();
        match message_json {
            ClientMessage::StartGame(start_game) => {
                println!("StartGame {:?}", start_game);
                *wait_start_order = false;
            }
            _ => {
                send_message(&stream, "Impossible to connect anymore");
            }
        }
    }
}

fn analyse_client_message(message: &str, stream: &TcpStream, should_accept_players: &mut bool, players: &mut Vec<Player>) {
    println!("{:?}", message);
    let message_json = serde_json::from_str(&message).unwrap();
    match message_json {
        ClientMessage::Hello => {
            println!("Hello");
            register_new_player(stream, players);
        }
        ClientMessage::Subscribe(subscribe) => {
            println!("Subscribe {:?}", subscribe);
            send_message(stream, "Unexpected message here");
        }
        ClientMessage::StartGame(start_game) => {
            println!("StartGame {:?}", start_game);
            *should_accept_players = false;
        }
    }
}

fn register_new_player(stream: &TcpStream, players: &mut Vec<Player>) {
    let response = ServerMessage::Welcome(Welcome { version: 1 });
    send_message(&stream, &serde_json::to_string(&response).unwrap());
    let message = read_message(&stream);
    let message_json = serde_json::from_str(&message).unwrap();
    match message_json {
        ClientMessage::Subscribe(subscribe) => {
            println!("Subscribe {:?}", subscribe);
            let player = Player::new(subscribe.name, &stream);
            players.push(player);
        }
        _ => {
            send_message(stream, "Unexpected message here");
        }
    }
}

fn start_challenge(players: &mut Vec<Player>) {
    for round_number in 0..100 {
        println!("Round {}", round_number);
        send_leaderboard(players);
        send_round_summary(players);
    }
}

fn send_leaderboard(players: &mut Vec<Player>) {
    players.sort_by(|a, b| b.score.cmp(&a.score));
    let public_players: Vec<PublicPlayer> = get_ordered_public_player_vec(players);
    let message = PublicLeaderBoard(public_players);
    send_message_to_players(players, &serde_json::to_string(&message).unwrap());
}

fn send_round_summary(players: &mut Vec<Player>) {

}

fn finish_game(players: &mut Vec<Player>) {
    players.sort_by(|a, b| b.score.cmp(&a.score));
    let public_players: Vec<PublicPlayer> = get_ordered_public_player_vec(players);
    let message = ServerMessage::EndOfGame(EndOfGame {
        leader_board: public_players,
    });
    let message_json = serde_json::to_string(&message).unwrap();
    println!("{:?}", message_json);
    send_message_to_players(players, &serde_json::to_string(&message).unwrap());
    for player in players {
        &player.socket.shutdown(Shutdown::Both);
    }
}

fn get_ordered_public_player_vec(players: &mut Vec<Player>) -> Vec<PublicPlayer> {
    players.sort_by(|a, b| b.score.cmp(&a.score));
    let public_players: Vec<PublicPlayer> = players.iter().map(|player| PublicPlayer {
        name: player.name.clone(),
        stream_id: player.socket.local_addr().unwrap().to_string(),
        score: player.score,
        steps: player.steps,
        is_active: player.is_active,
        total_used_time: player.total_used_time,
    }).collect();
    return public_players;
}
