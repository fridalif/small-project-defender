use crate::prelude::*;
use std::net::TcpStream;

pub fn send_message(stream: &TcpStream, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = stream.try_clone().unwrap();
    stream.write_all(message.as_bytes())?;
    Ok(())
}
