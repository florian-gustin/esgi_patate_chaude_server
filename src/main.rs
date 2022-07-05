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
    println!("Hello, world!");
    connection_manager::start_listening(get_password(&args), get_port(&args), get_round(&args), get_round_time(&args), words_list);
}

fn get_password(args: &ArgMatches) -> String {
    if args.is_present("password") {
        args.value_of("password").unwrap().to_string()
    } else {
        "1234".to_string()
    }
}

fn get_port(args: &ArgMatches) -> u16 {
    if args.is_present("port") {
        args.value_of("port").unwrap().parse().unwrap()
    } else {
        7878
    }
}

fn get_round(args: &ArgMatches) -> u32 {
    if args.is_present("round") {
        args.value_of("round").unwrap().parse().unwrap()
    } else {
        100
    }
}

fn get_round_time(args: &ArgMatches) -> u32 {
    if args.is_present("round-time") {
        args.value_of("round-time").unwrap().parse().unwrap()
    } else {
        2
    }
}
