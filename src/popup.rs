use crate::{Grid, AsFrame, Interact, Request, Position};
use crate::frame::Frame;

pub trait PopUp: Grid + AsFrame + Interact { }

fn min(lhs: usize, rhs: usize) -> usize {
	if lhs < rhs { lhs } else { rhs }
}

fn ceil_div(dividend: usize, divisor: usize) -> usize {
	(dividend + divisor - 1) / divisor 
}

struct TextBox {
	text: String,
	width: usize, 
	height: usize, 
}

impl TextBox {
	fn new(text: &str, width: usize, height: usize) -> Self {
		Self {
			text: Self::wrap_text(text, width, height),
			width, 
			height, 
		}
	}

	fn wrap_text(text: &str, max_width: usize, max_height: usize) -> String {
		let mut wrapped_text = String::new();

		for line in text.lines() {
			let mut line_length = 0;

			for c in line.chars() {
				if line_length == max_width {
					wrapped_text.push('\n');
					line_length = 0;
				}

				wrapped_text.push(c);
				line_length += 1;
			}

			wrapped_text.push('\n');
		}

		wrapped_text
			.split_inclusive(|c: char| c == '\n')
			.take(max_height)
			.collect::<String>()
	}


	fn write_to_frame(&self, frame: &mut Frame, position: &Position, text_color: &str, alignment: &Alignment) {
		for (line_index, line) in self.text.lines().enumerate() {
			for (char_index, c) in line.chars().enumerate() {
				let alignment_padding = match alignment {
					Alignment::Left => 0,
					Alignment::Center => (self.width - line.len()) / 2,
					Alignment::Right => self.width - line.len(),
				};

				let char_position = Position::new(
					position.x + alignment_padding + char_index, 
					position.y + line_index,
				);

				frame[char_position] = 
					format!("{text_color}{c}"); 
			}
		}
	}
}

// struct Color {
// 	r: u8, 
// 	g: u8, 
// 	b: u8, 
// }

// impl Color {
// 	fn new(r: u8, g: u8, b: u8) -> Self {
// 		Self { r, g, b }
// 	}

// 	fn new_with_random_offset(r: u8, g: u8, b: u8, offset: u8) -> Self {
// 		use rand::{thread_rng, Rng};

// 		let mut rng = thread_rng();
		
// 		Self {
// 			r: rng.gen_range(r.saturating_sub(offset)..=r.saturating_add(offset)),
// 			g: rng.gen_range(g.saturating_sub(offset)..=g.saturating_add(offset)),
// 			b: rng.gen_range(b.saturating_sub(offset)..=b.saturating_add(offset)),
// 		}
// 	}
// }

// use std::ops::{AddAssign, SubAssign};

// impl AddAssign<u8> for Color {
// 	fn add_assign(&mut self, tint: u8) {
// 		*self = Self {
// 			r: self.r.saturating_add(tint), 
// 			g: self.g.saturating_add(tint), 
// 			b: self.b.saturating_add(tint), 
// 		}
// 	}
// }

// impl SubAssign<u8> for Color {
// 	fn sub_assign(&mut self, shade: u8) {
// 		*self = Self {
// 			r: self.r.saturating_sub(shade), 
// 			g: self.g.saturating_sub(shade), 
// 			b: self.b.saturating_sub(shade), 
// 		}
// 	}
// }

// struct TextColor {
// 	foreground_color: Color, 
// 	background_color: Color, 
// }

// impl TextColor {
// 	fn new(foreground_r: u8, foreground_g: u8, foreground_b: u8, background_r: u8, background_g: u8, background_b: u8) -> Self {
// 		Self {
// 			foreground_color: Color::new(foreground_r, foreground_g, foreground_b), 
// 			background_color: Color::new(background_r, background_g, background_b), 
// 		}
// 	}

// 	fn mask(&mut self, color_mask: ColorMask) {
// 		match color_mask {
// 			ColorMask::Tint(tint) => {
// 				self.foreground_color += tint; 
// 				self.background_color += tint; 
// 			}

// 			ColorMask::Shade(shade) => {
// 				self.foreground_color -= shade; 
// 				self.background_color -= shade; 
// 			}
// 		}

