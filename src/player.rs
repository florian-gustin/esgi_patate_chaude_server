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
