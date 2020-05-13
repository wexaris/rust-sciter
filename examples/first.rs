//! First working example of the Sciter API.

extern crate sciter;

fn main() {
  // can be called as `examples/first ~/lib/libsciter.so`
  if cfg!(feature = "dynamic") {
    if let Some(arg) = std::env::args().nth(1) {
      println!("using {:?}", arg);
      if let Err(e) = sciter::set_library(&arg) {
        panic!("Invalid library path specified: {}", e);
      }
    }
  }

  let arch = if cfg!(target_arch = "x86_64") { "x64" } else { "x86" };
  println!("calling SciterAPI {}", arch);
  let scapi = sciter::SciterAPI();

  let abi_version = scapi.version;
  println!("sciter abi version: {:#0x}, windowless: {}", abi_version, sciter::is_windowless());

  let class_name = sciter::utf::w2s((scapi.SciterClassName)());
  println!("sciter class name: {:?}", class_name);

  use sciter::types::BOOL;
  let v1 = (scapi.SciterVersion)(true as BOOL);
  let v2 = (scapi.SciterVersion)(false as BOOL);
  let num = [v1 >> 16, v1 & 0xFFFF, v2 >> 16, v2 & 0xFFFF];
  let version = num.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(".");
  println!("sciter version: {} {:?}", version, num);
}
