use evdev_rs::{Device, GrabMode, ReadFlag};
use nix::{ioctl_read_buf, libc};
use std::fs::File;
use std::os::unix::prelude::AsRawFd;
use std::{env, io, process};

// see /usr/include/linux/input.h
ioctl_read_buf!(eviocgkey, b'E', 0x18, u8);

fn await_release_all_keys(file: &File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    let buf = &mut [0; libc::KEY_CNT / 8 + 1];
    loop {
        unsafe { eviocgkey(fd, buf) }?;
        if buf.iter().all(|x| *x == 0) {
            return Ok(());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage {} <device>", args[0]);
        process::exit(1);
    }

    let fname = &args[1];
    let file = File::open(fname)?;

    await_release_all_keys(&file)?;

    let mut d = Device::new_from_file(file)?;
    d.grab(GrabMode::Grab)?;
    loop {
        let ev = d
            .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
            .map(|(_, ev)| ev)?;
        println!("Event: {}", serde_json::to_string(&ev)?);
    }
}
