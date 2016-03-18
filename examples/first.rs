//! First working example of the Sciter API.

extern crate sciter;

fn main() {
	println!("calling SciterAPI");
	let scapi = sciter::SciterAPI();

	println!("getting abi version");
	let abi_version = scapi.version;
	println!("sciter abi version: {:?}", abi_version);

	let class_name = scapi.SciterClassName();
	println!("sciter class name: {}", class_name);

	let v1 = (scapi.SciterVersion)(true);
	let v2 = (scapi.SciterVersion)(false);
	let num = [v1 >> 16, v1 & 0xFFFF, v2 >> 16, v2 & 0xFFFF];
	let version = num.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(".");
	println!("sciter version: {} {:?}", version, num);
}
