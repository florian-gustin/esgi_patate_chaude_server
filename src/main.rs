use clap::{App, Arg, ArgMatches};
use crate::challenge_generator::WordsList;

mod challenge_message;
mod server_message;
mod client_message;
mod connection_manager;
mod player;
mod challenge;
mod md5cash_challenge;
mod challenge_generator;

fn main() {
    let args = App::new("patate_chaude_client")
        .version("1.0")
        .arg(Arg::with_name("complexity")
            .short("c".parse().unwrap())
            .help("Sets the complexity, default is 16")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("s".parse().unwrap())
            .help("Sets the secret password, default is 1234")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p".parse().unwrap())
            .help("Sets the port, default is 7878")
            .takes_value(true))
        .arg(Arg::with_name("round")
            .short("r".parse().unwrap())
            .help("Sets the number of round, default is 100")
            .takes_value(true))
        .arg(Arg::with_name("round-time")
            .short("t".parse().unwrap())
            .help("Sets the max time for a round, default is 2s")
            .takes_value(true))
        .get_matches();
    let mut words_list = WordsList::new();
    words_list = challenge_generator::init_word_list(words_list);
    connection_manager::start_listening(get_complexity(&args), get_password(&args), get_port(&args), get_round(&args), get_round_time(&args), words_list);
}

fn get_complexity(args: &ArgMatches) -> u32 {
    if args.is_present("complexity") {
        match args.value_of("complexity") {
            Some(value) =>
                match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => 16,
                },
            None => 16,
        }
    } else {
        16
    }
}

fn get_password(args: &ArgMatches) -> String {
    if args.is_present("password") {
        match args.value_of("password") {
            Some(password) => password.to_string(),
            None => "1234".to_string(),
        }
    } else {
        "1234".to_string()
    }
}

fn get_port(args: &ArgMatches) -> u16 {
    if args.is_present("port") {
        match args.value_of("port") {
            Some(port) =>
                match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(err) => panic!("{:?}", err),
                },
            None => 7878,
        }
    } else {
        7878
    }
}

fn get_round(args: &ArgMatches) -> u32 {
    if args.is_present("round") {
        match args.value_of("round") {
            Some(round) =>
                match round.parse::<u32>() {
                    Ok(round) => round,
                    Err(err) => panic!("{:?}", err),
                },
            None => 100,
        }
    } else {
        100
    }
}

fn get_round_time(args: &ArgMatches) -> u32 {
    if args.is_present("round-time") {
        match args.value_of("round-time") {
            Some(round_time) =>
                match round_time.parse(){
                    Ok(round_time) => round_time,
                    Err(err) => panic!("{:?}", err),
                },
            None => 2,
        }
    } else {
        2
    }
}
