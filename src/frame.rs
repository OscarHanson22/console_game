use crate::{Grid, Position};

pub struct Frame(Vec<Vec<String>>);

impl Frame {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn add_row(&mut self) {
		self.0.push(Vec::new());
	}

	pub fn add_to_last_row(&mut self, string: String) {
	    	let last_index = self.height() - 1;
		self.0[last_index].push(string);
	}

	pub fn add_overlay(&mut self, sub_frame: &Frame, position: &Position) {
		for y in 0..sub_frame.height() {
			for x in 0..sub_frame.width() {
				let iterating_position = Position::new(x, y);
				self[*position + iterating_position] = sub_frame[iterating_position].to_string();
			}
		}
	}

	//pub fn mask(&mut self
}

impl Grid for Frame {
	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn height(&self) -> usize {
		self.0.len()
	}
}

use std::ops::{Index, IndexMut};

impl Index<usize> for Frame {
	type Output = Vec<String>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl Index<Position> for Frame {
	type Output = String;

	fn index(&self, position: Position) -> &Self::Output {
		&self.0[position]
	}
}

impl IndexMut<usize> for Frame {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl IndexMut<Position> for Frame {
	fn index_mut(&mut self, position: Position) -> &mut Self::Output {
		&mut self.0[position]
	}
}

use std::fmt;

impl fmt::Display for Frame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		assert!(self.width() > 0);
	    	assert!(self.height() > 0);

		let mut frame_string = String::new();

		for row in 0..self.height() {
			for col in 0..self.width() {
				frame_string.push_str(&self.0[row][col]);
			}

			frame_string.push_str("\x1b[38;2;255;255;255m\x1b[48;2;0;0;0m\n");
		}

		write!(f, "{}", frame_string)
	}
}