mod life;
mod pattern;
mod point;

use std::{thread, time};

fn main() {
    let n: usize = 32;

    let mut li = life::Life::new(n);

    li.apply(pattern::generate_move(1, 1, n));
    for _ in 0..80 {
        print!("\x1B[2J\x1B[1;1H");
        li.live();
        thread::sleep(time::Duration::from_millis(500));
    }
}
