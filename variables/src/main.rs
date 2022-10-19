mod save;

fn main() {
    save::print_save();
    let result: i32 = my_function(10, 15);
    println!("My sum = {}", result);
    control_flow(15);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function with {}.", x);
    println!("Another function with {}.", y);
    y + x
}

fn control_flow(number: i32) {
    // Conditions
    if number < 10 {
        println!("first condition true");
    } else if number < 22 {
        println!("Second condition true");
    } else {
        println!("condition false");
    }

    let condition: bool = true;
    let float: f32 = if condition { 12.3 } else { 3.12 };
    println!("Other condition: {}", float);

    // Loops
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        println!("Count = {}", counter);

        if counter == 10 {
            break;
        }
    }

    let mut number2: i32 = 3;
    while number2 != 0 {
        println!("{} !", number2);

        number2 -= 1;
    }
    println!("Finish !!");

    let array: [i32; 5] = [0, 1, 2, 3, 4];
    for val in array.iter() {
        println!("Values from array: {}", val);
    }

    /* range */
    for number3 in 1..5 {
        println!("Number = {}", number3);
    }
}
