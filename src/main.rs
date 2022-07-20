use std::{thread,time};

fn main() {
    let four_seconds = time::Duration::new(4, 0);
    let six_seconds = time::Duration::new(6, 0);
    for i in 0..30 {
        println!("{} Breath In", i);
        thread::sleep(four_seconds);
        println!("{} Breath Out", i);
        thread::sleep(six_seconds);
    }
}
