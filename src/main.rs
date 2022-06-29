mod challenge_message;
mod server_message;
mod client_message;
mod connection_manager;
mod player;

fn main() {
    println!("Hello, world!");
    connection_manager::start_listening();
}
