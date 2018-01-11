extern crate hyper;

#[macro_use]
extern crate lazy_static;

mod marsrover;
mod webserver;

use hyper::server::Http;

use std::env;
use std::sync::{Arc, Mutex};

use marsrover::MarsRover as MarsRover;
use webserver::WebServer as WebServer;

//singleton
lazy_static! {
    static ref ROVER: Arc<Mutex<MarsRover>> = Arc::new(Mutex::new(MarsRover::new()));
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
