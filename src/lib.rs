pub mod frame;
pub mod tile;
pub mod popup;

use std::fmt;
use frame::Frame;
use tile::{Tile, Grass, /*Dirt, Fence,*/ Barrier, Blinker};
use popup::{PopUp, PopUps, Message, Exit, Info, More};

pub trait AsFrame {
	fn as_frame(&self) -> &Frame;
}

pub trait Interact {
	fn handle_input(&mut self, input: &char) -> Request {
		Request::None
	}
}

pub trait Grid {
	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

#[derive(Copy, Clone)]
enum Direction {
	Up, 
	Down,
	Left, 
	Right, 
}

pub enum Request {
	CloseGame, 
	Close, 
	Open(Box<dyn PopUp>), 
	None, 
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
	x: usize, 
	y: usize, 
}

impl Position {
	fn new(x: usize, y: usize) -> Self {
		Self {
			x, 
			y, 
		}
	}

	fn move_in_direction(&mut self, direction: &Direction) {
		match direction {
			Direction::Up => self.y -= 1, 
			Direction::Down => self.y += 1, 
			Direction::Left => self.x -= 1, 
			Direction::Right => self.x += 1, 
		}
	}

	fn position_in_direction(&self, direction: &Direction) -> Self {
		match direction {
			Direction::Up => Self { x: self.x, y: self.y - 1 },
			Direction::Down => Self { x: self.x, y: self.y + 1 }, 
			Direction::Left => Self { x: self.x - 1, y: self.y }, 
			Direction::Right => Self { x: self.x + 1, y: self.y }, 
		}
	}
}

use std::ops::{Add, Index, IndexMut};

impl<T> Index<Position> for Vec<Vec<T>> {
    	type Output = T;
    
    	fn index(&self, position: Position) -> &Self::Output {
		&self[position.y][position.x]
    	}
}

impl<T> IndexMut<Position> for Vec<Vec<T>> {    
    	fn index_mut(&mut self, position: Position) -> &mut Self::Output {
		&mut self[position.y][position.x]
    	}
}

impl Add<Position> for Position {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x, 
			y: self.y + other.y, 
		}
	}
}

pub const WINDOW_WIDTH_LIMIT: usize = 172;
pub const WINDOW_HEIGHT_LIMIT: usize = 49;

pub struct Game {
	world: World,
}

impl Game {
	pub fn new() -> Self {
		Self {
			world: World::new(250, 250, WINDOW_WIDTH_LIMIT, WINDOW_HEIGHT_LIMIT), 
		}
	}

	pub fn run(&mut self) {
		use std::{
			sync::mpsc,
			thread, 
			time::Duration,
		};

		use win32console::{
			console::WinConsole,
			structs::input_record::InputRecord::KeyEvent,
		};

		let (tx, rx) = mpsc::channel();

		thread::spawn(move || {
			loop {
				if let KeyEvent(event) = WinConsole::input().read_single_input().unwrap() {
					if event.key_down {
						tx.send(event.u_char).unwrap();
					}
				}

				thread::sleep(Duration::from_millis(10));
			}
		});

		use std::collections::HashSet;

		if let Ok(_) = WinConsole::output().clear() {};
		print!("\x1b[?25l");

	    	'mainloop: loop {
	    		print!("\x1b[H");
	    		
	    		let mut inputs = HashSet::new();

	    		while let Ok(input) = rx.try_recv() {
	    			if !inputs.contains(&input) {
	    				match self.world.handle_input(&input) {
	    					Request::CloseGame => break 'mainloop,
	    					_ => (),
	    				}
	    			}

				inputs.insert(input);
	    		}

	    		print!("{}", self.world);
	    		thread::sleep(Duration::from_millis(100));
	    	}

		print!("\x1b[H");
	    	if let Ok(_) = WinConsole::output().clear() {}
	    	print!("{}", Message::new("Thanks for playing!", 30, 5).as_frame().to_string());
	    	print!("\x1b[?25h");
	}
}

pub struct World {
	map: Map, 
	window: Window, 
	player: Player,
	popups: PopUps,
}

