use std::fmt;

pub trait Tile: fmt::Display {
	fn is_walkable(&self) -> bool {
		true
	}
	fn info(&self) -> String;
}

// pub trait WorldTile: Tile {}

// pub trait Spread {
// 	fn spread(&self) -> usize;
// }

// pub trait Item {
// 	fn r#use(&self, on_tile: &mut Box<dyn Tile>);
// }

// pub struct WorldTile {
// 	building: Option<Box<dyn Building>>, 
// 	tile: Box<dyn Tile>, 
// }

// trait Building {
// 	fn transparent(&self) -> bool {
// 		false
// 	}
// }

// trait Tile: fmt::Display {
// 	fn 

// struct Dirt;

// struct WetDirt;

// trait WaterSpread {
// 	fn on_water_spread(self);
// }

// trait GrassSpread {
// 	fn on_grass_spread(self);

// impl WaterSpread for Dirt {
// 	fn on_water_spread(self) {
// 		self = WetDirt;
// 	}
// }

// struct Dirt {
// 	planted: Option<Box<dyn Plant>>
// };

// enum Grass {
// 	Seedling,
// 	PartiallyGrown, 
// 	FullyGrown,
// 	Wilted,
// } 

// enum PlantStatus {
// 	Alive, 
// 	Dead, 
// }

// impl Plant for Grass {
// 	fn on_harvest(&self) -> Vec<Box<dyn Item>> {
// 		match self {
// 			Grass::Seedling => GrassSeed, 
// 			Grass::PartiallyGrown => GrassSeed, 
// 			Grass::FullyGrown => GrassSeed, 
// 		}
// 	}

// 	fn grow(self) -> PlantStatus {
// 		match &self {
// 			Grass::Seedling => *self = Grass::PartiallyGrown, 
// 			Grass::PartiallyGrown => *self = Grass::FullyGrown, 
// 			Grass::FullyGrown => *self = Grass::FullyGrown, 
// 		}
// 	}
// }

// struct Storage {
// 	items: Vec<Box<dyn Items>>,
// }

// struct Inventory

// trait Plant {
// 	fn on_harvest(&self) -> Vec<Box<dyn Items>>;
// 	fn grow(self);
// }

// trait Seed {
// 	fn plant(&self, on_tile: &mut Box<dyn Tile>) {

// impl Plant for Grass {
// 	fn on_harvest

// struct Dirt {
// 	plant: Box<dyn Plant>
// 	fn is_plantable(&self) -> bool {
// 		true
// 	}

// 	fn 

// enum Tool {
// 	Shovel, 
// 	Hoe, 
// 	Bucket {
// 		holding_water: bool, 
// 	}, 
// }

// impl Item

// impl UseTool for Dirt {
// 	fn on_use(&self, 

// struct Wheat;

//impl DynamicTile for Wheat {}


/*
 | | | | | |
    ^       
 | | | | | |
*/
/*
struct Inventory {
	items: Vec<Vec<Option<Box<dyn Item>>>>,
	cursor_position: Position, 
	in_hand_position: Position, 
	frame: Frame, 
}

impl Inventory {
	fn move_cursor_in_direction(&mut self, direction: &Direction) {
		if self.cursor_can_move_in_direction(direction) {
			self.cursor_position.move_in_direction(direction);
		}
	}

	fn inventory_width(&self) -> usize {
		self.items[0].len()
	}

	fn inventory_height(&self) -> usize {
		self.items.len()
	}

	fn cursor_can_move_in_direction(&self, direction: &Direction) -> bool {
		let new_cursor_position = self.cursor_position.position_in_direction(direction);

		if !(0..self.inventory_width()).contains(&new_cursor_position.x) && !(0..self.inventory_height()).contains(&new_cursor_position.y) {
			return false
		}

		if self.items[new_cursor_position.y][new_cursor_position.x].is_none() {
			return false
		}

		true
	}

	fn update_in_hand(&mut self) {
		self.in_hand_position = *self.cursor_position;
	}

	fn item_in_hand(&self) -> Option<&Box<dyn Item>> {
		self.items[self.in_hand_position.y][new_cursor_position.x]
	}
}	


impl Interact for Inventory {
	fn handle_input(&mut self, input: &char) -> Request {
		match input {
			'w' => self.move_cursor_in_direction(Direction::Up),
			's' => self.move_cursor_in_direction(Direction::Down),
			'a' => self.move_cursor_in_direction(Direction::Left),
			'd' => self.move_cursor_in_direction(Direction::Right),
		}
	}
}*/


pub struct Grass;

impl fmt::Display for Grass {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[38;2;0;0;0m\x1b[48;2;124;252;0m.")
	}
}

impl Tile for Grass {
	fn info(&self) -> String {
		String::from("Grassy field")
	}
}

pub struct Dirt;

impl fmt::Display for Dirt {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[38;2;0;0;0m\x1b[48;2;160;82;45m.")
	}
}

impl Tile for Dirt { 
	fn info(&self) -> String {
		String::from("Literally just dirt")
	}
}

pub struct Barrier;

impl fmt::Display for Barrier {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[38;2;255;255;255m\x1b[48;2;255;10;10mx")
	}
}

impl Tile for Barrier { 
	fn is_walkable(&self) -> bool {
		false
	}
	
	fn info(&self) -> String {
		String::from("impermeable world barrier")
	}
}

pub struct Fence;

impl fmt::Display for Fence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\x1b[38;2;80;41;23m\x1b[48;2;160;82;45m|")
	}
}

impl Tile for Fence {
	fn is_walkable(&self) -> bool {
		false
	}

	fn info(&self) -> String {
		String::from("fence")
	}
}

use std::cell::RefCell;

pub struct Blinker(RefCell<bool>);

impl Blinker {
	pub fn new() -> Self {
		Self(RefCell::new(false))
	}

	fn status(&self) -> bool {
		*self.0.borrow()
	}

	fn swap(&self) {
		self.0.replace_with(|&mut old| !old);
	}
}

impl fmt::Display for Blinker {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.swap();

		if self.status() {
			return write!(f, "\x1b[48;2;50;50;50m ")
		} else {
			return write!(f, "\x1b[48;2;0;0;0m ")
		}
	}
}

impl Tile for Blinker {
	fn info(&self) -> String {
		String::from("why it doing that")
	}
}