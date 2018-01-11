extern crate sphericalmercator;

mod directions;

use self::sphericalmercator::Coordinate;

use self::directions::Directions as Directions;

//degrees equivalent to 1km on mars surface
const MOVEMENT:f64 = 0.02712621553502488;

#[derive(Default, PartialEq)]
pub struct MarsRover {
	pub position: Coordinate,
	pub direction: Directions
}

impl MarsRover {
	pub fn new() -> MarsRover {
		Default::default()
	}

	pub fn forward(&mut self) -> Result<Coordinate, String> {
		if self.check_for_obstacles(&self.direction) {
			return Err("Obstacle detected, aborting".to_owned());
		}

		match self.direction {
			Directions::N => self.position.y += MOVEMENT,
			Directions::S => self.position.y -= MOVEMENT,
			Directions::E => self.position.x += MOVEMENT,
			Directions::W => self.position.x -= MOVEMENT,
		}

		Ok(self.position)
	}

	pub fn backward(&mut self) -> Result<Coordinate, String> {
		if self.check_for_obstacles(match self.direction {
			Directions::N => &Directions::S,
			Directions::S => &Directions::N,
			Directions::E => &Directions::W,
			Directions::W => &Directions::E,
		}) {
			return Err("Obstacle detected, aborting".to_owned());
		}

		match self.direction {
			Directions::N => self.position.y -= MOVEMENT,
			Directions::S => self.position.y += MOVEMENT,
			Directions::E => self.position.x -= MOVEMENT,
			Directions::W => self.position.x += MOVEMENT,
		}

		Ok(self.position)
	}

	pub fn left(&mut self) -> Result<Coordinate, String> {
		self.direction = match self.direction {
			Directions::N => Directions::W,
			Directions::S => Directions::E,
			Directions::E => Directions::N,
			Directions::W => Directions::S,
		};

		Ok(self.position)
	}

	pub fn right(&mut self) -> Result<Coordinate, String> {
		self.direction = match self.direction {
			Directions::N => Directions::E,
			Directions::S => Directions::W,
			Directions::E => Directions::S,
			Directions::W => Directions::N,
		};

		Ok(self.position)
	}

	/**
	 * Actually this functions is intended to be pseudo-random
	 */
	fn check_for_obstacles(&self, d: &Directions) -> bool {
		let (x, y) = match *d {
			Directions::N => (self.position.x, self.position.y + MOVEMENT),
			Directions::S => (self.position.x, self.position.y - MOVEMENT),
			Directions::E => (self.position.x + MOVEMENT, self.position.y),
			Directions::W => (self.position.x - MOVEMENT, self.position.y),
		};

		(x.sin() - y.cos()).abs() < 0.25
	}
}

impl ToString for MarsRover {
	fn to_string(&self) -> String {
		format!("position: {} {} direction: {}", self.position.x, self.position.y, self.direction.to_string())
	}
}
