use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;

    Ok(())
}
