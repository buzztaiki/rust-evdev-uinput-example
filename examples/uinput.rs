use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key};
use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::KEY_A);

    let mut device = VirtualDeviceBuilder::new()?
        .name("uinput-example")
        .with_keys(&keys)?
        .build()
        .unwrap();

    eprintln!("Waiting for Ctrl-C...");
    loop {
        device.emit(&[InputEvent::new(EventType::KEY, Key::KEY_A.code(), 1)])?;
        eprintln!("Pressed.");
        sleep(Duration::from_secs(2));

        device.emit(&[InputEvent::new(EventType::KEY, Key::KEY_A.code(), 0)])?;
        eprintln!("Released.");
        sleep(Duration::from_secs(2));
    }
}
