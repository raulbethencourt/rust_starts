use rand::Rng;
pub(crate) use restaurant::eat_at_restaurant;
pub(crate) use std::{collections::*, fmt::Result as fmtResult, io::Result};

/// .
fn main() {
    eat_at_restaurant();

    // . Work with exterior libraries
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 2);

    let _test1 = function1();
    let _test2 = function2();
    dbg!(_test1);
    dbg!(_test2);

    let secret_number = rand::thread_rng().gen_range(1..101);
    dbg!(secret_number);
}

fn function1() -> fmtResult {
    // .
    fmtResult::Ok(());
    Ok(())
}

fn function2() -> Result<()> {
    // .
    Result::Ok(());
    Ok(())
}
