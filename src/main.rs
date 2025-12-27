mod server;
mod c_hashmap;

use server::server::Server;
use c_hashmap::CMap;

use std::net::TcpListener;
use std::sync::Mutex;
use std::thread;

fn main()
{
	let topic_map: CMap = CMap::new();
	let key: u64 = 0x800ABB30;
	topic_map.set(&key, 4);
	let out: u64 = topic_map.get(&key);
	println!("got {out} from map");
	
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
						s_mutex.lock().unwrap().add_client(client.unwrap());
					}
				}
			);
			
			this_scope.spawn
			(
				||
				{
					println!("server has {} clients", s_mutex.lock().unwrap().clients.len());
				}
			);
		}
	);
}