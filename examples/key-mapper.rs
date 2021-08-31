use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device, InputEvent, InputEventKind, Key};
use nix::{ioctl_write_int, libc};
use std::collections::HashMap;
use std::os::unix::prelude::AsRawFd;
use std::{env, io, process};

// see /usr/include/linux/input.h
ioctl_write_int!(eviocgrab, b'E', 0x90);

fn await_release_all_keys(d: &Device) -> io::Result<()> {
    while d.get_key_state()?.iter().count() > 0 {}
    Ok(())
}

fn grab(d: &mut Device) -> io::Result<()> {
    unsafe { eviocgrab(d.as_raw_fd(), 1) }?;
    Ok(())
}

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

fn mapkey(ev: InputEvent, keymap: &HashMap<Key, Key>) -> InputEvent {
    if let InputEventKind::Key(key) = ev.kind() {
        match keymap.get(&key) {
            Some(ev1) => InputEvent::new(ev.event_type(), ev1.code(), ev.value()),
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
    let mut src_device = Device::open(fname)?;
    await_release_all_keys(&src_device)?;
    grab(&mut src_device)?;

    let mut keys = AttributeSet::<Key>::new();
    for code in (0..libc::KEY_CNT).map(|x| x as u16) {
        keys.insert(Key::new(code));
    }

    let mut dst_device = VirtualDeviceBuilder::new()?
        .name("key-mapper-example")
        .with_keys(&keys)?
        .build()?;

    let keymap = keymap();
    eprintln!("Waiting for Ctrl-C...");
    loop {
        for ev in src_device.fetch_events()? {
            dst_device.emit(&[mapkey(ev, &keymap)])?;
        }
    }
}
