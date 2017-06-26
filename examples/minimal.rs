//! Minimalistic Sciter sample.
#![windows_subsystem="windows"]
extern crate sciter;

fn main() {
	// Step 1: Include the 'minimal.html' file to a byte array.
	// Hint: Take a look into 'minimal.html' which contains tiscript
	// rust Macro from std (`std::include_bytes!`).
	let html = include_bytes!("minimal.htm");

	// Step 2: Create a new main sciter window of type sciter::Window
	// Hint: The sciter Window wrapper (/src/window.rs) contains more
	// interesting functions to open or attach a scriter window.
	let mut frame = sciter::Window::new();

	// Step 3: Load HTML byte array from memory to sciter::Window.
	// Hint: second parameter is a optional uri (here: None).
	// Also you can use load_file, but it requires an absolute path
	// of root loaded document to resolve HTML resources properly.
	frame.load_html(html, None);

	// Step 4: Show window and run the main app message loop until window been closed.
	frame.run_app();
}
