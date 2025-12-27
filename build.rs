fn main()
{
	cc::Build::new()
		.file("lib/c-hashmap/map.c")
		.compile("map");
	
	println!("cargo:rustc-link-lib=static=map");
	println!("cargo:rerun-if-changed=lib/c-hashmap/map.c");
}