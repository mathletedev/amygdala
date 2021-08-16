use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	if args.len() < 2 {
		println!("Error: No path given");
		return;
	}

	let path = Path::new(&args[1]);

	if !path.is_file() {
		println!("Error: File not found");
		return;
	}

	let error = |msg: &str, row: usize, col: usize, line: &str| {
		println!(
			"{}{}{}",
			path.to_str().expect(""),
			if row > 0 {
				format!(":{}", row)
			} else {
				"".to_owned()
			},
			if col > 0 {
				format!(":{}", col)
			} else {
				"".to_owned()
			}
		);

		if row > 0 && col > 0 {
			println!("{}\n{}{}\n", line, " ".repeat(col - 1), "^");
		}

		println!("Error: {}", msg);
	};

	let file = match File::open(path) {
		Ok(file) => file,
		Err(_) => {
			error("Unable to read file", 0, 0, "");
			return;
		}
	};
	let reader = BufReader::new(file);

	for (row, line) in reader.lines().enumerate() {
		let content = match line {
			Ok(line) => line,
			Err(_) => {
				error("Unable to read line", row, 0, "");
				return;
			}
		};

		for (col, token) in content.split("").enumerate() {
			match token {
				_ => {
					error(
						format!("Invalid token {}", token).as_str(),
						row + 1,
						col + 1,
						content.as_str(),
					);
					return;
				}
			}
		}
	}
}
