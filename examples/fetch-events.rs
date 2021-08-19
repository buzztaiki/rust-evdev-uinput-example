use std::{env, process};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage {} <device>", args[0]);
        process::exit(1);
    }

    let fname = &args[1];
    let mut d = evdev::Device::open(fname)?;
    println!("{}", d.name().unwrap_or("unnamed"));
    println!("{}", d.unique_name().unwrap_or("unnamed"));

    loop {
        for ev in d.fetch_events()? {
            println!("{:?}", ev);
        }
    }
}
