use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{uinput, Device, GrabMode, InputEvent, ReadFlag};
use nix::{ioctl_read_buf, libc};
use std::collections::HashMap;
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

fn keymap() -> HashMap<EV_KEY, EV_KEY> {
    vec![
        (EV_KEY::KEY_O, EV_KEY::KEY_0),
        (EV_KEY::KEY_0, EV_KEY::KEY_O),
        (EV_KEY::KEY_RIGHTCTRL, EV_KEY::KEY_RIGHTALT),
        (EV_KEY::KEY_RIGHTALT, EV_KEY::KEY_RIGHTCTRL),
    ]
    .iter()
    .cloned()
    .collect()
}

fn mapkey(ev: InputEvent, keymap: &HashMap<EV_KEY, EV_KEY>) -> InputEvent {
    if let EventCode::EV_KEY(key) = ev.event_code {
        match keymap.get(&key) {
            Some(key1) => InputEvent::new(&ev.time, &EventCode::EV_KEY(*key1), ev.value),
            None => ev,
        }
    } else {
        ev
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

    let mut device = Device::new_from_file(file)?;
    device.grab(GrabMode::Grab)?;

    // https://www.freedesktop.org/software/libevdev/doc/1.4/group__uinput.html
    let ui_device = uinput::UInputDevice::create_from_device(&device)?;

    let keymap = keymap();
    eprintln!("Waiting for Ctrl-C...");

    // https://gitlab.freedesktop.org/libevdev/libevdev/-/blob/master/tools/libevdev-events.c#L156
    // https://docs.rs/evdev-rs/latest/evdev_rs/struct.Device.html#method.next_event
    loop {
        let (st, ev) = device.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)?;
        match st {
            evdev_rs::ReadStatus::Success => ui_device.write_event(&mapkey(ev, &keymap))?,
            evdev_rs::ReadStatus::Sync => {
                eprintln!("dropped");
                'sync: loop {
                    match device.next_event(ReadFlag::SYNC) {
                        Ok((_, ev)) => ui_device.write_event(&mapkey(ev, &keymap))?,
                        Err(_) => {
                            eprintln!("re-synced");
                            break 'sync;
                        },
                    }
                }
            }
        }
    }
}
