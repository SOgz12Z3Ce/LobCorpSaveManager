use std::{error::Error, fs::OpenOptions};

use crate::metadata::abnormality::{Abnormality, TryLookup};
use crate::save::Save;
use crate::save::etc::EtcSave;
use crate::save::options::OptionsSave;

mod integrator;
mod metadata;
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
    options.last_log_index = (-10).try_into()?;
    println!("{}", options);
    if true {
        println!("JSON string: {}", options.into_json()?);
    } else {
        options.into_file("test/fixed_options.json")?;
    }

    let file = OpenOptions::new().read(true).open("test/etc.json")?;
    let mut etc = EtcSave::from_file(file)?;
    etc.is_core_suppression_tutorial_played = true.into();
    etc.is_kether_core_suppression_1_completed = true.into();
    etc.is_kether_core_suppression_2_completed = true.into();
    etc.is_kether_core_suppression_3_completed = true.into();
    etc.is_kether_core_suppression_4_completed = true.into();
    etc.is_kether_core_suppression_5_completed = true.into();
    etc.pending_abnormalities
        .push(Abnormality::try_lookup(100014u32)?);
    println!("{}", etc);
    if false {
        println!("JSON string: {}", etc.into_json()?);
    } else {
        etc.into_file("test/fixed_etc.json")?;
    }
    Ok(())
}
