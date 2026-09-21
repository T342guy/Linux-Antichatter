use std::path::PathBuf;
use evdev::{Device, KeyCode};
use evdev::EventSummary::Key;
use crate::error::{Error, Result};

fn is_keyboard(device: &Device) -> bool {
    device
        .supported_keys()
        .is_some_and(|keys| keys.contains(KeyCode::KEY_A))
}

pub fn list() {
    println!("{:<22} {:>5} {:<4} {}", "path", "keys", "KBD", "name");
    for (path, device) in evdev::enumerate() {
        let count = device.supported_keys().map_or(0, |k| k.iter().count());
        let kbd = if is_keyboard(&device) { "yes" } else { "no" };
        println!(
            "{:<22} {:>5} {:<4} {}",
            path.display(),
            count,
            kbd,
            device.name().unwrap_or("<unnamed>"),
        );
    }
}
