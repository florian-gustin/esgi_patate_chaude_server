use std::net::TcpStream;

#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) socket: TcpStream,
    pub(crate) score: i32,
    pub(crate) steps: u32,
    pub(crate) is_active: bool,
    pub(crate) total_used_time: f64,
}

//define player constructor with name and socket
impl Player {
    pub(crate) fn new(name: String, socket: &TcpStream) -> Player {
        Player {
            name,
            socket: socket.try_clone().unwrap(),
            score: 0,
            steps: 0,
            is_active: false,
            total_used_time: 0.0,
        }
    }
}
