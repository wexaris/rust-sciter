//! Platform-dependent windows support.

extern crate winapi;

use capi::sctypes::*;


pub trait BaseWindow {

	fn create(&mut self, rect: (i32,i32,i32,i32), flags: UINT, parent: HWINDOW) -> HWINDOW;

	fn get_hwnd(&self) -> HWINDOW;

	fn collapse(&self, hide: bool);
	fn expand(&self, maximize: bool);
	fn dismiss(&self);

	fn set_title(&mut self, title: &str);
	fn get_title(&self) -> String;

	fn run_app(&self);
	fn quit_app(&self);
}

pub enum TrayEvents {
  Unknown(usize),

  ContextMenu(i32, i32),

  MouseMove(i32, i32),

  LButtonDown(i32, i32),
  LButtonUp(i32, i32),
  LButtonDblClick(i32, i32),

  RButtonDown(i32, i32),
  RButtonUp(i32, i32),
  RButtonDblClick(i32, i32),

  MButtonDown(i32, i32),
  MButtonUp(i32, i32),
  MButtonDblClick(i32, i32),
}

pub type OnTrayIcon = fn(event: TrayEvents) -> Option<usize>;

#[cfg(windows)]
mod windows {

	use ::{_API};
  use platform::{OnTrayIcon};
  use platform::winapi::um::shellapi::{NOTIFYICONDATAW};
	use capi::sctypes::*;
	use capi::scdef::*;
  use ::std::ptr;


	#[link(name="user32")]
	extern "system"
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

	#[link(name="ole32")]
	extern "system"
	{
		fn OleInitialize(pv: LPCVOID) -> i32;	// HRESULT
	}

	pub struct OsWindow
	{
		hwnd: HWINDOW,
		flags: UINT,
    recreate_msg: UINT,
    tray_msg: UINT,
    tray_data: Option<NOTIFYICONDATAW>,
    tray_callback: Option<OnTrayIcon>,
	}

	impl OsWindow {

		pub fn new() -> OsWindow {
			OsWindow {
        hwnd: 0 as HWINDOW,
        flags: 0,
        tray_callback: None,
        tray_data: None,
        tray_msg: 0,
        recreate_msg: 0,
      }
		}

		pub fn from(hwnd: HWINDOW) -> OsWindow {
      let mut me = OsWindow::new();
      me.hwnd = hwnd;
			me
		}

		fn init_app() {
			unsafe { OleInitialize(ptr::null()) };
		}

    pub fn notify_icon(&mut self, title: &str, on_tray: OnTrayIcon) -> bool {
      unsafe {
        use ::std::mem;
        use platform::winapi::shared::windef::{HWND};
        use platform::winapi::um::winuser::{RegisterWindowMessageW};
        use platform::winapi::um::shellapi::{NOTIFYICON_VERSION_4, NIF_TIP, NIF_ICON, NIF_MESSAGE};

        let (s,_) = s2w!("TaskbarCreated");
        self.recreate_msg = RegisterWindowMessageW(s.as_ptr());

        let (sz_tip, cb_tip) = s2w!(title);

        let mut ni: NOTIFYICONDATAW = mem::zeroed();
        ni.cbSize = mem::size_of::<NOTIFYICONDATAW>() as u32;
        ni.uFlags = NIF_TIP | NIF_ICON | NIF_MESSAGE;
        ni.hWnd = self.hwnd as HWND;
        ni.uID = self.hwnd as usize as u32;
        ni.uCallbackMessage = 0x7AA1;
        *ni.u.uVersion_mut() = NOTIFYICON_VERSION_4;
        ptr::copy(sz_tip.as_ptr(), ni.szTip.as_mut_ptr(), cb_tip as usize);

        self.tray_msg = ni.uCallbackMessage;
        self.tray_data = Some(ni);
        self.tray_callback = Some(on_tray);

        self.add_icon()
      }
    }

    fn add_icon(&mut self) -> bool {
      use platform::winapi::um::shellapi::{Shell_NotifyIconW, NIM_ADD, NIM_SETVERSION};
      unsafe {
        if self.tray_data.is_none() {
          return false;
        }
        let ni = self.tray_data.as_mut().unwrap();
        let ni = ni as *mut _;
        let ok = Shell_NotifyIconW(NIM_ADD, ni);
        Shell_NotifyIconW(NIM_SETVERSION, ni);
        return ok != 0;
      }
    }

