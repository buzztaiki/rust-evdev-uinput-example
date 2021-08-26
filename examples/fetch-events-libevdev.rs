use std::fs::File;
use std::{env, process};

use evdev_rs::{Device, DeviceWrapper, ReadFlag};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage {} <device>", args[0]);
        process::exit(1);
    }

    let fname = &args[1];
    let file = File::open(fname)?;
    let d = Device::new_from_file(file)?;
    println!("{:?}", (d.name().unwrap_or(""), d.uniq().unwrap_or("")));

    loop {
        let ev = d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING).map(|(_, ev)| ev)?;
        println!("Event: {}", serde_json::to_string(&ev)?);
    }
}
