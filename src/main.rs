use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
// Imported mods
mod handel_connection;

fn main() {
	let server = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in server.incoming() {
		let stream = stream.unwrap();
		// Create threads for the server
		thread::spawn(|| {			
			handel_connection::run(stream);
		});
	}
}