    fn remove_icon(&mut self) -> bool {
      use platform::winapi::um::shellapi::{Shell_NotifyIconW, NIM_DELETE};
      unsafe {
        if self.tray_data.is_none() {
          return false;
        }
        let ni = self.tray_data.as_mut().unwrap();
        let ok = Shell_NotifyIconW(NIM_DELETE, ni as *mut _);
        return ok != 0;
      }
    }

    extern "system" fn on_message(hwnd: HWINDOW, msg: UINT, wparam: WPARAM, lparam: LPARAM,
      param: LPVOID, handled: *mut BOOL) -> LRESULT
    {
      assert!(!param.is_null());
      let me = param as *mut OsWindow;
      let me = unsafe { &mut *me };

      if msg == me.recreate_msg {
        me.add_icon();
      }

      if msg == me.tray_msg && me.tray_callback.is_some() {
        use platform::winapi::shared::windef::{HWND, POINT};
        use platform::winapi::shared::minwindef::{LOWORD, HIWORD};
        use platform::winapi::um::winuser::*;

        let msg = LOWORD(lparam as u32) as u32;
        let _id = HIWORD(lparam as u32);
        let cx = LOWORD(wparam as u32) as i16 as i32;
        let cy = HIWORD(wparam as u32) as i16 as i32;

        let mut pt = POINT {
          x: cx,
          y: cy,
        };
        unsafe { MapWindowPoints(HWND_DESKTOP, hwnd as HWND, &mut pt as *mut _, 1) };

        let cx = pt.x;
        let cy = pt.y;

        use platform::TrayEvents::*;
        let evt = match msg {
          WM_CONTEXTMENU => ContextMenu(cx, cy),

          WM_LBUTTONDOWN => LButtonDown(cx, cy),
          WM_LBUTTONUP => LButtonUp(cx, cy),
          WM_LBUTTONDBLCLK => LButtonDblClick(cx, cy),

          WM_RBUTTONDOWN => RButtonDown(cx, cy),
          WM_RBUTTONUP => RButtonUp(cx, cy),
          WM_RBUTTONDBLCLK => RButtonDblClick(cx, cy),

          WM_MBUTTONDOWN => MButtonDown(cx, cy),
          WM_MBUTTONUP => MButtonUp(cx, cy),
          WM_MBUTTONDBLCLK => MButtonDblClick(cx, cy),

          _ => Unknown(wparam),
        };

        let callback = me.tray_callback.as_ref().unwrap();
        if let Some(result) = callback(evt) {
          assert!(!handled.is_null());
          unsafe { *handled = true as BOOL };
          return result as LRESULT;
        }
      }
      return 0 as LRESULT;
    }
	}

	impl super::BaseWindow for OsWindow {

		/// Get native window handle.
		fn get_hwnd(&self) -> HWINDOW {
			return self.hwnd;
		}

