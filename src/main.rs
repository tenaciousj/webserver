use std::thread;
use std::fs::File;
use std::sync::{Arc,Mutex};
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write,BufReader,BufRead,BufWriter};

//with the help of https://dfockler.github.io/2016/05/20/web-server.html

//In return to a valid GET request, the web server spawns a thread that retrieves the request, records it to a log file, and generates a response. 
fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	for stream in listener.incoming() {
		match stream {
	        Ok(stream) => {
	            handle_request(stream);
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}
}

fn handle_request(mut stream: TcpStream) {
	thread::spawn(move || {
		let mut bufreader = BufReader::new(stream);
		for line in bufreader.by_ref().lines() {
			if line.unwrap() == "" {break;}
        }
        send_res(bufreader.into_inner());
    });
}

fn send_res(mut stream: TcpStream) {
	let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}

// fn log_request(data: &str) {
// 	let file = File::create("log.txt").expect("Unable to create log file");
//     let mut fwriter = BufWriter::new(file);
//     fwriter.write_all(data.as_bytes()).expect("Unable to write data");
// }