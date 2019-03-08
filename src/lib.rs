use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::sync::*;
use std::sync::mpsc;
use std::thread;

static mut RECEIVER: Option<Mutex<mpsc::Receiver<[u8; 512]>>> = None;

fn init() {
    let (tx, rx) = mpsc::channel();

    unsafe {
        RECEIVER = Some(Mutex::new(rx));
    }

    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:9841").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 512];

            stream.read(&mut buffer).unwrap();

            tx.send(buffer).unwrap();
        }
    });
}

fn get_msg() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        init();
    }
}