		/// Create a new native window.
		fn create(&mut self, rect: (i32,i32,i32,i32), flags: UINT, parent: HWINDOW) -> HWINDOW {

			if (flags & SCITER_CREATE_WINDOW_FLAGS::SW_MAIN as u32) != 0 {
				OsWindow::init_app();
			}

			let (x,y,w,h) = rect;
			let rc = RECT { left: x, top: y, right: x + w, bottom: y + h };

			let msg_cb = Some(&(OsWindow::on_message as SciterWindowDelegate));
      let msg_param = self as *mut _ as LPVOID;

			self.flags = flags;
			self.hwnd = (_API.SciterCreateWindow)(flags, &rc, msg_cb, msg_param, parent);
			if self.hwnd.is_null() {
				panic!("Failed to create window!");
			}
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
			let null: HWINDOW = ptr::null_mut();
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

#[cfg(target_os="linux")]
mod windows {

	use ::{_API};
	use capi::sctypes::*;
	use capi::scdef::*;
	use super::BaseWindow;

	use ::std::ptr;

	#[link(name="gtk-3")]
	extern "C"
	{
		fn gtk_init(argc: *const i32, argv: *const *const LPCSTR);
		fn gtk_main();
		fn gtk_main_quit();
		fn gtk_widget_get_toplevel(view: HWINDOW) -> HWINDOW;
		fn gtk_window_present(hwnd: HWINDOW);
		fn gtk_widget_hide(hwnd: HWINDOW);
		fn gtk_window_maximize(hwnd: HWINDOW);
		fn gtk_window_iconify(hwnd: HWINDOW);
		fn gtk_window_close(hwnd: HWINDOW);
		fn gtk_window_set_title(hwnd: HWINDOW, title: LPCSTR);
		fn gtk_window_get_title(hwnd: HWINDOW) -> LPCSTR;
	}

	pub struct OsWindow
	{
		hwnd: HWINDOW,
		flags: UINT,
	}

	impl OsWindow {

		pub fn new() -> OsWindow {
			OsWindow { hwnd: 0 as HWINDOW, flags: 0 }
		}

		pub fn from(hwnd: HWINDOW) -> OsWindow {
			OsWindow { hwnd: hwnd, flags: 0 }
		}

		fn init_app() {
			unsafe { gtk_init(ptr::null(), ptr::null()) };
		}

		fn window(&self) -> HWINDOW {
			let hwnd = self.get_hwnd();
			if hwnd.is_null() {
				hwnd
			} else {
				unsafe { gtk_widget_get_toplevel(hwnd) }
			}
		}

	}

	impl super::BaseWindow for OsWindow {

		/// Get native window handle.
		fn get_hwnd(&self) -> HWINDOW {
			return self.hwnd;
		}

		/// Create a new native window.
		fn create(&mut self, rect: (i32,i32,i32,i32), flags: UINT, parent: HWINDOW) -> HWINDOW {

			if (flags & SCITER_CREATE_WINDOW_FLAGS::SW_MAIN as u32) != 0 {
				OsWindow::init_app();
			}

			let (x,y,w,h) = rect;
			let rc = RECT { left: x, top: y, right: x + w, bottom: y + h };

			let cb = ptr::null();
			self.flags = flags;
			self.hwnd = (_API.SciterCreateWindow)(flags, &rc, cb, 0 as LPVOID, parent);
			if self.hwnd.is_null() {
				panic!("Failed to create window!");
			}
			return self.hwnd;
		}

		/// Minimize or hide window.
		fn collapse(&self, hide: bool) {
			unsafe {
				if hide {
					gtk_widget_hide(self.get_hwnd())
				} else {
					gtk_window_iconify(self.window())
				}
			};
		}

		/// Show or maximize window.
		fn expand(&self, maximize: bool) {
			let wnd = self.window();
			unsafe {
				if maximize {
					gtk_window_maximize(wnd)
				} else {
					gtk_window_present(wnd)
				}
			};
		}

		/// Close window.
		fn dismiss(&self) {
			unsafe { gtk_window_close(self.window()) };
		}

		/// Set native window title.
		fn set_title(&mut self, title: &str) {
			let (s,_) = s2u!(title);
			unsafe { gtk_window_set_title(self.window(), s.as_ptr()) };
		}

		/// Get native window title.
		fn get_title(&self) -> String {
			let s = unsafe { gtk_window_get_title(self.window()) };
			return u2s!(s);
		}

		/// Run the main app message loop until window been closed.
		fn run_app(&self) {
			unsafe { gtk_main() };
		}

		/// Post app quit message.
		fn quit_app(&self) {
			unsafe { gtk_main_quit() };
		}
	}

}

#[cfg(target_os="macos")]
mod windows {

	extern crate objc_foundation;

	use objc::runtime::{Class, Object};
	use self::objc_foundation::{NSString, INSString};


	/// Activation policies that control whether and how an app may be activated.
	#[repr(C)]
	#[allow(dead_code)]
	enum NSApplicationActivationPolicy {
		Regular = 0,
		Accessory,
		Prohibited,
	}


	// Note: Starting some OSX version (perhaps, 10.13),
	// the AppKit framework isn't loaded implicitly.
	#[link(name="CoreFoundation", kind="framework")]
	extern {}

	#[link(name="AppKit", kind="framework")]
	extern {}

	use ::{_API};
	use capi::sctypes::*;
	use capi::scdef::*;
	use super::BaseWindow;

	pub struct OsWindow
	{
		hwnd: HWINDOW,
		flags: UINT,
	}

	impl OsWindow {

		pub fn new() -> OsWindow {
			OsWindow { hwnd: 0 as HWINDOW, flags: 0, }
		}

		pub fn from(hwnd: HWINDOW) -> OsWindow {
			OsWindow { hwnd: hwnd, flags: 0 }
		}

		fn get_app() -> *mut Object {
			let cls = Class::get("NSApplication").expect("`NSApplication` is not registered.");
			let obj = unsafe { msg_send!(cls, sharedApplication) };
			return obj;
		}

		fn init_app() {
			// By default, unbundled apps start with `NSApplicationActivationPolicyProhibited` (no dock, no menu).
			let app = OsWindow::get_app();
			unsafe { msg_send!(app, setActivationPolicy:NSApplicationActivationPolicy::Regular) };
		}

		fn view(&self) -> *mut Object {
			let hwnd = self.get_hwnd();
			let hwnd: *mut Object = unsafe { ::std::mem::transmute(hwnd) };
			return hwnd;
		}

		fn window(&self) -> *mut Object {
			let hwnd = self.view();
			let obj: *mut Object = unsafe { msg_send!(hwnd, window) };
			assert!(!obj.is_null());
			return obj;
		}
	}

	impl super::BaseWindow for OsWindow {

		/// Get native window handle.
		fn get_hwnd(&self) -> HWINDOW {
			return self.hwnd;
		}

		/// Create a new native window.
		fn create(&mut self, rect: (i32,i32,i32,i32), flags: UINT, parent: HWINDOW) -> HWINDOW {

			if (flags & SCITER_CREATE_WINDOW_FLAGS::SW_MAIN as u32) != 0 {
				OsWindow::init_app();
			}

			let (x,y,w,h) = rect;
			let rc = RECT { left: x, top: y, right: x + w, bottom: y + h };
			let prc: *const RECT = if w > 0 && h > 0 {
				&rc
			} else {
				0 as *const RECT
			};

			let cb = 0 as *const SciterWindowDelegate;
			self.flags = flags;
			self.hwnd = (_API.SciterCreateWindow)(flags, prc, cb, 0 as LPVOID, parent);
			if self.hwnd.is_null() {
				panic!("Failed to create window!");
			}
			return self.hwnd;
		}

		/// Minimize or hide window.
		fn collapse(&self, hide: bool) {
			let wnd = self.window();
			if hide {
				unsafe { msg_send!(wnd, orderOut:0) };
			} else {
				let hwnd = self.view();
				unsafe { msg_send!(wnd, performMiniaturize:hwnd) };
			}
		}

		/// Show or maximize window.
		fn expand(&self, maximize: bool) {
			let wnd = self.window();
			if (self.flags & SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR as UINT) != 0 {
				let app = OsWindow::get_app();
				unsafe { msg_send!(app, activateIgnoringOtherApps:true) };
			}
			unsafe {
				msg_send!(wnd, makeKeyAndOrderFront:0);
				// msg_send!(wnd, orderFrontRegardless);
			}
			if maximize {
				unsafe { msg_send!(wnd, performZoom:0) };
			}
		}

		/// Close window.
		fn dismiss(&self) {
			let wnd = self.window();
			unsafe { msg_send!(wnd, close) };
		}

		/// Set native window title.
		fn set_title(&mut self, title: &str) {
			let s = NSString::from_str(title);
			let wnd = self.window();
			unsafe { msg_send!(wnd, setTitle:s) };
		}

		/// Get native window title.
		fn get_title(&self) -> String {
			String::new()
		}

		/// Run the main app message loop until window been closed.
		fn run_app(&self) {
			let app = OsWindow::get_app();
			unsafe { msg_send!(app, finishLaunching) };
			unsafe { msg_send!(app, run) };
		}

		/// Post app quit message.
		fn quit_app(&self) {
			let app = OsWindow::get_app();
			unsafe { msg_send!(app, terminate:app) };
		}
	}

}

pub type OsWindow = windows::OsWindow;
