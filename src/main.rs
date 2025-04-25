mod core;
mod threadpool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, time};
use core::{pattern, life};

fn main() {

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = threadpool::ThreadPool::new(4); 

    // client
    pool.execute(||{
        let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
        let n: usize = 32;
        let start_row: usize = 2;
        let start_col: usize = 3;
        stream.write(&usize::to_ne_bytes(n)).unwrap();
        stream.write(&usize::to_ne_bytes(start_row)).unwrap();
        stream.write(&usize::to_ne_bytes(start_col)).unwrap();
    });

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(move || {
            let n: usize = read_usize_stream(&mut stream);
            let start_row: usize = read_usize_stream(&mut stream);
            let start_col: usize = read_usize_stream(&mut stream);

            let mut gol = life::Life::new(n);
            gol.apply(pattern::generate_move(start_row, start_col, n));
            for _ in 0..80 {
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", gol);
                gol.live();
                thread::sleep(time::Duration::from_millis(100));
            }

        });
    }

}

fn read_usize_stream(stream: &mut TcpStream)->usize{
    let mut buf: [u8; 8] = [0; 8];
    stream.read(&mut buf).unwrap();

    usize::from_ne_bytes(buf)
}
