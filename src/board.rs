use crate::types::{Fields, N};

pub fn is_winner(fields: &Fields) -> u8 {
	let args = (fields, N as usize);
	let values = [columns(args), rows(args), left_cross(args), righ_cross(args)];

	let max = values.iter().max();

	match max {
		Some(max) => return *max,
		None => return 0,
	}
}

// check rows
fn rows((fields, n): (&Fields, usize)) -> u8 {
	for row in fields {
		let mut found = true;
		let compare: u8 = row[0];

		for i in 1..n {
			if row[i] != compare {
				found = false;
			}
		}

		if found && compare != 0 {
			return compare;
		}
	}

	0
}

// check columns
fn columns((fields, n): (&Fields, usize)) -> u8 {
	for i in 0..n {
		let mut found = true;
		let compare: u8 = fields[0][i];

		for j in 1..n {
			if fields[j][i] != compare {
				found = false;
			}
		}

		if found && compare != 0 {
			return compare;
		}
	}

	0
}

// check left cross
fn left_cross((fields, n): (&Fields, usize)) -> u8 {
	let mut i = 1;
	let mut j = 1;

	let compare = fields[0][0];
	let mut found = true;

	while i < n && j < n {
		if fields[i][j] != compare {
			found = false;
		}

		i += 1;
		j += 1;
	}

	if found && compare != 0 {
		return compare;
	}

	0
}

fn righ_cross((fields, n): (&Fields, usize)) -> u8 {
	let mut i = 1;
	let mut j = n - 2;

	let compare = fields[0][n - 1];
	let mut found = true;

	loop {
		if fields[i][j] != compare {
			found = false;
		}

		if i >= n - 1 || j <= 0 {
			break;
		}

		i += 1;
		j -= 1;
	}

	if found && compare != 0 {
		return compare;
	}

	0
}