impl World {
	pub fn new(map_width: usize, map_height: usize, window_width: usize, window_height: usize) -> Self {
		let mut world = Self {
			map: MapMaker::lines(map_width, map_height),
			window: Window {
				width: window_width, 
				height: window_height, 
				position: centered_position(map_width, map_height, window_width, window_height), 
			},
			player: Player::new(map_width/2, map_height/2), 
			popups: PopUps::new(),
		};

		world.popups.add_popup(Message::auto(
"Welcome!

Dismiss popups with ESC
i -> info on selected tiles
m -> fun
e -> `inventory`", 
		window_width - 4, window_height - 2));

		world
	}

	fn move_player_in_direction(&mut self, direction: &Direction) {
		let next_player_position = self.player.position().position_in_direction(direction);

		let tile_at_next_player_position = self.map.tile_at_position(&next_player_position);

		if tile_at_next_player_position.is_walkable() {
			self.player.move_in_direction(direction);
		} else {
			self.player.heading = *direction;
		}
	}

	fn move_window_to_position(&mut self, position: &Position) {
		let mut window_position = *position;

		if position.x + self.window.width() > self.map.width() {
			window_position.x = self.map.width() - self.window.width();
		}

		if position.y + self.window.height() > self.map.height() {
			window_position.y = self.map.height() - self.window.height();
		}

		self.window.position = window_position;
	}

	fn move_window_to_player_position(&mut self) {
		let player_position = self.player.position();
		let centered_position = Position {
			x: player_position.x.saturating_sub(self.window.width() / 2), 
			y: player_position.y.saturating_sub(self.window.height() / 2), 
		};

		self.move_window_to_position(&centered_position);
	}
}

use std::default::Default;

impl Default for World {
	fn default() -> Self {
		Self::new(300, 300, 100, 40)
	}
}

impl Interact for World {
	fn handle_input(&mut self, input: &char) -> Request {
		let backspace = &'\x08';

		if input == backspace {
			self.popups.add_popup(Exit::new());
		}

		if self.popups.is_empty() {
			match input {
				'w' => {
					self.move_player_in_direction(&Direction::Up);
					self.move_window_to_player_position();
				}

				's' => {
					self.move_player_in_direction(&Direction::Down);
					self.move_window_to_player_position();
				}

				'a' => {
					self.move_player_in_direction(&Direction::Left);
					self.move_window_to_player_position();
				}

				'd' => {
					self.move_player_in_direction(&Direction::Right);
					self.move_window_to_player_position();
				}

				'e' => {
					self.popups.add_popup(Message::new("*Inventory*", 15, 5));
				}

				'i' => {
					let tile_in_player_heading = self.map.tile_at_position(&self.player.position_in_heading());
					let max_width = self.window.width() - 4;
					let max_height = self.window.height() - 2;

					self.popups.add_popup(Info::new(&tile_in_player_heading.info(), max_width, max_height));
				}

				'm' => {
					self.popups.add_popup(More::new("More?", 13, 10));
				}

				_ => (),
			}
		} else {
			let request = self.popups.handle_input(input);
			
			match request {
				Request::Close => self.popups.close_popup(), 
				Request::Open(popup) => self.popups.add_popup(popup),
				Request::None => (), 
				Request::CloseGame => return Request::CloseGame,
		}
		}

		Request::None
	}
}

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

impl From<&World> for Frame {
	fn from(world: &World) -> Self {
		let mut frame = Frame::new();

		for row in 0..world.window.height() {
			frame.add_row();

			for col in 0..world.window.width() {
				let tile_position = Position::new(world.window.position.x + col, world.window.position.y + row);

				frame.add_to_last_row(
					world.map.tile_at_position(&tile_position).to_string()
				);
			}
		}

		let player_position = world.player.position();
		let x = player_position.x - world.window.position.x;
		let y = player_position.y - world.window.position.y;

		frame[y][x] = world.player.to_string();

		let player_selection_position = world.player.position_in_heading();
		let x = player_selection_position.x - world.window.position.x;
		let y = player_selection_position.y - world.window.position.y;

		let player_selection_string = mask(&frame[y][x]);
		frame[y][x] = player_selection_string;

		for popup in &world.popups {
			let popup_frame = popup.as_frame();
			let centered_position = centered_position(world.window.width(), world.window.height(), popup.width(), popup.height());

			frame.add_overlay(popup_frame, &centered_position);
		}

		frame
	}
}

impl fmt::Display for World {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let frame = Frame::from(self);
		write!(f, "{}", frame)
	}
}

