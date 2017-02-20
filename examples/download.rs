//! Download http content (Go sciter example port).

#![allow(dead_code)]

extern crate sciter;

use sciter::host::{self, HostHandler};
use sciter::utf;
use std::rc::{Rc, Weak};

// struct Handler;

struct Handler {
	host: Weak<sciter::Host>,
}

impl HostHandler for Handler {
	fn on_data_loaded(&mut self, pnm: &host::SCN_DATA_LOADED) {
		println!("data loaded, uri: `{}`, {} bytes.", utf::w2s(pnm.uri), pnm.dataSize);
	}

	fn on_attach_behavior(&mut self, pnm: &mut host::SCN_ATTACH_BEHAVIOR) -> bool {
		let el = sciter::Element::from(pnm.element);
		let name = utf::u2s(pnm.name);
		println!("{}: behavior {}", el, name);

		if let Some(host) = self.host.upgrade() {
			let result = host.eval_script("[Sciter.userName(), Sciter.machineName(true)].join(`@`)");
			match result {
				Ok(mut name) => {
					name.isolate();
					println!("running on {:?}", name);
				},
				Err(e) => {
					println!("error! {}", e.as_string().unwrap_or("?".to_string()));
				},
			}
		}

		return false;
	}

}

impl Drop for Handler {
	fn drop(&mut self) {
		println!("Good bye, window");
	}
}

fn main() {
	use sciter::window;
	let mut frame = window::Window::with_size((1024,768), window::Flags::main_window(true));
	let handler = Handler { host: Rc::downgrade(&frame.get_host()) };
	frame.sciter_handler(handler);
	frame.set_title("Download sample");
	frame.load_file("http://httpbin.org/html");
	frame.run_app();
}
