use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub(crate) fn start_listening() {
    let address = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(address);

    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };

    let should_accept_connections = true;
    while should_accept_connections {
        let mut incoming = listener.incoming();
        while let stream = incoming.next().unwrap() {
            let stream = stream.unwrap();
            println!("New connection!");
            let message = read_message(&stream);
            println!("{:?}", message);
        }
    }
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
