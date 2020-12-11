extern crate nbd;

use std::io::{Cursor, Result};
use std::net::{TcpListener, TcpStream};

use nbd::server::{handshake, transmission, Export};

fn handle_client(data: &mut [u8], mut stream: TcpStream) -> Result<()> {
    let e = Export {
        size: data.len() as u64,
        readonly: false,
        ..Default::default()
    };
    let pseudofile = Cursor::new(data);
    handshake(&mut stream, &e)?;
    transmission(&mut stream, pseudofile)?;
    Ok(())
}

fn main() {
    let mut data = vec![0; 1_474_560];
    let listener = TcpListener::bind("127.0.0.1:10809").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_client(&mut data, stream) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error: {}", e);
                }
            },
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
