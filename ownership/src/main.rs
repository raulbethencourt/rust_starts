fn main() {
    // Memory and Allocation
    fn first() {
        let x: &str = "hello ";
        second(x);
    }

    fn second(n: &str) {
        let mut x: String = String::from("world");
        x.push_str(" and universe");
        x.insert_str(0, n);
        let t = x.clone();
        let len = calculate_length(&t);
        let x_len = calculate_length(&mut x);
        println!("{} / {} / {} / {}", x, t, len, x_len);
    }

    {
        let s = "yepaaa !!!";
        let n = s;
        println!("{} / {}", s, n);
    }

    fn calculate_length(str: &String) -> usize {
        str.len()
    }

    first();

    // String Slices
    let s = String::from("One hello world");

    let word = first_word(&s);

    println!("{}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);
}

// Return string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
