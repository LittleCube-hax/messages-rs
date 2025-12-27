mod server;
mod c_hashmap;

use server::server::Server;

use std::net::TcpListener;

use std::sync::Mutex;
use std::thread;

fn main()
{
	let s: Server = Server::new();
	
	let listener: TcpListener = TcpListener::bind("127.0.0.1:6969").unwrap();
	
	println!("waiting for clients...");
	
	let s_mutex: Mutex<Server> = Mutex::new(s);
	
	thread::scope
	(
		|this_scope|
		{
			this_scope.spawn
			(
				||
				{
					for client in listener.incoming()
					{
						let mut s = match s_mutex.lock()
						{
							Ok(v) => v,
							Err(..) => unreachable!()
						};
						
						s.add_client(client.unwrap());
					}
				}
			);
			
			this_scope.spawn
			(
				||
				{
					loop
					{
						let mut s = match s_mutex.lock()
						{
							Ok(v) => v,
							Err(..) => unreachable!()
						};
						
						s.iterate_clients();
					}
				}
			);
		}
	);
}