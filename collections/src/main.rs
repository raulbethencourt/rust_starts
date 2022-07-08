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
}