fn centered_position(super_width: usize, super_height: usize, sub_width: usize, sub_height: usize) -> Position {
	assert!(super_width >= sub_width);
	assert!(super_height >= sub_height);

	Position {
		x: super_width / 2 - sub_width / 2,
		y: super_height / 2 - sub_height / 2, 
	}
}

struct Map {
	tiles: Vec<Vec<Box<dyn Tile>>>, 
}

impl Map {
	fn tile_at_position(&self, position: &Position) -> &Box<dyn Tile> {
		&self.tiles[*position]
	}
}

impl Grid for Map {
	fn width(&self) -> usize {
		self.tiles[0].len()
	}

	fn height(&self) -> usize {
		self.tiles.len()
	}
}

struct MapMaker;

impl MapMaker {
	fn lines(width: usize, height: usize) -> Map {
		let mut tiles: Vec<Vec<Box<dyn Tile>>> = Vec::with_capacity(height);

		for row in 0..height {
			tiles.push(Vec::with_capacity(width));

			for col in 0..width {
				tiles[row].push(
					match (col, row) {
						(0, _) | (_, 0) => Box::new(Barrier), 
						(c, _) if c == width - 1 => Box::new(Barrier),
						(_, r) if r == height - 1 => Box::new(Barrier),
						(c, _) if is_odd(c) && c < 20 => Box::new(Blinker::new()), 
						_ => Box::new(Grass),
					}
				);
			}
		}

		Map {
			tiles, 
		}
	}
}
 
struct Window {
	width: usize, 
	height: usize, 
	position: Position, 
}

/*impl Window {
	fn position(&self) -> &Position {
		&self.position
	}
}*/

impl Grid for Window {
	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}
}

fn is_odd(number: usize) -> bool {
	number & 1 == 1
}

struct Player {
	position: Position, 
	heading: Direction,
}

impl Player {
	fn new(x_position: usize, y_position: usize) -> Self {
		Self {
			position: Position::new(x_position, y_position),
			heading: Direction::Down, 
		}
	}

	fn position(&self) -> &Position {
		&self.position
	}

	fn heading(&self) -> &Direction {
		&self.heading
	}

	fn position_in_heading(&self) -> Position {
		self.position.position_in_direction(self.heading())
	}

	fn move_in_direction(&mut self, direction: &Direction) {
		self.position.move_in_direction(direction);
		self.heading = *direction;
	}

	/*fn as_frame(&self) -> Frame {
		let mut frame = Frame::new();
		
		for _ in 0..3 {
			frame.add_row();

			for _ in 0..3 {
				frame.add_to_last_row(" ");
			} 
		}

		frame[1][1] = player.to_string();

		match player.heading() {
			Direction::Up => frame[0][1] = 
	}*/

}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", "\x1b[38;2;0;0;0m\x1b[48;2;220;220;250m^")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::{Instant, Duration};

	pub fn lighten(color_string: &str) -> String {
		let mut lightened_string = String::new();

		for c in color_string.split_inclusive("m").map(|c| c.split(";").collect::<Vec<&str>>()) {
			match c[..] {
				[prefix, _, r, g, b] => {
					let lightening = 20;
					let r: u8 = r.parse::<u8>().unwrap().saturating_mul(lightening);
					let g: u8 = g.parse::<u8>().unwrap().saturating_mul(lightening);
					let b: u8 = b.strip_suffix("m").unwrap().parse::<u8>().unwrap().saturating_mul(lightening);

					lightened_string = format!("{lightened_string}{prefix};2;{r};{g};{b}m");
				}

				[any] => lightened_string = format!("{lightened_string}{any}"),
				_ => ()
			}
		}

		lightened_string
	}

	#[test]
	fn test_frame_to_string() {
		let trials = 1000;
		let mut sum = Duration::ZERO;

		for _ in 0..trials {
			//let world = World::new(101, 101, 101, 101);

			let now = Instant::now();

			//world.to_string();
			assert!("\x1b[38;2;100;100;100m\x1b[48;2;255;255;255m^".to_string() == lighten("\x1b[38;2;0;0;0m\x1b[48;2;220;220;250m^"));

			sum += now.elapsed();
		}

		let average = sum / trials;
		println!("average elapsed time: {:.2?}", average);
	}
}






 
    
    
    
    
    
    


