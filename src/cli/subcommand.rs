use std::{fs::File, io::Read, path::Path};

use crate::{error::Error, serde::de};

pub(crate) fn deserialize<P: AsRef<Path>>(file: P) -> Result<(), Error> {
    let mut file = File::open(file)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let file = de::parse(&content)?;
    println!("{file:?}"); // FIXME: Use a better way to display.
    Ok(())
}
