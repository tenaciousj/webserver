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

		//in case response does not take up all of buffer
		if bytes_read < 128 { break; }
	}

	contents
}

// I try 
// #[cfg(test)]
// mod read_stream_tests {
// 	use super::read_stream;
// 	use std::io::Write;
// 	use std::net::{TcpStream, TcpListener};

// 	#[test]
// 	fn test() {
// 		stream_assert();
// 	}
// 	fn stream_assert() {
// 		let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// 		if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
// 			let _ = stream.write("foobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfo\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}".as_bytes());
// 			let listen_stream = listener.incoming().next();
// 			let mut result = listen_stream.unwrap().unwrap();
// 			let output = read_stream(&mut result);
// 			assert_eq!("", output.as_str());
// 			// assert!(true);
// 		} else {
// 			assert!(false);
// 		}
// 	}

// }

/* validate_request */
// checks whether request is valid
// will return Response if request is valid
// will return ReqErr (400, 403, 404) otherwise
pub fn validate_request(req_info: &Vec<&str>) -> Result<Response, ReqErr> {

	//regex to match file name
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
				//Step 4: Check whether file is not off limits 
				let file = File::open(&path_string);
				match file {
					Ok(mut f) => {
						//200 Ok! Create response
						return Ok(generate_response(&mut f, &req_info)); 
					},
					Err(_) => {
						//(403 Forbidden)
						return Err(ReqErr::Err403); 
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
	} else {
		//(400 Bad Request)
		return Err(ReqErr::Err400);
	}
}

#[cfg(test)]
mod validate_request_tests {
	use super::{Response, ReqErr, validate_request,WEB_SERVER_NAME};
	#[test]
	fn empty_input() {
		validate_assert(&vec![], Err(ReqErr::Err400));
	}

	#[test]
	fn one_input() {
		validate_assert(&vec!["lol"], Err(ReqErr::Err400));
	}

	#[test]
	fn two_input() {
		validate_assert(&vec!["lol", "meow"], Err(ReqErr::Err400));
	}

	#[test]
	fn bad_get() {
		validate_assert(&vec!["get", "/src/", "HTTP"], Err(ReqErr::Err400));
	}

	#[test]
	fn bad_path() {
		validate_assert(&vec!["GET", "src/", "HTTP"], Err(ReqErr::Err400));
	}

	#[test]
	fn bad_proto() {
		validate_assert(&vec!["GET", "/src/", "http"], Err(ReqErr::Err400));
	}

	#[test]
	fn path_does_not_exist_file() {
		validate_assert(&vec!["GET", "/srcLOL/", "HTTP"], Err(ReqErr::Err404));
	}

	#[test]
	fn successful_file() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/plain".to_owned(),
			content_length: 4,
			file_content: "meow".to_owned(),
		};
		validate_assert(&vec!["GET", "/test/meow.txt", "HTTP"], Ok(response));
	}

	#[test]
	fn no_index_files() {
		validate_assert(&vec!["GET", "/test/", "HTTP"], Err(ReqErr::Err404));
	}

	#[test]
	fn successful_html_index() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/html".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		validate_assert(&vec!["GET", "/test/html/", "HTTP"], Ok(response));
	}

	#[test]
	fn successful_shtml_index() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/html".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		validate_assert(&vec!["GET", "/test/shtml/", "HTTP"], Ok(response));
	}

	#[test]
	fn successful_txt_index() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/plain".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		validate_assert(&vec!["GET", "/test/txt/", "HTTP"], Ok(response));
	}

	// Need to create forbidden files for forbiddden tests to work
	// #[test]
	// fn forbidden_access_file() {
	// 	validate_assert(&vec!["GET", "/test/locked_test.txt", "HTTP"], Err(ReqErr::Err403));
	// }
	// #[test]
	// fn forbidden_access_dir() {
	// 	validate_assert(&vec!["GET", "/test/forbidden/", "HTTP"], Err(ReqErr::Err403));
	// }
	// #[test]
	// fn forbidden_html_index() {
	// 	validate_assert(&vec!["GET", "/test/forbidden_html/", "HTTP"], Err(ReqErr::Err403));
	// }
	// #[test]
	// fn forbidden_shtml_index() {
	// 	validate_assert(&vec!["GET", "/test/forbidden_shtml/", "HTTP"], Err(ReqErr::Err403));
	// }
	// #[test]
	// fn forbidden_txt_index() {
	// 	validate_assert(&vec!["GET", "/test/forbidden_txt/", "HTTP"], Err(ReqErr::Err403));
	// }

	fn validate_assert(req: &Vec<&str>, expected: Result<Response, ReqErr>) {
		let output = validate_request(req);
		assert_eq!(expected, output);
	}
}


/* generate_response */
//takes in the file to be read, request info
//returns a Response
//generates response to be written onto stream
//does not validate req_info because this function is only called 
// by validate_request(), which handles validation
fn generate_response(file: &mut File, req_info: &Vec<&str>) -> Response {
	let mut file_contents = String::new();
	let bytes_read = file.read_to_string(&mut file_contents).unwrap();

	//checks whether content is html or plain
	let mut content_type = String::new();
	content_type.push_str("text/");
	if req_info[1].contains("html") {
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

#[cfg(test)]
mod generate_response_tests {
	use std::env;
	use std::fs::File;
	use super::{Response, generate_response, WEB_SERVER_NAME};

	#[test] 
	fn txt_response() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/plain".to_owned(),
			content_length: 4,
			file_content: "meow".to_owned(),
		};
		response_assert(&vec!["GET", "/test/meow.txt", "HTTP"], response);
	}

	#[test] 
	fn html_response() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/html".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		response_assert(&vec!["GET", "/test/html/index.html", "HTTP"], response);
	}
	#[test] 
	fn shtml_response() {
		let response = Response {
			protocol: "HTTP".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/html".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		response_assert(&vec!["GET", "/test/shtml/index.shtml", "HTTP"], response);
	}

	#[test] 
	fn http11_response() {
		let response = Response {
			protocol: "HTTP/1.1".to_owned(),
			status_message: "200 OK".to_owned(), 
			web_server_name: WEB_SERVER_NAME.to_owned(),
			content_type: "text/html".to_owned(),
			content_length: 0,
			file_content: "".to_owned(),
		};
		response_assert(&vec!["GET", "/test/shtml/index.shtml", "HTTP/1.1"], response);
	}

	fn response_assert(req_info: &Vec<&str>, expected: Response) {
		let mut path_string = String::new();
		let env_path = env::current_dir().unwrap();

		path_string.push_str(&env_path.display().to_string());
		path_string.push_str(req_info[1]);

		let file = File::open(&path_string);

		let output = generate_response(&mut file.unwrap(), req_info);
		assert_eq!(expected, output);

	}
}