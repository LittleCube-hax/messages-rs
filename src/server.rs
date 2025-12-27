pub mod server
{
	use std::io::Read;
	use std::net::TcpStream;
	
	pub struct Topic
	{
		index: u64,
		key: u64
	}
	
	pub struct Server
	{
		pub clients: Vec<TcpStream>,
		topics: Vec<Topic>
	}
	
	impl Server
	{
		pub fn new() -> Self
		{
			return Self{clients: Vec::new(), topics: Vec::new()};
		}
		
		pub fn add_client(&mut self, client: TcpStream)
		{
			let port: u16 = client.peer_addr().unwrap().port();
			println!("got connection from {port}");
			self.clients.push(client);
			let mut buf: [u8; 4] = [0; 4];
			let mut client: &TcpStream = self.clients.last_mut().unwrap();
			let _ = client.read(&mut buf);
			println!("got:");
			for i in 0..4
			{
				println!("{}", buf[i]);
			}
		}
	}
}