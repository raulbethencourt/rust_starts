use restaurant::eat_at_restaurant;
use std::collections::HashMap;
use std::fmt;
use std::fmt::format;
use std::io;

/// .
fn main() {
    eat_at_restaurant();

    // . Work with exterior libraries
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);

    let _test1: Result<(), fmt::Error> = function1();
    let _test2: Result<(), io::Error> = function2();
}

fn function1() -> fmt::Result {
    // .
    format(format_args!("{}", 1));
    Ok(())
}

fn function2() -> io::Result<()> {
    // .
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}
