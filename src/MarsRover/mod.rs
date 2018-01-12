extern crate sphericalmercator;

mod directions;

use self::sphericalmercator::Coordinate;

use self::directions::Directions as Directions;

use std::f64;

//degrees equivalent to 1km on mars surface
const MOVEMENT:f64 = 360000.0 / (2.0 * f64::consts::PI * sphericalmercator::A);

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
			return Err("Obstacle detected, aborting!".to_owned());
		}

		match self.direction {
			Directions::N => self.position.y += MOVEMENT,
			Directions::S => self.position.y -= MOVEMENT,
			Directions::E => self.position.x += MOVEMENT,
			Directions::W => self.position.x -= MOVEMENT,
		}

		self.check_edge_wrapping();

		Ok(self.position)
	}

	pub fn backward(&mut self) -> Result<Coordinate, String> {
		if self.check_for_obstacles(&self.direction.opposite()) {
			return Err("Obstacle detected, aborting!".to_owned());
		}

		match self.direction {
			Directions::N => self.position.y -= MOVEMENT,
			Directions::S => self.position.y += MOVEMENT,
			Directions::E => self.position.x -= MOVEMENT,
			Directions::W => self.position.x += MOVEMENT,
		}

		self.check_edge_wrapping();

		Ok(self.position)
	}

	pub fn left(&mut self) -> Result<Coordinate, String> {
		self.direction = self.direction.left();

		Ok(self.position)
	}

	pub fn right(&mut self) -> Result<Coordinate, String> {
		self.direction = self.direction.right();

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

	fn check_edge_wrapping(&mut self) {
		//going north you'll find yourself going south on the opposite side
		if self.position.y > sphericalmercator::MAXEXTENT {
			self.position.x += sphericalmercator::MAXEXTENT;
			self.position.y = -sphericalmercator::MAXEXTENT + (self.position.y - sphericalmercator::MAXEXTENT);
			self.direction = self.direction.opposite();
		}
		if self.position.y < -sphericalmercator::MAXEXTENT {
			self.position.x += sphericalmercator::MAXEXTENT;
			self.position.y = sphericalmercator::MAXEXTENT + (self.position.y + sphericalmercator::MAXEXTENT);
			self.direction = self.direction.opposite();
		}
		//going west you keep going west
		if self.position.x > sphericalmercator::MAXEXTENT {
			self.position.x = -sphericalmercator::MAXEXTENT + (self.position.x - sphericalmercator::MAXEXTENT);
		}
		if self.position.x < -sphericalmercator::MAXEXTENT {
			self.position.x = sphericalmercator::MAXEXTENT + (self.position.x + sphericalmercator::MAXEXTENT);
		}
	}
}

impl ToString for MarsRover {
	fn to_string(&self) -> String {
		format!("position: {} {}\ndirection: {}", self.position.x, self.position.y, self.direction.to_string())
	}
}
