use evdev::{Device, InputEventKind, Key};
use nix::ioctl_write_int;
use std::os::unix::prelude::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use std::{env, process};

// see /usr/include/linux/input.h
ioctl_write_int!(eviocgrab, b'E', 0x90);

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage {} <device>", args[0]);
        process::exit(1);
    }

    let fname = &args[1];
    let mut d = Device::open(fname)?;

    while d.get_key_state()?.iter().count() > 0 {
        sleep(Duration::from_millis(100));
    }

    unsafe {
        eviocgrab(d.as_raw_fd(), 1)?;
    }

    eprintln!("type space key to stop");
    'outer: loop {
        for ev in d.fetch_events()? {
            match ev.kind() {
                InputEventKind::Key(Key::KEY_SPACE) => break 'outer,
                _ => println!("{:?}", ev),
            }
        }
    }

    Ok(())
}
