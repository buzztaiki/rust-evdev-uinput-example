use std::fs;

fn main() -> anyhow::Result<()> {
    for x in fs::read_dir("/dev/input")? {
        let path = x?.path();
        let name = path
            .file_name()
            .and_then(|x| x.to_str())
            .ok_or(anyhow::Error::msg("invalid path"))?;
        if name.starts_with("event") {
            let device = evdev::Device::open(&path)?;
            println!("{}: {}", name, device);
        }
    }

    Ok(())
}
