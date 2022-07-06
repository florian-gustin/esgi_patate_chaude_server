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
        let socket_clone = socket.try_clone();
        match socket_clone {
            Ok(stream) => Player {
                name,
                socket: stream,
                score: 0,
                steps: 0,
                is_active: false,
                total_used_time: 0.0,
            },
            Err(_) => panic!("Error cloning socket"),
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Player {
        let socket_clone = self.socket.try_clone();
        match socket_clone {
            Ok(socket) => Player {
                name: self.name.clone(),
                socket,
                score: self.score,
                steps: self.steps,
                is_active: self.is_active,
                total_used_time: self.total_used_time,
            },
            Err(_) => panic!("Error cloning socket"),
        }
    }
}
