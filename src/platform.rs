//! Platform-dependent windows support.

use sctypes::*;


pub trait BaseWindow {

	fn create(&mut self, flags: UINT, parent: HWINDOW) -> HWINDOW;

	fn get_hwnd(&self) -> HWINDOW;
	
	fn collapse(&self, hide: bool);
	fn expand(&self, maximize: bool);
	fn dismiss(&self);

	fn set_title(&mut self, title: &str);
	fn get_title(&self) -> String;

	fn run_app(&self);
	fn quit_app(&self);
}

#[cfg(windows)]
mod windows {

	use ::{_API};
	use sctypes::*;
	use scdef::*;


	#[link(name="user32")]
	extern "stdcall"
	{
		fn ShowWindow(hwnd: HWINDOW, show: INT) -> BOOL;
		fn PostMessageW(hwnd: HWINDOW, msg: UINT, w: WPARAM, l: LPARAM) -> BOOL;
		fn SetWindowTextW(hwnd: HWINDOW, s: LPCWSTR) -> BOOL;
		fn GetWindowTextLengthW(hwnd: HWINDOW) -> INT;
		fn GetWindowTextW(hwnd: HWINDOW, s: LPWSTR, l: INT) -> INT;
		fn GetMessageW(msg: LPMSG, hwnd: HWINDOW, min: UINT, max: UINT) -> BOOL;
		fn DispatchMessageW(msg: LPMSG) -> LRESULT;
		fn TranslateMessage(msg: LPMSG) -> BOOL;
		fn PostQuitMessage(code: INT);
	}

	pub struct OsWindow
	{
		hwnd: HWINDOW,
	}

	impl OsWindow {

		pub fn new() -> OsWindow {
			OsWindow { hwnd: 0 as HWINDOW }
		}

	}

	impl super::BaseWindow for OsWindow {

		/// Get native window handle.
		fn get_hwnd(&self) -> HWINDOW {
			return self.hwnd;
		}

		/// Create a new native window.
		fn create(&mut self, flags: UINT, parent: HWINDOW) -> HWINDOW {
			let rc = RECT::default();
			let cb = 0 as *const SciterWindowDelegate;
			self.hwnd = (_API.SciterCreateWindow)(flags, &rc, cb, 0 as LPVOID, parent);
			return self.hwnd;
		}
		
		/// Minimize or hide window.
		fn collapse(&self, hide: bool) {
			let n: INT = if hide { 0 } else { 6 };
			unsafe { ShowWindow(self.hwnd, n) };
		}

		/// Show or maximize window.
		fn expand(&self, maximize: bool) {
			let n: INT = if maximize { 3 } else { 1 };
			unsafe { ShowWindow(self.hwnd, n) };
		}

		/// Close window.
		fn dismiss(&self) {
			unsafe { PostMessageW(self.hwnd, 0x0010, 0, 0) };
		}

		/// Set native window title.
		fn set_title(&mut self, title: &str) {
			let (s,_) = s2w!(title);
			unsafe { SetWindowTextW(self.hwnd, s.as_ptr()) };
		}

		/// Get native window title.
		fn get_title(&self) -> String {

			let n = unsafe { GetWindowTextLengthW(self.hwnd) + 1 };
			let mut title: Vec<u16> = Vec::new();
			title.resize(n as usize, 0);
			unsafe { GetWindowTextW(self.hwnd, title.as_mut_ptr(), n) };
			return ::utf::w2s(title.as_ptr());
		}

		/// Run the main app message loop until window been closed.
		fn run_app(&self) {
			let mut msg = MSG { hwnd: 0 as HWINDOW, message: 0, wParam: 0, lParam: 0, time: 0, pt: POINT { x: 0, y: 0 } };
			let pmsg: LPMSG = &mut msg;
			let null: HWINDOW = ::std::ptr::null_mut();
			unsafe {
				while GetMessageW(pmsg, null, 0, 0) != 0 {
					TranslateMessage(pmsg);
					DispatchMessageW(pmsg);
				}
			};
		}

		/// Post app quit message.
		fn quit_app(&self) {
			unsafe { PostQuitMessage(0) };
		}
	}

}

#[cfg(unix)]
mod windows {

}

#[cfg(darwin)]
mod windows {

}

pub type OsWindow = windows::OsWindow;
