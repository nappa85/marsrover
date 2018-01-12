extern crate sphericalmercator;
extern crate hyper;
extern crate futures;

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::sphericalmercator::Coordinate;

use ROVER;

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
				//concat every request's body chunk
				Box::new(req.body().concat2().map(|chunks| {
					//convert chunks to String
					let body = String::from_utf8(chunks.to_vec()).unwrap();

					//retrieve Rover singleton
					let temp = ROVER.clone();
					let mut rover = temp.lock().unwrap();
					let mut result: Result<Coordinate, String>;

					//treat String as Vec<char>
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
								return Response::new().with_body("Error: ".to_owned() + &error + "\n" + &rover.to_string());
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
