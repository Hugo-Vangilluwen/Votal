use std::{io::Write, net::TcpStream};

use crate::network::server::VotalServer;

pub struct VotalClient {
    stream: TcpStream,
}

impl VotallyClient {
    /// Create a new VotalClient
    pub fn new(address: &str) -> VotalClient {
        let stream = TcpStream::connect(address.to_owned() + ":" + VotalServer::PORT).unwrap();

        VotalClient {
            stream,
        }
    }

    /// Get choices from server
    pub fn choices(&mut self) {
        let message = "CHOICES";

        self.stream.write_all(message.as_bytes()).unwrap();
    }
}
