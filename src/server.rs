pub mod server
{
	use crate::c_hashmap::CMap;
	
	use std::io::Read;
	use std::net::TcpStream;
	
	use num_derive::FromPrimitive;
	use num_traits::FromPrimitive;
	
	#[derive(FromPrimitive)]
	pub enum Command
	{
		Subscribe = 0
	}
	
	fn u32_to_cmd(word: u32) -> Option<Command>
	{
		return Command::from_u32(word);
	}
	
	fn read_word(c: &mut TcpStream) -> u32
	{
		let mut buf: [u8; 4] = [0; 4];
		let _ = c.read(&mut buf);
		return u32::from_le_bytes(buf);
	}
	
	pub struct Topic
	{
		index: u64,
		key: u32
	}
	
	pub struct Server
	{
		pub clients: Vec<TcpStream>,
		next_topic_index: u64,
		topics: Vec<Topic>,
		topic_map: CMap
	}
	
	impl Server
	{
		pub fn new() -> Self
		{
			return Self{clients: Vec::new(), next_topic_index: 0, topics: Vec::new(), topic_map: CMap::new()};
		}
		
		pub fn add_client(&mut self, client: TcpStream)
		{
			self.clients.push(client);
		}
		
		pub fn iterate_clients(&mut self)
		{
			for mut c in &mut self.clients
			{
				let word: u32 = read_word(&mut c);
				let cmd_u32: Option<Command> = u32_to_cmd(word);
				
				if let Some(cmd) = cmd_u32
				{
					match cmd
					{
						Command::Subscribe =>
						{
							println!("subscribe");
							let topic_key: u32 = read_word(&mut c);
							let topic: Topic = Topic{index: self.next_topic_index, key: topic_key};
							self.topics.push(topic);
							self.topic_map.set(&self.topics.last().unwrap().key, self.next_topic_index);
							self.next_topic_index += 1;
						}
					};
				}
				
				else
				{
					// TODO: kill the connection if bad command received
					unreachable!();
				}
			}
		}
	}
}