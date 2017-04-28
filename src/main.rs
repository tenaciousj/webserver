use std::thread;
use std::fs::File;
use std::sync::{Arc,Mutex};
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write,BufReader,BufRead,BufWriter};

mod reqhandler;
mod reshandler;

//with the help of https://dfockler.github.io/2016/05/20/web-server.html

//In return to a valid GET request, the web server spawns a thread that retrieves the request, records it to a log file, and generates a response. 



fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Listening on port 8080...");

	let log_file_raw = File::create("logs/log.txt").unwrap();
	let log_file = Arc::new(Mutex::new(log_file_raw));
	println!("Log file created in /logs/log.txt");

	for stream in listener.incoming() {
		let log_file = log_file.clone();
		println!("New connection, thread spawned");

		match stream {
			Ok(mut stream) => {
				handle_client(&mut stream);
			}
			Err(e) => {
				println!("Connection failed! Try again later...");
			}
		}

	}
}

fn handle_client(stream: &mut TcpStream) {
	//get req (by line) from stream
	//check for 400 bad request error
	//then check if file exists (404 Not Found error)
	//then check whether it is off limits (403 Forbidden)
	//if no error, lock and modify log file then print response to stream 
	//otherwise, print error response
}
