use std::{error::Error, fs::OpenOptions, io::Read, str::FromStr};

use crate::save::{
    RawSave,
    settings::{SettingsRawSave, SettingsSave},
};

mod integrator;
mod save;

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("settings.json")?;
    let raw_settings = SettingsRawSave::from_file(file)?;
    println!("Raw: {:?}", raw_settings);
    let settings = SettingsSave::from_raw(raw_settings)?;
    println!("Pure: {:?}", settings);
    Ok(())
}
