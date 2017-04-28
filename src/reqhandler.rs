use super::{Request,Response,ReqErr};

use std::io::Read;
use std::error::Error;
use std::net::TcpStream;


/* read_stream */
// takes in a TcpStream and reads contents into buffer
// returns contents in String format
pub fn read_stream(stream: &mut TcpStream) -> String {
	let mut buf = [0; 128];
	let mut contents = String::new();

	while let Ok(bytes_read) = stream.read(&mut buf) {
		let c = String::from_utf8(buf.to_vec()).unwrap();
		contents.push_str(&c);
		// println!("{}", bytes_read);

		//in case response does not take up all of buffer
		if bytes_read < 128 { break; }
	}

	contents
}

/* validate_request */
// checks whether request is valid
// will return Response if request is valid
// will return ReqErr (400, 403, 404) otherwise
pub fn validate_request(input: String) -> Result<Response, ReqErr> {
	let split_input: Vec<&str> = input.split_whitespace().collect();

	//if valid input
	if split_input.len() >= 3 &&
		split_input[0] == "GET"     &&
		split_input[1][0..1] == *"/" &&
		split_input[2].contains("HTTP") {
		//then check if file exists (404 Not Found error)
		//then check whether it is off limits (403 Forbidden)
		
		println!("Ok request!");
		return Ok(Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(),
			web_server_name: "test".to_owned(),
			content_type: "text/plain".to_owned(),
			content_length: 100,
		});
	}
	//400 Bad Request: improperly formatted GET command
	else {
		return Err(ReqErr::Err_400);
	}
}