use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::slice;
use std::sync::*;
use std::sync::mpsc;
use std::thread;

//static mut RECEIVER: Option<Mutex<mpsc::Receiver<[u8; 1024]>>> = None;
static mut RECEIVER: Option<mpsc::Receiver<[u8; 1024]>> = None;

#[no_mangle]
pub extern "C" fn init() {
    let (tx, rx) = mpsc::channel();

    unsafe {
        //RECEIVER = Some(Mutex::new(rx));
        RECEIVER = Some(rx);
    }

    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:9841").unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();

            tx.send(buffer).unwrap();
        }
    });
}

#[no_mangle]
pub extern "C" fn get_msg(p_s: *mut u8, n: usize) {
    let s = unsafe {slice::from_raw_parts_mut(p_s, n)};

    //let a = [0x68u8, 0x65u8, 0x6Cu8, 0x6Cu8, 0x6Fu8, 0x00u8];
    //let s_a = &a[..];

    let rx = unsafe {RECEIVER.as_ref().unwrap()};

    let received: [u8; 1024] = rx.recv().unwrap();

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
