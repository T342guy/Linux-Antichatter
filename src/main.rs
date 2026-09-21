mod filter;
mod error;
mod device;

use evdev::{KeyCode, EventSummary, };

fn main() -> error::Result<()> {
    let (path, mut keyboard) = device::find_keyboard()
    for (path, device) in evdev::enumerate() {
        let is_keyboard = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(KeyCode::KEY_A));
        if is_keyboard {
            println!("{}: {}", path.display(), device.name().unwrap_or("<unnamed>"));
        }
    }
    Ok(())
}
