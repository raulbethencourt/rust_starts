mod save;
use save::examples;

use std::{
    error::Error,
    fs::{self, File},
    io,
};

fn main() -> Result<(), Box<dyn Error>> {
    let f: Result<File, io::Error> = File::open("hello.txt");
    examples();

    Ok(())
}
