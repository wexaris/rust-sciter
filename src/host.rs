//! Sciter host application helpers.

use ::{_API};
use sctypes::*;
use scdef::*;
use scdom::{HELEMENT};
use screquest::HREQUEST;
use scvalue::{VALUE};
use value::{Value};
use utf::{w2s};
use schandler::NativeHandler;


/// Trait for SCITER_CALLBACK_NOTIFY handler.
#[allow(unused_variables)]
pub trait HostHandler {

	/// Notifies that Sciter is about to download a referred resource.
	fn on_data_load(&self, pnm: &mut SCN_LOAD_DATA) -> LOAD_RESULT { return LOAD_RESULT::LOAD_OK; }

	/// This notification indicates that external data (for example image) download process completed.
	fn on_data_loaded(&self, pnm: & SCN_DATA_LOADED) -> u32 { return 0; }

	/// This notification is sent on parsing the document and while processing elements having non empty `style.behavior` attribute value.
	fn on_attach_behavior(&self, pnm: &mut SCN_ATTACH_BEHAVIOR) -> u32 { return 0; }

	fn on_engine_destroyed(&self) -> u32 { return 0; }

	#[cfg(optional)]
	fn on_graphics_critical_failure(&self) -> u32 { return 0; }

	/// This output function will be used for reprting problems found while loading html and css documents.
	fn on_debug_output(&self, subsystem: OUTPUT_SUBSYTEMS, severity: OUTPUT_SEVERITY, message: &str) {
		if message.len() > 0 {
			println!("{:?}:{:?}: {}", severity, subsystem, message);
		}
	}
}


/// Default HostHandler implementation
struct DefaultHandler;

/// Default HostHandler implementation
impl HostHandler for DefaultHandler {

}


/// Sciter host runtime support.
pub struct Host {
	hwnd: HWINDOW,
	handler: NativeHandler,
}

impl Host {

	pub fn new() -> Host {
		Host { hwnd: 0 as HWINDOW, handler: NativeHandler::default() }
	}

	pub fn from(hwnd: HWINDOW) -> Host {
		// Host with default debug handler installed
		let mut host = Host { hwnd: hwnd, handler: NativeHandler::default() };
		host.setup_callback(hwnd, DefaultHandler);
		return host;
	}

	/// Set callback for sciter engine events.
	pub fn setup_callback<T: HostHandler>(&mut self, hwnd: HWINDOW, handler: T) {
		self.handler = NativeHandler::from(handler);
		self.hwnd = hwnd;
		self.enable_debug(true);
		(_API.SciterSetCallback)(hwnd, _on_handle_notification::<T>, self.handler.as_mut_ptr());
		(_API.SciterSetupDebugOutput)(0 as HWINDOW, self.handler.as_mut_ptr(), _on_debug_notification::<T>);
	}

	/// Setup debug output function for specific window or globally.
	pub fn enable_debug(&mut self, enable: bool) {
		let hwnd = 0 as HWINDOW;
		(_API.SciterSetOption)(hwnd, SCITER_RT_OPTIONS::SCITER_SET_DEBUG_MODE, enable as UINT_PTR);
	}

	/// Get window root DOM element.
	pub fn get_root(&self) -> HELEMENT {
		let mut he = 0 as HELEMENT;
		(_API.SciterGetRootElement)(self.hwnd, &mut he);
		return he;
	}

	/// Load HTML document from file.
	pub fn load_file(&mut self, uri: &str) {
		let (s,_) = s2w!(uri);
		(_API.SciterLoadFile)(self.hwnd, s.as_ptr());
	}

	/// Load HTML document from memory.
	pub fn load_html(&mut self, html: &[u8], uri: Option<&str>) {
		match uri {
			Some(uri) => {
				let (s,_) = s2w!(uri);
				(_API.SciterLoadHtml)(self.hwnd, html.as_ptr(), html.len() as UINT, s.as_ptr())
			},
			None => {
				(_API.SciterLoadHtml)(self.hwnd, html.as_ptr(), html.len() as UINT, 0 as LPCWSTR)
			}
		};
	}

	/// This function is used as response to SCN_LOAD_DATA request.
	pub fn data_ready(&self, uri: &str, data: &[u8], request_id: Option<HREQUEST>) {
		let (s,_) = s2w!(uri);
		match request_id {
			Some(req) => {
				(_API.SciterDataReadyAsync)(self.hwnd, s.as_ptr(), data.as_ptr(), data.len() as UINT, req)
			},
			None => {
				(_API.SciterDataReady)(self.hwnd, s.as_ptr(), data.as_ptr(), data.len() as UINT)
			},
		};
	}

	/// Evaluate script in context of current document.
	pub fn eval_script(&self, script: &str, _name: Option<&str>) -> Value {
		let (s,n) = s2w!(script);
		let mut rv = Value::new();
		(_API.SciterEval)(self.hwnd, s.as_ptr(), n, rv.as_ptr());
		return rv;
	}

	/// Call scripting function defined in the global namespace.
	pub fn call_function(&self, name: &str/*, Args… args*/) -> Value {
		let (s,_) = s2u!(name);
		let argv = 0 as *const VALUE;
		let argc: u32 = 0;
		let mut rv = Value::new();
		(_API.SciterCall)(self.hwnd, s.as_ptr(), argc, argv, rv.as_ptr());
		return rv;
	}

}


// Sciter notification handler.
// This comes as free function due to https://github.com/rust-lang/rust/issues/32364
extern "stdcall" fn _on_handle_notification<T: HostHandler>(pnm: *mut SCITER_CALLBACK_NOTIFICATION, param: LPVOID) -> UINT
{
	// reconstruct pointer to Handler
	let boxed = NativeHandler::from_mut_ptr2(param);

	// process notification
	let nm: &mut SCITER_CALLBACK_NOTIFICATION = unsafe { &mut *pnm };
	let code: SCITER_NOTIFICATION = unsafe { ::std::mem::transmute(nm.code) };

	let result: UINT = match code {
		SCITER_NOTIFICATION::SC_LOAD_DATA => {
			let me = boxed.as_ref::<T>();
			let scnm = pnm as *mut SCN_LOAD_DATA;
			let re = me.on_data_load(unsafe { &mut *scnm} );
			re as UINT
		},

		SCITER_NOTIFICATION::SC_DATA_LOADED => {
			let me = boxed.as_ref::<T>();
			let scnm = pnm as *mut SCN_DATA_LOADED;
			let re = me.on_data_loaded(unsafe { &mut *scnm} );
			re as UINT
		},

		SCITER_NOTIFICATION::SC_ATTACH_BEHAVIOR => {
			let me = boxed.as_ref::<T>();
			let scnm = pnm as *mut SCN_ATTACH_BEHAVIOR;
			let re = me.on_attach_behavior(unsafe { &mut *scnm} );
			re as UINT
		},

		SCITER_NOTIFICATION::SC_ENGINE_DESTROYED => {
			let me = boxed.as_ref::<T>();
			let re = me.on_engine_destroyed();
			re as UINT
		},

		_ => 0,
	};

	return result;
}

// Sciter debug output handler.
extern "stdcall" fn _on_debug_notification<T: HostHandler>(param: LPVOID, subsystem: OUTPUT_SUBSYTEMS, severity: OUTPUT_SEVERITY,
	text: LPCWSTR, _text_length: UINT)
{
	// reconstruct pointer to Handler
	let boxed = NativeHandler::from_mut_ptr2(param);

	{
		let me = boxed.as_ref::<T>();
		let message = w2s(text).replace("\r", "\n");
		me.on_debug_output(subsystem, severity, message.trim_right());
	}
}