// 		self.color_string = format!(
// 			"\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
// 			self.foreground_color.r.to_string(), 
// 			self.foreground_color.g.to_string(),
// 			self.foreground_color.b.to_string(),
// 			self.background_color.r.to_string(), 
// 			self.background_color.g.to_string(),
// 			self.background_color.b.to_string(),
// 		);
// 	}



// 	fn as_str(&self) -> &str {
// 		&self.color_string
// 	}
// }

	//"\x1b[38;2;10;10;10m\x1b[48;2;210;180;140m"

enum ColorMask {
	Tint(u8), 
	Shade(u8), 
}

/*fn mask(color_string: &str, color_mask: ColorMask) -> String {
	let mut new_color = String::new();

	for c in color_string.split_inclusive("m").map(|c| c.split(";").collect::<Vec<&str>>()) {
		match c[..] {
			[prefix, _, r, g, b] => {
				let lightening = 150;
				let r: u8 = r.parse::<u8>().unwrap().saturating_add(lightening);
				let g: u8 = g.parse::<u8>().unwrap().saturating_add(lightening);
				let b: u8 = b.strip_suffix("m").unwrap().parse::<u8>().unwrap().saturating_add(lightening);

				lightened_string = format!("{lightened_string}{prefix};2;{r};{g};{b}m");
			}

			[non_color_string] => lightened_string = format!("{lightened_string}{any}"),
			_ => ()
		}
	}
}*/

pub fn mask(color_string: &str) -> String {
	let mut lightened_string = String::new();

	for c in color_string.split_inclusive("m").map(|c| c.split(";").collect::<Vec<&str>>()) {
		match c[..] {
			[prefix, _, r, g, b] => {
				let lightening = 150;
				let r: u8 = r.parse::<u8>().unwrap().saturating_add(lightening);
				let g: u8 = g.parse::<u8>().unwrap().saturating_add(lightening);
				let b: u8 = b.strip_suffix("m").unwrap().parse::<u8>().unwrap().saturating_add(lightening);

				lightened_string = format!("{lightened_string}{prefix};2;{r};{g};{b}m");
			}

			[any] => lightened_string = format!("{lightened_string}{any}"),
			_ => ()
		}
	}

	lightened_string
}

enum Alignment {
	Left, 
	Center, 
	Right, 
}

pub struct Message {
	text: String, 
	frame: Frame, 
}

impl Message {
	pub fn new(text: &str, width: usize, height: usize) -> Box<Self> {
		let mut frame = Frame::new();

		let is_corner = |col, row| -> bool {
			if (col == 0 || col == width - 1) && (row == 0 || row == height - 1) { true } else { false }
		};

		let is_vertical_edge = |col, _| -> bool {
			if col == 0 || col == width - 1 { true } else { false }
		};

		let is_horizontal_edge = |_, row| -> bool {
			if row == 0 || row == height - 1 { true } else { false }
		};

		let text_color = "\x1b[38;2;10;10;10m\x1b[48;2;210;180;140m";

		for row in 0..height {
			frame.add_row();

			for col in 0..width {
				frame.add_to_last_row(format!("{text_color}{c}", c = 
					if is_corner(col, row) { "x" }
					else if is_vertical_edge(col, row) { "|" }
					else if is_horizontal_edge(col, row) { "-" }
					else { " " }
				));
			}
		}

		let horizontal_padding = 3;
		let message_width = width - horizontal_padding * 2;

		let vertical_padding = 2;
		let message_height = height - vertical_padding * 2;

		let text_box = TextBox::new(text, message_width, message_height);

		text_box.write_to_frame(&mut frame, &Position::new(horizontal_padding, vertical_padding), &text_color, &Alignment::Center);

		let message = Self {
			text: text.to_string(), 
			frame, 
		};

		Box::new(message)
	}

