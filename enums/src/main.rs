#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    Atlanta,
}

#[derive(Debug)]
enum Coin {
    Dollar,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home, loopback);

    let help = Message::Write(String::from("Socorroooo !!"));
    help.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    dbg!(some_number, some_string, absent_number);
    dbg!(value_in_cents(Coin::Dollar));
    dbg!(value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six, none);

    // empty tuple with catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // let-if
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut _count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}

/// Fn to test match with brackets
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dollar => {
            println!("Lucky Dollar");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}!", state);
            25
        }
    }
}

/// Fn to test match with options
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
