use libc::size_t;

#[link(name = "map")]
unsafe extern "C"
{
	fn hashmap_create() -> u64;
	fn hashmap_get(map: u64, key: *const u32, ksize: size_t, out_val: *mut u64) -> i32;
	fn hashmap_set(map: u64, key: *const u32, ksize: size_t, value: u64) -> i32;
	fn hashmap_free(map: u64);
}

pub struct CMap
{
	map: u64
}

impl CMap
{
	pub fn new() -> Self
	{
		unsafe
		{
			return Self{map: hashmap_create()};
		}
	}
	
	pub fn get(&self, key: &u32) -> u64
	{
		let mut value: u64 = 0xFFFFFFFFFFFFFFFF;
		
		unsafe
		{
			hashmap_get(self.map, key as *const u32, 4, &mut value);
		}
		
		return value;
	}
	
	pub fn set(&self, key: &u32, value: u64) -> i32
	{
		unsafe
		{
			return hashmap_set(self.map, key as *const u32, 4, value);
		}
	}
}

impl Drop for CMap
{
	fn drop(&mut self)
	{
		unsafe
		{
			hashmap_free(self.map);
		}
	}
}