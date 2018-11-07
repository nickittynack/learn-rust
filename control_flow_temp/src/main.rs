// Convert temperatures between Fahrenheit and Celsius.
use std::io;

fn main() {
	loop {
		let mut t = String::new();
		let mut units = String::new();

		println!("What temperature is it?");

		io::stdin().read_line(&mut t).expect("Failed to read line");

		let t: f32 = match t.trim().parse() {
            Ok(num) => num,
            Err(err) => panic!(err),
        };

        println!("{} in what though?", t);

        io::stdin().read_line(&mut units).expect("Failed to read line");

        let degrees: f32 = match units.trim() {
            "f" => (t - 32.0) * 5.0/9.0,
            "d" => t,
            _ => panic!("'f' or 'd' only thanks"),
        };
        let fahrenheit: f32 = (degrees * 9.0/5.0) + 32.0;

        println!("degrees   : {}", degrees);
        println!("fahrenheit: {}", fahrenheit);
	}
}
