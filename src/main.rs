use std::net::UdpSocket;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Write, BufRead, stdout, stdin};
use std::io::ErrorKind;
use std::time::Duration;

fn encrypt(key: &Vec<u8>, msg: &[u8]) -> Vec<u8> {
	let mut v: Vec<u8> = msg.to_vec();
	let mut key2 = key.clone();
	let mut index = 0;

	while key2.len() < v.len() {
		key2.push(key2[index]);
		index += 1;	
	}
	
	v.iter_mut().zip(key2.iter()).for_each(| (b, k) | *b ^= *k);
	return v
}



fn main() {
	let socket = UdpSocket::bind("0.0.0.0:8000").expect("couldn't bind to address");	
	socket.set_nonblocking(true).expect("could not set nonblocking");
	socket.set_broadcast(true).expect("could not set broadcast");
	let mut buf = [0u8; 1024];
	let input_mutex = Arc::new(Mutex::new(String::new()));	
	let mut key = vec![0u8;1];

	// spawning a thread to handle the user input because its blocking
	let inp_mutex = Arc::clone(&input_mutex);
	thread::spawn(move ||{
		let stdin = stdin();
		for line in stdin.lock().lines() {	
			*inp_mutex.lock().unwrap() = String::from(line.unwrap());
		}
	});


	loop { // check every 50 ms if there is a new message
		thread::sleep(Duration::from_millis(50));
		let mut msg = input_mutex.lock().unwrap();
		if  *msg != ""{ // checks the mutex for new user input to send
			let mut v: Vec<&str> = (*msg).split(' ').collect();
			if v.len() > 1 {
				match v[0] {
					"key" => {
						key = Vec::<u8>::from(v[1]);	
						println!("received command {}", v[0]);
						println!("key is {:?}", key);
						},
					_ => {
						socket.send_to(&encrypt(&key, &(*msg).as_bytes()), "255.255.255.255:8000").ok();
						print!("sent ");
						stdout().write(&encrypt(&key, &(*msg).as_bytes())).ok();
						stdout().flush().ok();
						print!("\n");
						},
				}
			} else {
				socket.send_to(&encrypt(&key, &(*msg).as_bytes()), "255.255.255.255:8000").ok();
				print!("sent ");
				stdout().write(&encrypt(&key, &(*msg).as_bytes())).ok();
				stdout().flush().ok();
				print!("\n");
			}
			*msg = "".to_string();
		}

		let result = socket.recv_from(&mut buf);
		match result {
			Ok((n, addr)) => {
				print!("{:?} wrote: ", addr);
				stdout().write(&encrypt(&key, &mut buf[..n])).ok();
				stdout().flush().ok();
				print!("\n\n");
			},
			Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
				println!("received error: {}", err)
			}
			_ => {}
		}			
	}
	
}
