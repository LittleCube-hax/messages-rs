use libc::size_t;

#[link(name = "map")]
unsafe extern "C"
{
	fn hashmap_create() -> u64;
	fn hashmap_get(map: u64, key: *const u32, ksize: size_t, out_val: *mut usize) -> i32;
	fn hashmap_set(map: u64, key: *const u32, ksize: size_t, value: usize) -> i32;
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
	
	pub fn get(&self, key: &u32) -> Option<usize>
	{
		let mut value: usize = 0xFFFFFFFFFFFFFFFF;
		
		let result: i32;
		
		unsafe
		{
			result = hashmap_get(self.map, key as *const u32, 4, &mut value);
		}
		
		let ret: Option<usize> = if result != 0 { Some(value) } else { None };
		
		return ret;
	}
	
	pub fn set(&self, key: &u32, value: usize) -> i32
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