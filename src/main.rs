use std::fmt;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc,Mutex};
use std::net::{TcpListener,TcpStream};

use chrono::prelude::*;
extern crate chrono;

mod req_handler;


//with the help of https://dfockler.github.io/2016/05/20/web-server.html

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

	let log_file_raw = File::create("logs/log.txt").unwrap();
	let log_file = Arc::new(Mutex::new(log_file_raw));
	println!("Log file created in /logs/log.txt");

	for stream in listener.incoming() {
		println!("------------------------------");
		println!("New connection, thread spawned");

		match stream {
			Ok(mut stream) => {
				let mut log_file = log_file.clone();
				thread::spawn (move || {
					handle_request(&mut stream, &mut log_file);
				});
			}
			Err(_) => {
				println!("Connection failed! Try again later.");
			}
		}
	}
}

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

					
fn log_info(req_info: &Vec<&str>, log_file: &mut Arc<Mutex<File>>, response: &str) {
	let mut log_guard = log_file.lock().unwrap();
	let mut log_info = String::new();

	let method = req_info[0];
	let path = req_info[1];
	let prot = req_info[2];

	//Get current time
	let time: DateTime<UTC> = UTC::now();
	let time_str = time.format("%Y-%m-%d %H:%M:%S").to_string();

	log_info.push_str(&time_str);
	log_info.push_str("  -  ");

	//Request
	log_info.push_str(method);
	log_info.push_str(" ");
	log_info.push_str(path);
	log_info.push_str(" ");
	log_info.push_str(prot);
	log_info.push_str("  -  ");

	//Response
	log_info.push_str(response);

	log_guard.write(log_info.as_bytes()).expect("Unable to write data to log file");

}
