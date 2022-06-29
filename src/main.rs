mod challenge_message;
mod server_message;
mod client_message;
mod connection_manager;

fn main() {
    println!("Hello, world!");
    connection_manager::start_listening();
}
