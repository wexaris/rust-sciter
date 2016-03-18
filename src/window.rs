//! High level window wrapper.

#![allow(dead_code)]

use scdef::*;
use sctypes::*;

use platform::{BaseWindow, OsWindow};
use host::{Host};


/// Basic Sciter window.
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

	/// Show window and run the main app message loop until window been closed.
	pub fn run_app(&self, show_window: bool) {
		if show_window {
			self.base.expand(false);
		}
		self.base.run_app();
	}

}
