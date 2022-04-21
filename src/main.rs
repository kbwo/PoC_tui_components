use std::{fs, os::unix::prelude::AsRawFd};

use nix::unistd::isatty;

fn main() {
    let tty = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .expect("cannot open tty");
    let is_tty = isatty(tty.as_raw_fd()).expect("not atty");
    println!("{}", is_tty);
}
