use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::time::Instant;
use crate::challenge::Challenge;
use crate::challenge_generator::generate_sentence_from_words_list;
use crate::challenge_message::Challenge::MD5HashCash;
use crate::challenge_message::{ChallengeOutput, ChallengeValue, MD5HashCashInput, ReportedChallengeResult};
use crate::challenge_message::ChallengeMessage::ChallengeResult;
use crate::client_message::ClientMessage;
use crate::md5cash_challenge::HashCash;
use crate::player::Player;
use crate::server_message::{EndOfGame, PublicPlayer, RoundSummary, ServerMessage, Welcome};
use crate::server_message::ServerMessage::{PublicLeaderBoard};
use crate::WordsList;

pub(crate) fn start_listening(complexity: u32, password: String, port: u16, round: u32, time: u32, words_list: WordsList) {
    let players = &mut Vec::<Player>::new();
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {}", err)
    };

    accept_clients_connection(&listener, players, password.clone());
    wait_for_game_to_start(&listener, password.clone());

    start_game(complexity, players, round, time, &words_list);

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
        let mut stream = &player.socket;
        send_message(&mut stream, message);
    }
}

fn accept_clients_connection(listener: &TcpListener, players: &mut Vec<Player>, password: String) {
    let should_accept_players = &mut true;
    while *should_accept_players {
        println!("Players: {:?}", players);
        let mut incoming = listener.incoming();
        let stream = match incoming.next() {
            Some(stream) => match stream {
                Ok(stream) => stream,
                Err(_) => {
                    println!("Failed to connect client");
                    break;
                }
            },
            None => {
                println!("Failed to connect client");
                break;
            }
        };
        let message = read_message(&stream);
        analyse_client_message(&message, &stream, should_accept_players, players, password.clone());
    }
    println!("Stop accepting clients connections");
}

fn wait_for_game_to_start(listener: &TcpListener, password: String) {
    let wait_start_order = &mut true;
    while *wait_start_order {
        let mut incoming = listener.incoming();
        let stream = match incoming.next() {
            Some(stream) => match stream {
                Ok(stream) => stream,
                Err(_) => {
                    println!("Client failed to answer");
                    break;
                }
            },
            None => {
                println!("Client failed to answer");
                break;
            }
        };
        let message = read_message(&stream);
        let message_json = match serde_json::from_str(&message) {
            Ok(message_json) => message_json,
            Err(_) => continue
        };
        match message_json {
            ClientMessage::StartGame(start_game) => {
                println!("StartGame {:?}", start_game);
                if start_game.key == password {
                    *wait_start_order = false;
                } else {
                    println!("Wrong password");
                }
            }
            _ => {
                send_message(&stream, "Impossible to connect anymore");
            }
        }
    }
}

fn analyse_client_message(message: &str, stream: &TcpStream, should_accept_players: &mut bool, players: &mut Vec<Player>, password: String) {
    // println!("{:?}", message);
    let message_json = match serde_json::from_str(&message) {
        Ok(message_json) => message_json,
        Err(_) => return
    };
    match message_json {
        ClientMessage::Hello => {
            // println!("Hello");
            register_new_player(stream, players);
        }
        ClientMessage::Subscribe(subscribe) => {
            println!("Subscribe {:?}", subscribe);
            send_message(stream, "Unexpected message here");
        }
        ClientMessage::StartGame(start_game) => {
            println!("StartGame {:?}", start_game);
            if start_game.key == password {
                *should_accept_players = false;
            } else {
                println!("Wrong password");
            }
        }
    }
}

