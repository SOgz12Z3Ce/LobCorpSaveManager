use std::{error::Error, fs::OpenOptions, io::Read, str::FromStr};

use crate::save::settings::{SettingsRawSave, SettingsSave};

mod integrator;
mod save;

// The complier is screaming, I'll suppress they after a second.

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open("settings.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let raw_settings = SettingsRawSave::from_str(&content)?;
    println!("Raw: {:?}", raw_settings);
    let settings = SettingsSave::from_raw(raw_settings)?;
    println!("Pure: {:?}", settings);
    Ok(())
}
