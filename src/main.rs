use std::fmt;
use std::thread;
use std::fs::File;
use std::error::Error;
use std::sync::{Arc,Mutex};
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write,BufReader,BufRead,BufWriter};

mod ReqHandler;

//with the help of https://dfockler.github.io/2016/05/20/web-server.html

pub struct Request {
	method: String,
	path_to_file: String,
	protocol: String,
}

pub struct Response {
	protocol: String,
	status_message: String,
	web_server_name: String,
	content_type: String,
	content_length: usize,
	file_content: String,
}

pub enum ReqErr {
	Err_400,
	Err_403,
	Err_404,
}

fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Listening on port 8080...");

	let log_file_raw = File::create("logs/log.txt").unwrap();
	let log_file = Arc::new(Mutex::new(log_file_raw));
	println!("Log file created in /logs/log.txt");

	for stream in listener.incoming() {
		println!("------------------------------");
		println!("New connection, thread spawned");

		match stream {
			Ok(mut stream) => {
				handle_request(&mut stream);
			}
			Err(_) => {
				println!("Connection failed! Try again later.");
			}
		}
	}
}

fn handle_request(stream: &mut TcpStream) {
	//get req (by line) from stream
	let stream_contents = ReqHandler::read_stream(stream);
	// println!("{}", stream_contents);

	//if no error, lock and modify log file then print response to stream 
	//otherwise, print error response
	match ReqHandler::validate_request(stream_contents) {
		Ok(response) => {
			println!("{}", response);
		},
		Err(err) => {
			println!("{}", err);
		}
	}
	
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    	write!(f, "{} {}\n{}\nContent-type: {}\nContent-length: {}\n\n{}",
    				self.protocol,
		        	self.status_message,
		        	self.web_server_name,
		        	self.content_type,
		        	self.content_length,
		        	self.file_content)
        
    }
}

impl fmt::Display for ReqErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ReqErr::Err_400 => "400 Bad Request",
            ReqErr::Err_403 => "403 Forbidden",
            ReqErr::Err_404 => "404 Not Found",
        };
        write!(f, "{}", printable)
    }
}
