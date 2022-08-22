use colored::Colorize;

mod board;
mod game;
mod input;
mod types;

fn main() {
	let text = format!("{}", "Welcome to Tic Tac Toe game").green().bold();
	println!("{}", text);
	let first_nickname = input::get("Enter first nickname: ");
	let second_nickname = input::get("Enter second nickname: ");

	game::start(first_nickname, second_nickname);
}
