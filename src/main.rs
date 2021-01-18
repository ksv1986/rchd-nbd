extern crate nbd;

use std::fs::File;
use std::io;
use std::io::{Read, Result, Seek, Write};
use std::net::{TcpListener, TcpStream};

use chd::Chd;
use nbd::server::{handshake, transmission, Export};

fn handle_client<T>(file: &mut T, size: u64, mut stream: TcpStream) -> Result<()>
where
    T: Read + Seek + Write,
{
    let e = Export {
        size,
        readonly: false,
        ..Default::default()
    };
    handshake(&mut stream, &e)?;
    transmission(&mut stream, file)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let path = std::env::args_os()
        .nth(1)
        .expect("Usage: rchd-nbd <chd-file> [parent-chd-file]");
    let mut chd = Chd::open(File::open(path)?)?;
    chd.write_summary(&mut std::io::stdout())?;
    println!("");
    if let Some(parent_path) = std::env::args_os().nth(2) {
        let parent = Chd::open(File::open(parent_path)?)?;
        println!("Using parent chd file:");
        parent.write_summary(&mut std::io::stdout())?;
        println!("");
        chd.set_parent(parent)?;
    }
    let size = chd.size();

    let listener = TcpListener::bind("127.0.0.1:10809").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_client(&mut chd, size, stream) {
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
    Ok(())
}
