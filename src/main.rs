use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

fn main() {
	let args = env::args().collect::<Vec<String>>();

	if args.len() < 2 {
		println!("Error: No path given");
		process::exit(1);
	}

	let path = Path::new(&args[1]);

	if !path.is_file() {
		println!("Error: File not found");
		process::exit(1);
	}

	let error = |msg: &str, pos: (usize, usize), line: &str| {
		println!(
			"{}{}{}",
			path.to_str().expect(""),
			if pos.0 > 0 {
				format!(":{}", pos.0)
			} else {
				String::new()
			},
			if pos.1 > 0 {
				format!(":{}", pos.1)
			} else {
				String::new()
			}
		);

		if pos.0 > 0 && pos.1 > 0 {
			println!("{}\n{}{}\n", line, " ".repeat(pos.1 - 1), "^");
		}

		println!("Error: {}", msg);
		process::exit(1);
	};

	let file = match File::open(path) {
		Ok(file) => file,
		Err(_) => {
			error("Unable to read file", (0, 0), "");
			return;
		}
	};

	let reader = BufReader::new(file);
	let contents = reader
		.lines()
		.map(|line| line.expect("Could not parse line"))
		.collect::<Vec<String>>();

	let mut pos: (usize, usize) = (0, 0);
	let mut brackets: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
	let mut stack: Vec<(usize, usize)> = vec![];

	let add_one = |pos: (usize, usize), tabs: usize| (pos.0 + 1, pos.1 + 1 - tabs);

	let parse_tabs = |line: String| {
		let mut tabs: usize = 0;

		for token in line.chars().map(|c| c.to_string()) {
			if token != "\t" {
				break;
			}
			tabs += 1;
		}

		(tabs, (&line[tabs..line.len()]).to_string())
	};

	while pos.0 < contents.len() {
		let line = &contents[pos.0];

		while pos.1 < line.len() {
			match line.chars().nth(pos.1).unwrap().to_string().as_str() {
				"[" => {
					stack.push(pos);
				}
				"]" => {
					let start = match stack.pop() {
						Some(pos) => pos,
						None => {
							let (tabs, parsed_line) = parse_tabs(line.to_string());

							error(
								"Missing opening bracket",
								add_one(pos, tabs),
								parsed_line.as_str(),
							);
							return;
						}
					};

					brackets.insert(start, pos);
					brackets.insert(pos, start);
				}
				_ => (),
			}

			pos.1 += 1;
		}

		pos.1 = 0;
		pos.0 += 1;
	}

	if stack.len() > 0 {
		let start = stack.first().copied().expect("");
		let (tabs, line) = parse_tabs((&contents[start.0]).to_string());

		error(
			"Missing closing bracket",
			add_one(start, tabs),
			line.as_str(),
		);
	}

	pos = (0, 0);

	let mut cells: Vec<u8> = vec![0];
	let mut pointer = 0;

	while pos.0 < contents.len() {
		let line = &contents[pos.0];

		while pos.1 < line.len() {
			match line.chars().nth(pos.1).unwrap().to_string().as_str() {
				">" => {
					pointer += 1;
					if pointer == cells.len() {
						cells.push(0);
					}
				}
				"<" => {
					if pointer > 0 {
						pointer -= 1;
					}
				}
				"+" => {
					if cells[pointer] < 255 {
						cells[pointer] += 1;
					}
				}
				"-" => {
					if cells[pointer] > 0 {
						cells[pointer] -= 1;
					}
				}
				"[" => {
					if cells[pointer] == 0 {
						pos = brackets.get(&pos).copied().expect("");
					}
				}
				"]" => {
					if cells[pointer] > 0 {
						pos = brackets.get(&pos).copied().expect("");
					}
				}
				"." => {
					print!("{}", cells[pointer] as char);
				}
				_ => (),
			}

			pos.1 += 1;
		}

		pos.1 = 0;
		pos.0 += 1;
	}
}
