//! High level window wrapper.

use ::{_API};
use scdef::*;
use sctypes::*;

use platform::{BaseWindow, OsWindow};
use host::{Host, HostHandler};
use dom::event::{EventHandler};
use eventhandler::*;

/// Sciter window.
pub struct Window
{
	base: OsWindow,
	host: Host,
}

impl Window {

	/// Create a new window and setup the sciter and dom callbacks.
	pub fn new() -> Window {
		let flags = SCITER_CREATE_WINDOW_FLAGS::SW_MAIN
							 | SCITER_CREATE_WINDOW_FLAGS::SW_CONTROLS
							 | SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR
							 | SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE;

		let mut base = OsWindow::new();
		let hwnd = base.create(flags as UINT, 0 as HWINDOW);
		assert!(!hwnd.is_null());

		let wnd = Window { base: base, host: Host::from(hwnd)};
		return wnd;
	}

	/// Set callback for sciter engine events.
	pub fn sciter_handler<T: HostHandler + Sized>(&mut self, handler: T) {
		self.host.setup_callback(self.base.get_hwnd(), handler);
	}

	/// Attach `dom::EventHandler` to the Sciter window.
	///
	/// You can install Window EventHandler only once - it will survive all document reloads.
	pub fn event_handler<T: EventHandler>(&mut self, handler: T) {
		let boxed = Box::new( WindowHandler { hwnd: self.base.get_hwnd(), handler: handler } );
		let ptr = Box::into_raw(boxed);
		(_API.SciterWindowAttachEventHandler)(self.base.get_hwnd(), _event_handler_window_proc::<T>, ptr as LPVOID, ::dom::event::default_events() as UINT);
	}

	/// Load HTML document from file.
	pub fn load_file(&mut self, uri: &str) {
		self.host.load_file(uri)
	}

	/// Load HTML document from memory.
	pub fn load_html(&mut self, html: &[u8], uri: Option<&str>) {
		self.host.load_html(html, uri)
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
