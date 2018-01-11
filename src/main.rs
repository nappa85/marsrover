extern crate sphericalmercator;
extern crate hyper;
extern crate futures;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use futures::future::Future;
use futures::Stream;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

use sphericalmercator::Coordinate;

use std::env;
use std::sync::{Arc, Mutex};
use std::str::FromStr;

//degrees equivalent to 1km on mars surface
const MOVEMENT:f64 = 0.02712621553502488;

#[derive(PartialEq)]
pub enum Directions {
	N,
	S,
	E,
	W
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

#[derive(Default, PartialEq)]
pub struct MarsRover {
	pub position: Coordinate,
	pub direction: Directions
}

impl MarsRover {
	fn new() -> MarsRover {
		Default::default()
	}

	fn forward(&mut self) -> Result<Coordinate, String> {
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

	fn backward(&mut self) -> Result<Coordinate, String> {
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

	fn left(&mut self) -> Result<Coordinate, String> {
		self.direction = match self.direction {
			Directions::N => Directions::W,
			Directions::S => Directions::E,
			Directions::E => Directions::N,
			Directions::W => Directions::S,
		};

		Ok(self.position)
	}

	fn right(&mut self) -> Result<Coordinate, String> {
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

//singleton
lazy_static! {
    static ref ROVER: Arc<Mutex<MarsRover>> = Arc::new(Mutex::new(MarsRover::new()));
}

pub struct WebServer;

impl Service for WebServer {
	// boilerplate hooking up hyper's server types
	type Request = Request;
	type Response = Response;
	type Error = hyper::Error;
	// The future representing the eventual Response your call will
	// resolve to. This can change to whatever Future you need.
	type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

	fn call(&self, req: Request) -> Self::Future {
		match (req.method(), req.path()) {
			(&Method::Post, "/move") => {
				Box::new(req.body().concat2().map(|chunks| {
					let body = String::from_utf8(chunks.to_vec()).unwrap();

					let temp = ROVER.clone();
					let mut rover = temp.lock().unwrap();
					let mut result: Result<Coordinate, String>;

					for command in body.chars() {
						match command {
							'f' => result = rover.forward(),
							'b' => result = rover.backward(),
							'l' => result = rover.left(),
							'r' => result = rover.right(),
							_ => {
								return Response::new().with_body("unrecognized command ".to_owned() + &command.to_string());
							}
						}

						match result {
							Ok(_) => {},
							Err(error) => {
								return Response::new().with_body("Error: ".to_owned() + &error);
							}
						}
					}

					Response::new().with_body(rover.to_string())
				}))
			},
			_ => {
				Box::new(futures::future::ok(
					Response::new().with_status(StatusCode::NotFound)
				))
			},
		}
	}
}

fn main() {
	//launch example: 12.34567890 0.987654321 N
	let args:Vec<String> = env::args().collect();
	assert!(args.len() == 4, "Usage: <latitude> <longitude> <direction>");

	//let the parser validate values itself
	{
		let temp = ROVER.clone();
		let mut rover = temp.lock().unwrap();
		rover.position.x = args.get(1).unwrap().parse().unwrap();
		rover.position.y = args.get(2).unwrap().parse().unwrap();
		rover.direction = args.get(3).unwrap().parse().unwrap();
	}

	//start webserver
	let addr = "127.0.0.1:3000".parse().unwrap();
	let server = Http::new().bind(&addr, || Ok(WebServer)).unwrap();
	server.run().unwrap();
}