fn register_new_player(stream: &TcpStream, players: &mut Vec<Player>) {
    let response = ServerMessage::Welcome(Welcome { version: 1 });
    let server_message = match serde_json::to_string(&response) {
        Ok(server_message) => server_message,
        Err(_) => return
    };
    send_message(&stream, &server_message);
    let message = read_message(&stream);
    let message_json = match serde_json::from_str(&message) {
        Ok(message_json) => message_json,
        Err(_) => return
    };
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

fn start_game(complexity: u32, players: &mut Vec<Player>, round: u32, time: u32, words_list: &WordsList) {
    for round_number in 0..round {
        let random_time = rand::random::<f32>() * time as f32;
        println!("Round {} will last {}", round_number, random_time);
        send_leaderboard(players);
        let chain = process_round(complexity, players, random_time, &words_list);
        send_round_summary(players, chain);
    }
}

fn send_leaderboard(players: &mut Vec<Player>) {
    players.sort_by(|a, b| b.score.cmp(&a.score));
    let public_players: Vec<PublicPlayer> = get_ordered_public_player_vec(players);
    let message = match serde_json::to_string(&PublicLeaderBoard(public_players)) {
        Ok(message) => message,
        Err(_) => return
    };
    send_message_to_players(players, &message);
}

fn process_round(complexity: u32, players: &mut Vec<Player>, round_time: f32, words_list: &WordsList) -> Vec<ReportedChallengeResult>{
    let mut round_ended = false;
    let mut elapsed_time = 0.0;
    let mut chain = Vec::<ReportedChallengeResult>::new();

    let players_list_size = players.len();
    let mut target_player = &mut players[rand::random::<usize>() % players_list_size];
    let mut next_target = &String::new();

    while !round_ended {
        let input = MD5HashCashInput {
            complexity,
            message: generate_sentence_from_words_list(&words_list)
        };
        let challenge = ServerMessage::Challenge(MD5HashCash(input.clone()));

        let hashcash = HashCash::new(input.clone());
        let challenge_string = match serde_json::to_string(&challenge) {
            Ok(challenge_string) => challenge_string,
            Err(_) => continue
        };

        let time_before_completion = Instant::now();
        send_message(&target_player.socket, &challenge_string);

        let message = read_message(&target_player.socket);
        let solving_time = Instant::now().duration_since(time_before_completion).as_secs_f64();
        let message_json = match serde_json::from_str(&message) {
            Ok(message_json) => message_json,
            Err(_) => continue
        };

        let mut is_solved = false;
        match message_json {
            ChallengeResult(ref challenge_result) => {
                next_target = &challenge_result.next_target;
                match &challenge_result.answer {
                    ChallengeOutput::MD5HashCash(md5hashcash) => {
                        is_solved = hashcash.verify(md5hashcash.clone());
                    }
                    ChallengeOutput::RecoverSecret(_) => {}
                }
            }
        }

        elapsed_time += solving_time;
        target_player.total_used_time += solving_time;
        if elapsed_time > round_time as f64 {
            round_ended = true;
            target_player.score -= 1;
        } else if !is_solved {
            round_ended = true;
            target_player.score -= 1;
        } else {
            target_player.steps += 1;
        }

        chain.push(ReportedChallengeResult {
            name: target_player.name.to_string(),
            value: ChallengeValue::Ok(crate::challenge_message::ChallengeValueResult {
                used_time: elapsed_time,
                next_target: next_target.to_string(),
            }),
        });
        println!("For challenge={:?}, correct answer={}, round time={} elapsed time={} answer found in {}, player answered {:?}", input.clone(), is_solved, round_time, elapsed_time, solving_time, message_json);
        target_player = match find_player_by_username(players, next_target.to_string()) {
            Some(player) => player,
            None => {
                println!("No active player found");
                return chain;
            }
        };
    }
    println!("End of round, player {} lost a point", target_player.name);
    return chain;
}

fn find_player_by_username(players: &mut Vec<Player>, username: String) -> Option<&mut Player> {
    let mut found_player = None;
    for player in players {
        if player.clone().name == username && player.clone().is_active {
            return Some(player);
        }
        if player.clone().is_active {
            found_player = Some(player);
        }
    }
    return found_player;
}

fn send_round_summary(players: &mut Vec<Player>, chain: Vec<ReportedChallengeResult>) {
    let challenge_name = match chain.first() {
        Some(challenge) => challenge.name.clone(),
        None => "".to_string()
    };
    let message = ServerMessage::RoundSummary(RoundSummary{
        challenge: challenge_name,
        chain,
    });
    let message_json = match serde_json::to_string(&message) {
        Ok(message) => message,
        Err(_) => return
    };
    send_message_to_players(players, &message_json);
}

fn finish_game(players: &mut Vec<Player>) {
    players.sort_by(|a, b| b.score.cmp(&a.score));
    let public_players: Vec<PublicPlayer> = get_ordered_public_player_vec(players);
    let message = ServerMessage::EndOfGame(EndOfGame {
        leader_board: public_players,
    });
    let message_json = match serde_json::to_string(&message) {
        Ok(message) => message,
        Err(_) => "".to_string()
    };
    println!("{:?}", message_json);
    send_message_to_players(players, &message_json);
    for player in players {
        let _ = &player.socket.shutdown(Shutdown::Both);
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
