use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device, InputEvent, InputEventKind, Key};
use nix::{ioctl_write_int, libc};
use std::collections::HashMap;
use std::os::unix::prelude::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use std::{env, process};

// see /usr/include/linux/input.h
ioctl_write_int!(eviocgrab, b'E', 0x90);

fn keymap() -> HashMap<Key, Key> {
    vec![
        (Key::KEY_O, Key::KEY_0),
        (Key::KEY_0, Key::KEY_O),
        (Key::KEY_RIGHTCTRL, Key::KEY_RIGHTALT),
        (Key::KEY_RIGHTALT, Key::KEY_RIGHTCTRL),
    ]
    .iter()
    .cloned()
    .collect()
}

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage {} <device>", args[0]);
        process::exit(1);
    }

    let fname = &args[1];
    let mut input_device = Device::open(fname)?;
    while input_device.get_key_state()?.iter().count() > 0 {
        sleep(Duration::from_millis(100));
    }

    unsafe {
        eviocgrab(input_device.as_raw_fd(), 1)?;
    }

    let mut keys = AttributeSet::<Key>::new();
    for code in (0..libc::KEY_CNT).map(|x| x as u16) {
        keys.insert(Key::new(code));
    }

    let mut device = VirtualDeviceBuilder::new()?
        .name("key-mapper-example")
        .with_keys(&keys)?
        // TODO
        // .with_relative_axes(axes)?
        .build()?;

    let keymap = keymap();
    eprintln!("Waiting for Ctrl-C...");
    loop {
        for ev in input_device.fetch_events()? {
            // TODO key 以外のイベント
            if let InputEventKind::Key(key) = ev.kind() {
                let ev1 = match keymap.get(&key) {
                    Some(ev1) => InputEvent::new(ev.event_type(), ev1.code(), ev.value()),
                    None => ev,
                };
                device.emit(&[ev1])?;
            }
        }
    }
}
