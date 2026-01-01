use crate::c_hashmap::CMap;

use std::collections::HashSet;
use std::io::Read;
use std::io::Write;
use std::io::Error;
use std::io::ErrorKind;
use std::net::TcpStream;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Command
{
	Subscribe = 0,
	Publish = 1,
	Message = 2
}

fn u32_to_cmd(word: u32) -> Option<Command>
{
	return Command::from_u32(word);
}

fn read_word_option(c: &mut TcpStream) -> Result<u32, Error>
{
	let mut buf: [u8; 4] = [0; 4];
	
	let result_option: Result<usize, Error> = c.read(&mut buf);
	
	if let Ok(result) = result_option && result != 0
	{
		return Ok(u32::from_le_bytes(buf));
	}
	
	else if let Ok(result) = result_option && result == 0
	{
		return Err(Error::from(ErrorKind::Other));
	}
	
	else if let Err(result) = result_option
	{
		return Err(result);
	}
	
	else
	{
		unreachable!();
	}
}

fn read_word(c: &mut TcpStream) -> u32
{
	let mut buf: [u8; 4] = [0; 4];
	let _ = c.read(&mut buf);
	return u32::from_le_bytes(buf);
}

pub struct Topic
{
	key: u32,
	subscribers: HashSet<usize>
}

pub struct Server
{
	pub clients: Vec<TcpStream>,
	next_topic_index: usize,
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
		let mut disconnected: Vec<usize> = Vec::new();
		
		for i in 0..self.clients.len()
		{
			let c: &mut TcpStream = &mut self.clients[i];
			let _ = c.set_nonblocking(true);
			let word_result: Result<u32, Error> = read_word_option(c);
			_ = c.set_nonblocking(false);
			let mut _cmd_u32: Option<Command> = None;
			
			match word_result
			{
				Ok(w) =>
				{
					_cmd_u32 = u32_to_cmd(w);
				},
				
				Err(e) =>
				{
					if e.kind() == ErrorKind::Other
					{
						disconnected.push(i);
					}
					
					continue;
				}
			}
			
			if let Some(cmd) = _cmd_u32
			{
				match cmd
				{
					Command::Subscribe =>
					{
						let topic_key: u32 = read_word(c);
						let topic_index: usize;
						
						if let Some(some_topic_index) = self.topic_map.get(&topic_key)
						{
							topic_index = some_topic_index;
						}
						
						else
						{
							let topic: Topic = Topic{key: topic_key, subscribers: HashSet::new()};
							self.topics.push(topic);
							self.topic_map.set(&self.topics.last().unwrap().key, self.next_topic_index);
							topic_index = self.next_topic_index;
							self.next_topic_index += 1;
						}
						
						self.topics[topic_index].subscribers.insert(i);
					},
					
					Command::Publish =>
					{
						let topic_key: u32 = read_word(c);
						let data_size: u32 = read_word(c);
						
						if let Some(topic_index) = self.topic_map.get(&topic_key)
						{
							let mut data: Vec<u8> = Vec::new();
							data.resize(usize::from_u32(data_size).unwrap(), 0);
							let _ = c.read(&mut data);
							
							self.send_message(topic_key, topic_index, &data, data_size);
						}
						
						else
						{
							// TODO: handle when the topic doesn't exist
							unimplemented!();
						}
					},
					
					_ =>
					{
						// TODO: kill the connection if other command received
						unimplemented!();
					}
				};
			}
			
			else
			{
				// TODO: kill the connection if bad command received
				unimplemented!();
			}
		}
		
		for c_i in disconnected
		{
			let _ = self.clients.swap_remove(c_i);
			let c_len = self.clients.len();
			
			for t in &mut self.topics
			{
				if t.subscribers.contains(&c_len)
				{
					t.subscribers.remove(&c_len);
					t.subscribers.insert(c_i);
				}
				
				else if t.subscribers.contains(&c_i)
				{
					t.subscribers.remove(&c_i);
				}
			}
		}
	}
	
	pub fn send_message(&mut self, topic_key: u32, topic_index: usize, data: &Vec<u8>, data_size: u32)
	{
		for sub_i in &self.topics[topic_index].subscribers
		{
			let sub: &mut TcpStream = &mut self.clients[*sub_i];
			let message_cmd: u32 = Command::Message as u32;
			let _ = sub.write(&message_cmd.to_le_bytes());
			_ = sub.write(&topic_key.to_le_bytes());
			_ = sub.write(&data_size.to_le_bytes());
			_ = sub.write(&data);
		}
	}
}