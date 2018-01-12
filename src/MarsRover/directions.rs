use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub enum Directions {
	N,
	S,
	E,
	W
}

impl Directions {
	pub fn opposite(&self) -> Directions {
		match *self {
			Directions::N => Directions::S,
			Directions::S => Directions::N,
			Directions::E => Directions::W,
			Directions::W => Directions::E,
		}
	}

	pub fn left(&self) -> Directions {
		match *self {
			Directions::N => Directions::W,
			Directions::S => Directions::E,
			Directions::E => Directions::N,
			Directions::W => Directions::S,
		}
	}

	pub fn right(&self) -> Directions {
		match *self {
			Directions::N => Directions::E,
			Directions::S => Directions::W,
			Directions::E => Directions::S,
			Directions::W => Directions::N,
		}
	}
}

impl Default for Directions {
	fn default() -> Directions {
		Directions::N
	}
}

impl FromStr for Directions {
	type Err = String;

	fn from_str(d: &str) -> Result<Self, Self::Err> {
		match d {
			"N" => Ok(Directions::N),
			"S" => Ok(Directions::S),
			"E" => Ok(Directions::E),
			"W" => Ok(Directions::W),
			_ => Err("error: unrecognized direction ".to_owned() + d)
		}
	}
}

impl ToString for Directions {
	fn to_string(&self) -> String {
		match *self {
			Directions::N => "N".to_owned(),
			Directions::S => "S".to_owned(),
			Directions::E => "E".to_owned(),
			Directions::W => "W".to_owned(),
		}
	}
}
