#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
	#[allow(unused_variables)]
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
    	let stream = stream.unwrap();
    	println!("Connection istablished, {:?}", stream);
    }
}
