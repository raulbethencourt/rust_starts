use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

/// .
///
/// # Panics
///
/// Panics if .
pub(crate) fn examples() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    let _greeting_file: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _greeting_file: File =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    //TODO!continue propagating errors
}

/// .
///
/// # Errors
///
/// This function will return an error if .
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
fn read_username_from_file_mini() -> Result<String, io::Error> {
    let mut s: String = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn read_username_from_file_last_version() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
