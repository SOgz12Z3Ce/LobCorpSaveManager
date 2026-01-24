use std::{error::Error, fs::OpenOptions};

use crate::save::Save;
use crate::save::options::OptionsSave;

mod integrator;
mod save;

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("test/options.json")?;
    let mut options = OptionsSave::from_file(file)?;
    println!("{}", options);
    options.language = save::options::Language::French;
    options.tooltips_enabled = false.into();
    options.backer_abnormalities_enabled = false.into();
    options.master_volume = (1.10101010110011111f32).try_into()?;
    options.music_volume = (0.5).try_into()?;
    options.last_log_index = 3;
    println!("{}", options);
    // println!("JSON string: {}", options.into_json()?);
    options.into_file("test/fix_options.json")?;
    Ok(())
}
