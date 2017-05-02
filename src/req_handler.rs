use super::{Response,ReqErr};

use std::env;
use std::io::Read;
use std::fs::File;
use std::fs::read_dir;
use std::path::Path;
use std::net::TcpStream;


use regex::Regex;

const WEB_SERVER_NAME: &'static str = "jrp338-kqj094-web-server/0.1";


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
// TODO: handle zips
pub fn validate_request(req_info: &Vec<&str>) -> Result<Response, ReqErr> {

	//regex to match file name
	// TODO: fix handling just a directory 
	let re = Regex::new(r"^(/[\w\d-_]+)*/[\w\d-_]+").unwrap();

	//Step 1: Check if valid request
	//Check if it is a GET request,
	//whether file path is really a file path,
	//and whether the protocol is HTTP
	if req_info.len() >= 3 &&
		req_info[0] == "GET"     &&
		re.is_match(req_info[1]) &&
		req_info[2].starts_with("HTTP") {
		//Step 2: Check if file exists
		//generate path with environment's current directory
		let mut path_string = String::new();
		let env_path = env::current_dir().unwrap();

		path_string.push_str(&env_path.display().to_string());
		path_string.push_str(req_info[1]);

		// this is a compressed file, cannot open
		if path_string.contains(".zip") || path_string.contains(".7z") {
			//(403 Forbidden)
			return Err(ReqErr::Err403);
		}

		let path = Path::new(&path_string);
		println!("{}", path_string);
		if path.exists() {
			// Step 3: Check if it's a file or directory
			if path.is_file() {
				//Step 3: Check whether file is not off limits 
				let file = File::open(&path_string);
				match file {
					Ok(mut f) => {
						//200 Ok! Create response
						return Ok(generate_response(&mut f, &req_info)); 
					},
					Err(_) => {
						
					}
				}
			} else if path.is_dir() {
				if let Ok(dir_entries) = read_dir(path) {
					// check for index files 
					for dir_entry in dir_entries {
						if let Ok(entry) = dir_entry {
							let file_type_result = entry.file_type();
							if let Ok(file_type) = file_type_result {
								if file_type.is_file() {
									let entry_name = entry.file_name();
									let entry_str = entry_name.to_str().unwrap();
									if entry_str == "index.html" || 
									    entry_str == "index.shtml" || 
									    entry_str == "index.txt" {
										let file = File::open(&entry.path().as_path());
										match file {
											Ok(mut f) => {
												//200 Ok! Create response
												return 
												Ok(generate_response(&mut f, &req_info)); 
											},
											Err(_) => {
												//(403 Forbidden)
												return Err(ReqErr::Err403);
											}
										}
									}
								}
							}
						}
					}

					// return 404 if not found
					return Err(ReqErr::Err404);
				} else {

					// since we already checked if the path exists
					// and it's a directory, that means that any error
					// comes from forbidden access to directory
					return Err(ReqErr::Err403);					
				}

				
			}	
		} 
		return Err(ReqErr::Err404)


	}

	else {
		//(400 Bad Request)
		return Err(ReqErr::Err400);
	}
}

/* generate_response */
//takes in the file to be read, request info
//returns a Response
//generates response to be written onto stream
fn generate_response(file: &mut File, req_info: &Vec<&str>) -> Response {
	let mut file_contents = String::new();
	let bytes_read = file.read_to_string(&mut file_contents).unwrap();

	//checks whether content is html or plain
	let mut content_type = String::new();
	content_type.push_str("text/");
	if req_info[1].contains(".html") {
		content_type.push_str("html");
	} else {
		content_type.push_str("plain");
	}

	//should be some variant of HTTP
	let protocol = req_info[2];

	Response {
		protocol: protocol.to_owned(),
		status_message: "200 OK".to_owned(),
		web_server_name: WEB_SERVER_NAME.to_owned(),
		content_type: content_type,
		content_length: bytes_read,
		file_content: file_contents,
	}
}
