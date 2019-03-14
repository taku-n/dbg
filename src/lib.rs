use std::io::ErrorKind;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::slice;
use std::sync::*;
use std::thread;

const SIZE: usize = 256;
const ADDR_PORT: &str = "127.0.0.1:9841";

static mut RECEIVER: Option<Mutex<mpsc::Receiver<Vec<u8>>>> = None;
//static mut RECEIVER: Option<mpsc::Receiver<Vec<u8>>> = None;

#[no_mangle]
pub extern "C" fn init() {
    let (tx, rx) = mpsc::channel();

    unsafe {
        RECEIVER = Some(Mutex::new(rx));
        //RECEIVER = Some(rx);
    }

    thread::spawn(move || {
        let listener = TcpListener::bind(ADDR_PORT)
                .expect("Failed to make a listener.");

        'incoming: for stream in listener.incoming() {
            let mut stream: std::net::TcpStream
                    = stream.expect("A new client came but failed.");
            let mut buf = vec![0u8; SIZE];

            'reading: loop {
                match stream.read(&mut buf) {
                    Ok(0) => continue 'incoming,  // No Bytes
                    //Ok(1) => {continue 'incoming;},  // Only NUL
                    Ok(n) => break 'reading,
                    Err(e) => match e.kind() {
                        ErrorKind::Interrupted => {continue 'reading;},
                        other_error
                                => panic!("Reading Error: {:?}", other_error),
                    },
                };
            }

            tx.send(buf).expect("Sending Error");  // buf is moved
        }
    });
}

#[no_mangle]
pub extern "C" fn get_msg(p_s: *mut u8, n: usize) {
    let s = unsafe {slice::from_raw_parts_mut(p_s, n)};

    //let a = [0x68u8, 0x65u8, 0x6Cu8, 0x6Cu8, 0x6Fu8, 0x00u8];
    //let s_a = &a[..];

    let rx = unsafe {RECEIVER.as_ref().unwrap().lock().unwrap()};

    let received = rx.recv().unwrap();

    ref_s2s(s, &received);
}

fn ref_s2s(s_dst: &mut [u8], s_src: &[u8]) {
    for (i, x) in s_src.iter().enumerate() {
        s_dst[i] = *x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        init();
    }

    #[test]
    fn ref_s2s_test() {
        let a1 = [1u8, 2u8, 3u8, 4u8];
        let s1 = &a1[..];
        let mut a2 = [5u8, 6u8, 7u8, 8u8];
        let s2 = &mut a2[..];

        ref_s2s(s2, s1);

        assert_eq!(s2, [1u8, 2u8, 3u8, 4u8]);
    }
}
