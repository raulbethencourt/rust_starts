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
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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
    dbg!(value_in_cents(Coin::Penny));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    
}
