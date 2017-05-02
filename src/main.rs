/*
* webserver
* a rudimentary web server
* Responds to a single command of HTTP/0.9,
* the GET method, with the follwing shape:
* GET /path/to/file HTTP
* Spawns a thread that retrieves the request,
* Records it to a log file, and generates a response
*
* Assumptions
* 1) Assumes filepath is valid
*	- Assumes file path begins with /
*	- Assumes file path matches traditional file name regex
*	- Will return 400 Bad Request if access to directory is attempted
* 	- Will return 403 Forbidden if a compressed file is attempted
*/

use std::fmt;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc,Mutex};
use std::net::{TcpListener,TcpStream};

use chrono::prelude::*;

extern crate chrono;
extern crate regex;

mod req_handler;

pub struct Response {
	protocol: String,
	status_message: String,
	web_server_name: String,
	content_type: String,
	content_length: usize,
	file_content: String,
}

pub enum ReqErr {
	Err400,
	Err403,
	Err404,
}

fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Listening on port 8080...");

	//creates log file
	let log_file_raw = File::create("logs/log.txt").unwrap();
	let log_file = Arc::new(Mutex::new(log_file_raw));
	println!("Log file created in /logs/log.txt");

	for stream in listener.incoming() {

		match stream {
			Ok(mut stream) => {
				let mut log_file = log_file.clone();

				//spawns new thread
				thread::spawn (move || {
					println!("------------------------------");
					println!("New connection, thread spawned");
					//handles incoming request
					handle_request(&mut stream, &mut log_file);
				});
			}
			Err(_) => {
				println!("Connection failed! Try again later.");
			}
		}
	}
}

/* handle_request */
// takes in a TcpStream and log file
// checks whether request is valid
// writes results back to stream
fn handle_request(stream: &mut TcpStream, log_file: &mut Arc<Mutex<File>>) {
	//get req (by line) from stream
	let stream_contents = req_handler::read_stream(stream);
	let req_info: Vec<&str> = stream_contents.split_whitespace().collect();


	//if no error, lock and modify log file then print response to stream 
	//otherwise, print error response
	match req_handler::validate_request(&req_info) {
		Ok(response) => {
			log_info(&req_info, log_file, "200 OK\n");
			&stream.write_all(response.to_string().as_bytes()).unwrap();
		},
		Err(err) => {
			log_info(&req_info, log_file, &err.to_string());
			&stream.write_all(err.to_string().as_bytes()).unwrap();
		}
	}
	
}

/* log_info */
// takes in request info, log file, and response
// locks log file, generates log info (datetime, request, response),
// and logs it to the file			
fn log_info(req_info: &Vec<&str>, log_file: &mut Arc<Mutex<File>>, response: &str) {
	let mut log_guard = log_file.lock().unwrap();
	let mut log_info = String::new();

	//Get current time
	let time: DateTime<UTC> = UTC::now();
	let time_str = time.format("%Y-%m-%d %H:%M:%S").to_string();

	log_info.push_str(&time_str);
	log_info.push_str("  -  ");

	//only push GET, filepath, and protocol
	for i in 0..3 {
		if req_info.len() >= i {
			log_info.push_str(req_info[i]);
			log_info.push_str(" ");
		}
	}

	log_info.push_str(" -  ");

	//Response
	log_info.push_str(response);

	//write to log file
	write!(&mut log_guard, "{}", log_info).expect("Unable to write data to log file");

}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    	write!(f, "{} {}\n{}\nContent-type: {}\nContent-length: {}\n\n{}\n",
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
            ReqErr::Err400 => "400 Bad Request",
            ReqErr::Err403 => "403 Forbidden",
            ReqErr::Err404 => "404 Not Found",
        };
        write!(f, "{}\n", printable)
    }
}
