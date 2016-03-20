//! Download http content (Go sciter example port).

#![allow(dead_code)]

extern crate sciter;

use sciter::host::HostHandler;
use sciter::utf;

struct Handler;

impl HostHandler for Handler {
	fn on_data_loaded(&self, pnm: & sciter::SCN_DATA_LOADED) -> u32 {
		println!("data loaded, uri: `{}`, {} bytes.", utf::w2s(pnm.uri), pnm.dataSize);
		return 0;
	}
}

fn main() {
	let mut frame = sciter::Window::new();
	frame.sciter_handler(Handler);
	frame.set_title("Download sample");
	frame.load_file("http://httpbin.org/html");
	frame.run_app(true);
}
