use super::{Request,Response,ReqErr};

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;
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
	let req_info: Vec<&str> = input.split_whitespace().collect();

	//Step 1: Check if valid request (400 Bad Request)
	if req_info.len() >= 3 &&
		req_info[0] == "GET"     &&
		req_info[1][0..1] == *"/" &&
		req_info[2].contains("HTTP") {

		//Step 2: Check if file exists (404 Not Found error)
		let mut path_string = String::new();
		let env_path = env::current_dir().unwrap();
		path_string.push_str(&env_path.display().to_string());
		path_string.push_str(req_info[1]);

		let path_str = path_string.clone();

		let path = Path::new(&path_str);
		if(path.exists()) {

			//Step 3: Check whether file is not off limits (403 Forbidden)
			let mut file = File::open(path_string);
			match file {
				Ok(mut f) => {
					return Ok(generate_response(&mut f, &req_info)); 
				},
				Err(_) => {
					return Err(ReqErr::Err_403);
				}
			}

		} else {
			return Err(ReqErr::Err_404);
		}

	}
	//400 Bad Request: improperly formatted GET command
	else {
		return Err(ReqErr::Err_400);
	}
}

fn generate_response(file: &mut File, req_info: &Vec<&str>) -> Response {
	let mut file_contents = String::new();
	let bytes_read = file.read_to_string(&mut file_contents).unwrap();

	let mut content_type = String::new();
	content_type.push_str("text/");
	if req_info[1].contains(".html") {
		content_type.push_str("html");
	} else {
		content_type.push_str("plain");
	}

	let protocol = req_info[2];
	let web_server_name = req_info[4];

	Response {
		protocol: protocol.to_owned(),
		status_message: "200 OK".to_owned(),
		web_server_name: web_server_name.to_owned(),
		content_type: content_type,
		content_length: bytes_read,
		file_content: file_contents,
	}
}