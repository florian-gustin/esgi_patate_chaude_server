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

impl Clone for Player {
    fn clone(&self) -> Player {
        Player {
            name: self.name.clone(),
            socket: self.socket.try_clone().unwrap(),
            score: self.score,
            steps: self.steps,
            is_active: self.is_active,
            total_used_time: self.total_used_time,
        }
    }
}
