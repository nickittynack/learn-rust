
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
    for i in &v {
        println!("{}", i);
    }
    // Match so there's no panics
    match v.get(100) {
        Some(value) => println!("{:?}", value),
        None => println!("No 100th element"),
    }
    println!("{:?}", v[1]);
    // Iterating over a mutable list and changing it
    for i in &mut v {
        *i += 50;
    }

    // Vector can use an enum as the T
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Remove and return the last boi
    let x = v.pop();
    println!("{:?}", x);
    match x {
        Some(value) => println!("{:?}", value),
        _ => println!("No value"),
    }
}
