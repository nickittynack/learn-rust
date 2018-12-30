#![allow(dead_code)]

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(AustralianState),
}

#[derive(Debug)]
enum AustralianState {
    NSW,
    QLD,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => Some(0)
    }
}

fn main() {
	println!("{}", value_in_cents(Coin::Quarter(AustralianState::QLD)));
	plus_one(Some(1));
	match plus_one(plus_one(plus_one(None))) {
		Some(2) => println!("Works I guess"),
		_ => println!("Nope"),
	}
}
