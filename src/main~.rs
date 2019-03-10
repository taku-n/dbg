extern crate dbg;

const SIZE: usize = 256;

fn main() {
    dbg::init();

    let mut msg = vec![0u8; SIZE];

    loop {
        dbg::get_msg(msg.as_mut_ptr(), SIZE);

        println!("{:X?}", msg);
    }
}
