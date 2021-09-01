use std::convert::TryInto;
use std::time::SystemTime;

use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{uinput, DeviceWrapper, InputEvent, UninitDevice};

fn main() -> anyhow::Result<()> {
    // https://www.freedesktop.org/software/libevdev/doc/1.4/group__uinput.html
    let base_device = UninitDevice::new().ok_or_else(|| anyhow::anyhow!("UninitDevice::new"))?;
    base_device.enable_event_code(&EventCode::EV_KEY(EV_KEY::KEY_A), None)?;

    let device = uinput::UInputDevice::create_from_device(&base_device)?;

    device.write_event(&InputEvent::new(
        &(SystemTime::now().try_into()?),
        &EventCode::EV_KEY(EV_KEY::KEY_A),
        1,
    ))?;

    device.write_event(&InputEvent::new(
        &(SystemTime::now().try_into()?),
        &EventCode::EV_KEY(EV_KEY::KEY_A),
        0,
    ))?;

    Ok(())
}
