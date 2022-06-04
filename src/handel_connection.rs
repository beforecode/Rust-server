use std::io::Read;
use std::io::Write;
use crate::TcpStream;
use std::fs;

pub fn run(mut stream: TcpStream) {
    // Read incoming requests.
    let mut buffer: Vec<u8> = vec![0; 1024];
    stream.read(&mut buffer).unwrap();

    //Checking request type
    let get_request: &[u8; 16] = b"GET / HTTP/1.1\r\n";


    let (res, contents) = handel_server_response(&get_request, &buffer);

    let response = format!(
    	"{}\r\nContent-Length: {}\r\n\r\n {}",
    	res,
    	contents.len(),
    	contents
    	);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn handel_server_response(getrequest: &[u8; 16], buf: &Vec<u8>) -> (String, String) {
	let mut res:String = String::new();
	let mut filename:String = String::new();
	if buf.starts_with(getrequest) {
		res = String::from("HTTP/1.1 200 OK");
		filename = fs::read_to_string("index.html").unwrap();
	} else {
		res = String::from("HTTP/1.1 404 NOT FOUND");
		filename = fs::read_to_string("404.html").unwrap();
	}
	(res, filename)
}