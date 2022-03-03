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
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    println!("Integers: {}, {}, {}, {}, {}", a, b, c, d, e);

    let _h: f64 = 2.0;
    let f: f32 = 5.0;
    println!("Floating-point numbers: {}, {}", _h, f);

    let sum: u32 = 5 + 10;
    println!("Addition: 5 + 10 = {}", sum);

    let difference: f64 = 94.5 - 10.4;
    println!("Substraction: 94.5 - 10.1 = {}", difference);

    let product: i32 = 9 * 14;
    println!("Multiplication: 9 * 14 = {}", product);

    let quotient: f64 = 51.9 / 21.6;
    println!("Division: 51.9 / 21.6 = {}", quotient);

    let remainder: f64 = 4.9 % 2.6;
    println!("Remainder: 4.9 % 2.6 = {}", remainder);

    // Booleans
    let t: bool = true;
    let h: bool = false;
    println!("Booleans: {} or {}", t, h);

    // Character
    let q: char = 'z';
    let z: char = 'Z';
    let cat: char = '';
    println!("Characters: {}, {}, {}", q, z, cat);

    // Compound Types
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);
    let (channel, sub_count1) = tup;
    let sub_count2 = tup.1;
    println!("Compound type: {} {} {}", channel, sub_count1, sub_count2);

    // Array
    let error_codes: [i32; 3] = [200, 404, 500];
    let not_found: i32 = error_codes[1];
    let array: [i32; 6] = [9; 6]; 
    println!("Array results: {}, {}", not_found, array[2]);


}
