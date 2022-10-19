mod hashmap;
use crate::hashmap::set_hashmap;
use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(5);
    v.push(6);

    dbg!(&mut v, &v1);

    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    dbg!(&row);
    for value in &row {
        println!("{:?}", value);
    }

    let s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();
    let word = "hello";
    let mut s4 = word.to_string();

    //format! macro uses references so that this call doesn’t take ownership of any of its parameters.
    let s5 = format!("{} {}!!", s4, s3);

    s4.push_str(&s3);

    dbg!(&s1, &s2, &s3, &s4, &s5);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    "नमस्ते".bytes().for_each(|b| {
        println!("{}", b);
    });

    let hash_data = set_hashmap(String::from("red"), 69);
    dbg!(hash_data.get(&String::from("red")));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    print!("{:?}", map);
}
