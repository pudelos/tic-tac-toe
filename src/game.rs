use crate::board;
use crate::input;
use crate::types::{Fields, UserId, N};

use colored::Colorize;
use inflector::Inflector;

pub fn start(first_name: String, second_name: String) {
	let mut game = Game::new(first_name, second_name);

	while !game.ended {
		game.show();
		let (x, y) = game.get_move();
		game.play(x, y);
	}
}

struct Game {
	fields: Fields,
	first_nickname: String,
	second_nickname: String,
	ended: bool,
	moves: u8,
}

impl Game {
	// construct game
	fn new(first_nickname: String, second_nickname: String) -> Game {
		let fields = vec![vec![0u8; 3]; 3];
		return Game {
			fields,
			first_nickname,
			second_nickname,
			ended: false,
			moves: 0,
		};
	}

	// show board
	fn show(&mut self) {
		println!();
		for row in self.fields.iter() {
			for field in row {
				print!("{} ", field);
			}
			println!();
		}
		println!();
	}

	// get userid
	fn get_userid(&mut self) -> u8 {
		if self.moves % 2 == 0 {
			UserId::First as u8
		} else {
			UserId::Second as u8
		}
	}

	// get player nickname
	fn get_nickname(&mut self) -> String {
		if self.moves % 2 == 0 {
			self.first_nickname.clone()
		} else {
			self.second_nickname.clone()
		}
	}

	// new move
	fn play(&mut self, x: usize, y: usize) {
		let user_id = self.get_userid();
		if self.ended {
			let text = format!("{}", "Error: this game has eneded.").red();
			return println!("{}", text);
		}

		if x >= N as usize || y >= N as usize {
			let text = format!("Error: (x: {}, y: {}) field doesn't exist. Select another.", x + 1, y + 1).red();
			return println!("{}", text);
		}

		if self.fields[y][x] != 0 {
			let text = format!("Error: (x: {}, y: {}) field was used. Select another.", x + 1, y + 1).red();
			return println!("{}", text);
		}

		self.fields[y][x] = user_id;
		self.moves += 1;

		let winner_id = board::is_winner(&self.fields);
		if winner_id > 0 {
			return self.end(winner_id);
		}

		if self.moves >= 9 {
			self.ended = true;
			self.show();

			let text = format!("{}", "Draw!").yellow();
			println!("{}", text);
		}
	}

	// end game
	fn end(&mut self, winner_id: u8) {
		self.ended = true;

		let winner_nickname = if winner_id == UserId::First as u8 {
			self.first_nickname.clone()
		} else {
			self.second_nickname.clone()
		};

		self.show();

		let text = format!("{} won!", winner_nickname.to_title_case()).green();
		print!("{}", text);
	}

	// game prompt
	fn get_move(&mut self) -> (usize, usize) {
		let nickname = self.get_nickname();

		let text = format!("{}'s move: ", nickname.to_title_case()).yellow();
		println!("{}", text);

		let y: usize = input::get("Number of row (y): ").parse().unwrap();
		let x: usize = input::get("Number of column (x): ").parse().unwrap();

		return (x - 1, y - 1);
	}
}
