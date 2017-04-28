use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufRead};

fn main() {

	let tcp_listen = TcpListener::bind("127.0.0.1:8080").unwrap();
	let stream = tcp_listen.accept().unwrap().0;

	manage_req(stream);
}

fn manage_req(stream: TcpStream) {

	let mut bufreader = BufReader::new(stream);

	for line in bufreader.by_ref().lines() {
        if line.unwrap() == "" {
            break;
        }
    }

	send_res(bufreader.into_inner());
}

fn send_res(mut stream: TcpStream) {
	let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}