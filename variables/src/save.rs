#[derive(Debug)]
struct Result(
    i32,
    i32,
    i32,
    i32,
    f64,
    f32,
    u32,
    f64,
    i32,
    f64,
    f64,
    bool,
    bool,
    char,
    char,
    char,
);

pub fn print_save() {
    /********** VARIABLES **********/
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 2;
    println!("The value of x is : {}", x);

    const CONSTANT_NUMBER: u32 = 100_000;
    println!("Some constant value: {}", CONSTANT_NUMBER);

    // shadowing
    let x: u32 = 5;
    println!("The value of x is : {}", x);
    let x: &str = "six";
    println!("The value of x is : {}", x);

    /********** TYPES **********/
    // Integers
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
                              // let e: u8 = b'A'; // Byte (u8 only)

    // Floating-point numbers
    let h: f64 = 2.0;
    let f: f32 = 5.0;

    // addititon
    let sum: u32 = 5 + 10;
    // substraction
    let difference: f64 = 94.5 + 10.4;
    // multiplication
    let product: i32 = 9 * 14;
    // division
    let quotient: f64 = 51.9 / 21.6;
    // remainder
    let remainder: f64 = 4.9 % 2.6;

    // Booleans
    let t: bool = true;
    let h: bool = false;

    // Character
    let q: char = 'z';
    let z: char = 'Z';
    let cat: char = '';

    // Compound Types
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);
    let (channel, sub_count) = tup;

    // 6:48

    let result = (
        a, b, c, d, f, sum, difference, product, quotient, remainder, t, h, q, z, cat,
    );

    println!("{:?}", result);
}