	pub fn auto(text: &str, width_limit: usize, height_limit: usize) -> Box<Self> {
		let horizontal_padding = 3;
		let vertical_padding = 2;

		let max_message_width = text.lines().map(|line| line.len()).max().unwrap_or(0);
		let message_width_limit = width_limit - horizontal_padding * 2;

		let message_width = min(max_message_width, width_limit); 
		let message_height: usize = text.lines().map(|line| ceil_div(line.len(), message_width)).sum();

		let frame_width = message_width + horizontal_padding * 2;
		let frame_height = message_height + vertical_padding * 2;

		Self::new(text, frame_width, frame_height)
	}

}

struct Log(Message);

impl PopUp for Message { }

impl AsFrame for Message {
	fn as_frame(&self) -> &Frame {
		&self.frame
	}
}

impl Interact for Message {
	fn handle_input(&mut self, input: &char) -> Request {
		match input {
			'e' => Request::Close,
			_ => Request::None, 
		}
	}
}

impl Grid for Message {
	fn width(&self) -> usize {
		self.frame.width()
	}

	fn height(&self) -> usize {
		self.frame.height()
	}
}

pub struct Info(Message);

impl Info {
	pub fn new(text: &str, max_width: usize, max_height: usize) -> Box<Self> {
		Box::new(Self(*Message::auto(text, max_width, max_height)))
	}
}

impl Grid for Info {
	fn width(&self) -> usize {
		self.0.frame.width()
	}

	fn height(&self) -> usize {
		self.0.frame.height()
	}
}

impl PopUp for Info { }

impl AsFrame for Info {
	fn as_frame(&self) -> &Frame {
		&self.0.frame
	}
}

impl Interact for Info {
	fn handle_input(&mut self, input: &char) -> Request {
		match input {
			'i' => Request::Close,
			_ => Request::None, 
		}
	}
}

pub struct Exit(Message);

impl Exit {
	pub fn new() -> Box<Self> {
		Box::new(Self(*Message::auto("Exit Game?\ny/n", 114, 115)))
	}
}

impl PopUp for Exit { }

impl Grid for Exit {
	fn width(&self) -> usize {
		self.0.frame.width()
	}

	fn height(&self) -> usize {
		self.0.frame.height()
	}
}

impl AsFrame for Exit {
	fn as_frame(&self) -> &Frame {
		&self.0.frame
	}
}

impl Interact for Exit {
	fn handle_input(&mut self, input: &char) -> Request {
		match input {
			'y' => Request::CloseGame,
			'n' => Request::Close,
			_ => Request::None, 
		}
	}
}

pub struct More(Message);

impl More {
	pub fn new(text: &str, max_width: usize, max_height: usize) -> Box<Self> {
		Box::new(
			Self(
				*Message::auto(text, max_width, max_height)
			)
		)
	}
}

impl Grid for More {
	fn width(&self) -> usize {
		self.0.frame.width()
	}

	fn height(&self) -> usize {
		self.0.frame.height()
	}
}

impl PopUp for More { }

impl AsFrame for More {
	fn as_frame(&self) -> &Frame {
		&self.0.frame
	}
}

impl Interact for More {
	fn handle_input(&mut self, input: &char) -> Request {
		match input {
			'm' => {
				let new_message_text = format!("{} do it again", &self.0.text);
				Request::Open(More::new(&new_message_text, 150, 60))
			}

			_ => Request::None, 
		}
	}
}

pub struct PopUps(Vec<Box<dyn PopUp>>);

impl PopUps {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn add_popup(&mut self, boxed_popup: Box<dyn PopUp>) {
		self.0.push(boxed_popup);
	}

	pub fn close_popup(&mut self) {
		self.0.pop();
	}

	pub fn is_empty(&self) -> bool {
		self.0.len() == 0
	}
}

impl Interact for PopUps {
	fn handle_input(&mut self, input: &char) -> Request {
		let escape_char = 27 as char;

		if *input == escape_char {
			self.0.pop();
			return Request::None;
		} else {
			let last_index = self.0.len() - 1;
			return self.0[last_index].handle_input(input);
		}
	}
}

impl<'a> IntoIterator for &'a PopUps {
	type Item = &'a Box<dyn PopUp + 'a>;
	type IntoIter = std::slice::Iter<'a, Box<dyn PopUp + 'a>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}


