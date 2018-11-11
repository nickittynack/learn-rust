fn main() {
	christmas(2)
}

fn christmas(i: usize) {
	let suffix = match i + 1 {
		1 => "st",
		2 => "nd",
		3 => "rd",
		_ => "th",
	};
	println!("\nOn the {}{} of Christmas my true love gave to me", i + 1, suffix);
	lyric(i);
	if i > 0 {
		christmas(i - 1);
	}
}

fn lyric(i: usize) {
	let lyrics: [&str; 3] = [
		"A partridge in a pear tree",
		"Two turtle doves and ",
		"Three French hens",
	];
	print!("{}\n", lyrics[i]);
	if i > 0 {
		lyric(i - 1);
	}
}
