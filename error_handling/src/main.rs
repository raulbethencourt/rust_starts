mod save;
use save::examples;
use std::net::IpAddr;
use std::{error::Error, fs::File, io};

fn main() -> Result<(), Box<dyn Error>> {
    let f: Result<File, io::Error> = File::open("hello.txt");
    examples();

    // use expect coz we have more information than compiler
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    Ok(())
}