//! Minimalistic Sciter sample.

extern crate sciter;

fn main() {
	let me = ::std::env::current_exe().unwrap();
	let dir = ::std::env::current_dir().unwrap();
	println!("me:  {},\ndir: {}", me.display(), dir.display());

	let mut frame = sciter::Window::new();
	frame.host.load_file("minimal.htm");
	frame.run_app(true);
}
