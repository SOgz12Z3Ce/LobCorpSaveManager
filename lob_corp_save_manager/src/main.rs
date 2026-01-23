use std::{error::Error, fs::OpenOptions};

use crate::save::options::OptionsSave;

mod integrator;
mod save;

fn main() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open("options.json")?;
    let options = OptionsSave::from_file(file)?;
    println!("{}", options);
    Ok(())
}
