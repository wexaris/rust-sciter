//! High level window wrapper.

use scdef::*;
use sctypes::*;

use platform::{BaseWindow, OsWindow};
use host::{Host};


/// Sciter window.
pub struct Window
{
	base: OsWindow,
	pub host: Host,
	// event: EventHandler,
}


impl Window {

	/// Create a new window and setup the sciter and dom callbacks.
	pub fn new() -> Window {
		let mut wnd = Window { base: OsWindow::new(), host: Host::new() };
		
		let flags = SCITER_CREATE_WINDOW_FLAGS::SW_MAIN
							 | SCITER_CREATE_WINDOW_FLAGS::SW_CONTROLS
							 | SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR
							 | SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE;
		wnd.base.create(flags as UINT, 0 as HWINDOW);

		let hwnd = wnd.base.get_hwnd();
		if !hwnd.is_null() {
			wnd.host.setup_debug();
			wnd.host.setup_callback(hwnd);
			// wnd.event.attach(hwnd);
		}
		return wnd;
	}

	/// Get native window handle.
	pub fn get_hwnd(&self) -> HWINDOW {
		self.base.get_hwnd()
	}
	
	/// Minimize or hide window.
	pub fn collapse(&self, hide: bool) {
		self.base.collapse(hide)
	}

	/// Show or maximize window.
	pub fn expand(&self, maximize: bool) {
		self.base.expand(maximize)
	}

	/// Close window.
	pub fn dismiss(&self) {
		self.base.dismiss()
	}

	/// Set native window title.
	pub fn set_title(&mut self, title: &str) {
		self.base.set_title(title)
	}

	/// Get native window title.
	pub fn get_title(&self) -> String {
		self.base.get_title()
	}

	/// Show window and run the main app message loop until window been closed.
	pub fn run_app(&self, show_window: bool) {
		if show_window {
			self.base.expand(false);
		}
		self.base.run_app();
	}

	/// Post app quit message.
	pub fn quit_app(&self) {
		self.base.quit_app()
	}
}
