use std::{
    fs::{self, File},
    io::{self, Write},
    os::unix::prelude::AsRawFd,
};

use nix::{
    errno::Errno,
    sys::termios::{cfmakeraw, tcgetattr, tcsetattr, OutputFlags, SetArg, Termios},
    unistd::isatty,
};

pub fn configuration(tty: &File) -> Result<Termios, io::Error> {
    let is_tty = isatty(tty.as_raw_fd()).expect("not atty");
    if !is_tty {
        return Err(io::Error::from(Errno::ENOTTY));
    }
    let prev_conf = tcgetattr(tty.as_raw_fd()).unwrap();
    let mut conf = prev_conf.clone();
    cfmakeraw(&mut conf);
    conf.output_flags |= OutputFlags::OPOST;
    tcsetattr(tty.as_raw_fd(), SetArg::TCSANOW, &conf).unwrap();
    return Ok(conf);
}

fn main() {
    let tty = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .expect("cannot open tty");
    let is_tty = isatty(tty.as_raw_fd()).expect("not atty");
    println!("is tty {}", is_tty);
}
