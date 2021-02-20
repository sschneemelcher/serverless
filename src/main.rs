use std::net::UdpSocket;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Write, BufRead, stdout, stdin};
use std::io::ErrorKind;
use std::time::Duration;


fn main() {
	let socket = UdpSocket::bind("0.0.0.0:8000").expect("couldn't bind to address");	
	socket.set_nonblocking(true).expect("could not set nonblocking");
	socket.set_broadcast(true).expect("could not set broadcast");
	let mut buf = [0u8; 1024];
	let msg_mutex = Arc::new(Mutex::new(String::new()));	
	let arc_mutex = Arc::clone(&msg_mutex);

	// spawning a thread to handle the user input because its blocking
	thread::spawn(move ||{
		let stdin = stdin();
		for line in stdin.lock().lines() {	
			*arc_mutex.lock().unwrap() = String::from(line.unwrap());
		}
	});

	loop { // check every 50 ms if there is a new message
	        if *msg_mutex.lock().unwrap() != ""{ // checks the mutex for new user input to send
			let mut msg = msg_mutex.lock().unwrap();
			socket.send_to((*msg).as_bytes(), "255.255.255.255:8000").ok();
			println!("You wrote: {}\n", *msg);
			*msg = "".to_string();
		}
		let result = socket.recv_from(&mut buf);
		match result {
			Ok((n, addr)) => {
				print!("{:?} wrote: ", addr);
				stdout().write(&mut buf[..n]).ok();
				stdout().flush().ok();
				print!("\n\n");
			},
			Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
				println!("received error: {}", err)
			}
			_ => {}
		}			
		thread::sleep(Duration::from_millis(50));
	}
}
