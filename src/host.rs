//! Sciter host application helpers.

use ::{_API};
use sctypes::*;
use scdef::*;
use screquest::HREQUEST;
use schandler::NativeHandler;
use value::{Value};

/// A specialized `Result` type for sciter host operations.
pub type Result<T> = ::std::result::Result<T, ()>;

macro_rules! ok_or {
	($ok:ident) => {
		if $ok != 0 {
			Ok(())
		} else {
			Err(())
		}
	};

	($ok:ident, $rv:expr) => {
		if $ok != 0 {
			Ok($rv)
		} else {
			Err(())
		}
	};

	($ok:ident, $rv:expr, $err:expr) => {
		if $ok != 0 {
			Ok($rv)
		} else {
			Err($err)
		}
	};
}


/** Sciter notification handler for `Window.sciter_handler()`.

## Resource handling and custom resource loader

HTML loaded into Sciter may contain external resources: CSS (Cascading Style Sheets), images, fonts, cursors and scripts.
To get any of such resources Sciter will first send `on_data_load(SCN_LOAD_DATA)` notification to your application using
callback handler registered with `sciter::Window.sciter_handler()` function.

Your application can provide your own data for such resources (for example from resource section, DB or other storage of your choice)
or delegate resource loading to built-in HTTP client and file loader or discard loading at all.

Note: This handler should be registere before any `load_html` call in order to send notifications while loading.

*/
#[allow(unused_variables)]
pub trait HostHandler {

	/// Notifies that Sciter is about to download a referred resource.
	///
	/// You can load or overload data immediately by calling `self.data_ready()` with parameters provided by `SCN_LOAD_DATA`,
	/// or save them (including `request_id`) for later usage and answer here with `LOAD_RESULT::LOAD_DELAYED` code.
	///
	/// Also you can discard request (data will not be loaded at document) or take care about this request completely (via request API).
	fn on_data_load(&mut self, pnm: &mut SCN_LOAD_DATA) -> LOAD_RESULT { return LOAD_RESULT::LOAD_DEFAULT; }

	/// This notification indicates that external data (for example image) download process completed.
	fn on_data_loaded(&mut self, pnm: & SCN_DATA_LOADED) { }

	/// This notification is sent on parsing the document and while processing elements
	/// having non empty `style.behavior` attribute value.
	fn on_attach_behavior(&mut self, pnm: &mut SCN_ATTACH_BEHAVIOR) -> bool { return false; }

	/// This notification is sent when instance of the engine is destroyed.
	fn on_engine_destroyed(&mut self) { }

	/// This notification is sent when the engine encounters critical rendering error: e.g. DirectX gfx driver error.
  /// Most probably bad gfx drivers.
	fn on_graphics_critical_failure(&mut self) { }

	/// This output function will be used for reprting problems found while loading html and css documents.
	fn on_debug_output(&mut self, subsystem: OUTPUT_SUBSYTEMS, severity: OUTPUT_SEVERITY, message: &str) {
		if message.len() > 0 {
			println!("{:?}:{:?}: {}", severity, subsystem, message);
		}
	}

	/// This function is used as response to `on_data_load` request.
	///
	/// Parameters here must be taken from `SCN_LOAD_DATA` structure. You can store them for later usage,
	/// but you must answer as `LOAD_RESULT::LOAD_DELAYED` code and provide an `request_id` here.
	///
	fn data_ready(&mut self, hwnd: HWINDOW, uri: &str, data: &[u8], request_id: Option<HREQUEST>) {
		let (s,_) = s2w!(uri);
		match request_id {
			Some(req) => {
				(_API.SciterDataReadyAsync)(hwnd, s.as_ptr(), data.as_ptr(), data.len() as UINT, req)
			},
			None => {
				(_API.SciterDataReady)(hwnd, s.as_ptr(), data.as_ptr(), data.len() as UINT)
			},
		};
	}

}


/// Default HostHandler implementation
struct DefaultHandler;

/// Default HostHandler implementation
impl HostHandler for DefaultHandler {

}

use std::cell::{Cell, RefCell};

/// Sciter host runtime support.
pub struct Host {
	hwnd: Cell<HWINDOW>,
	handler: RefCell<NativeHandler>,
}

impl Host {

	#[doc(hidden)]
	pub fn from(hwnd: HWINDOW) -> Host {
		// Host with default debug handler installed
		let host = Host { hwnd: Cell::new(hwnd), handler: RefCell::new(NativeHandler::default()) };
		host.setup_callback(hwnd, DefaultHandler);
		return host;
	}

	/// Set callback for sciter engine events.
	pub fn setup_callback<T: HostHandler>(&self, hwnd: HWINDOW, handler: T) {
		*self.handler.borrow_mut() = NativeHandler::from(handler);
		self.hwnd.set(hwnd);
		self.enable_debug(true);
		(_API.SciterSetCallback)(hwnd, _on_handle_notification::<T>, self.handler.borrow().as_mut_ptr());
		(_API.SciterSetupDebugOutput)(0 as HWINDOW, self.handler.borrow().as_mut_ptr(), _on_debug_notification::<T>);
	}

	/// Setup debug output function for specific window or globally.
	pub fn enable_debug(&self, enable: bool) {
		let hwnd = 0 as HWINDOW;
		(_API.SciterSetOption)(hwnd, SCITER_RT_OPTIONS::SCITER_SET_DEBUG_MODE, enable as UINT_PTR);
	}

	/// Get native window handle.
	pub fn get_hwnd(&self) -> HWINDOW {
		self.hwnd.get()
	}

	/// Get window root DOM element.
	pub fn get_root(&self) -> Option<::dom::Element> {
		::dom::Element::from_window(self.hwnd.get()).ok()
	}

	/// Load HTML document from file.
	pub fn load_file(&self, uri: &str) {
		let (s,_) = s2w!(uri);
		(_API.SciterLoadFile)(self.hwnd.get(), s.as_ptr());
	}

	/// Load HTML document from memory.
	pub fn load_html(&self, html: &[u8], uri: Option<&str>) {
		match uri {
			Some(uri) => {
				let (s,_) = s2w!(uri);
				(_API.SciterLoadHtml)(self.hwnd.get(), html.as_ptr(), html.len() as UINT, s.as_ptr())
			},
			None => {
				(_API.SciterLoadHtml)(self.hwnd.get(), html.as_ptr(), html.len() as UINT, 0 as LPCWSTR)
			}
		};
	}

	/// This function is used as response to `SC_LOAD_DATA` request.
	pub fn data_ready(&self, uri: &str, data: &[u8], request_id: Option<HREQUEST>) {
		let (s,_) = s2w!(uri);
		match request_id {
			Some(req) => {
				(_API.SciterDataReadyAsync)(self.hwnd.get(), s.as_ptr(), data.as_ptr(), data.len() as UINT, req)
			},
			None => {
				(_API.SciterDataReady)(self.hwnd.get(), s.as_ptr(), data.as_ptr(), data.len() as UINT)
			},
		};
	}

	/// Evaluate script in context of current document.
	pub fn eval_script(&self, script: &str) -> ::std::result::Result<Value, Value> {
		let (s,n) = s2w!(script);
		let mut rv = Value::new();
		let ok = (_API.SciterEval)(self.hwnd.get(), s.as_ptr(), n, rv.as_ptr());
		ok_or!(ok, rv, rv)
	}

	/// Call scripting function defined in the global namespace.
	///
	/// This function returns `Result<Value,Value>` with script function result value or with sciter script error.
	/// You can use the `make_args!(a,b,c)` macro which help you construct script arguments from Rust types.
	pub fn call_function(&self, name: &str, args: &[Value]) -> ::std::result::Result<Value, Value> {
		let mut rv = Value::new();
		let (s,_) = s2u!(name);
		let argv = Value::pack_args(args);
		let ok = (_API.SciterCall)(self.hwnd.get(), s.as_ptr(), argv.len() as UINT, argv.as_ptr(), rv.as_ptr());
		ok_or!(ok, rv, rv)
	}

	/// Set various sciter engine options, see the `SCITER_RT_OPTIONS`.
	pub fn set_option(&self, option: SCITER_RT_OPTIONS, value: usize) -> Result<()> {
		let ok = (_API.SciterSetOption)(self.hwnd.get(), option, value as UINT_PTR);
		ok_or!(ok)
	}

	/// Set home url for sciter resources.
	///
	/// If you will set it like `set_home_url("http://sciter.com/modules/")` then
	///
	///  `<script src="sciter:lib/root-extender.tis">` will load
	///  root-extender.tis from
	///
	/// `http://sciter.com/modules/lib/root-extender.tis`.
	pub fn set_home_url(&self, url: &str) -> Result<()> {
		let (s,_) = s2w!(url);
		let ok = (_API.SciterSetHomeURL)(self.hwnd.get(), s.as_ptr());
		ok_or!(ok)
	}

	/// Set media type of this sciter instance.
	pub fn set_media_type(&self, media_type: &str) -> Result<()> {
		let (s,_) = s2w!(media_type);
		let ok = (_API.SciterSetMediaType)(self.hwnd.get(), s.as_ptr());
		ok_or!(ok)
	}

	/// Set media variables (dictionary) for this sciter instance.
	///
	/// By default sciter window has `"screen:true"` and `"desktop:true"/"handheld:true"` media variables.
	///
	/// Media variables can be changed in runtime. This will cause styles of the document to be reset.
	pub fn set_media_vars(&self, media: Value) -> Result<()> {
		let ok = (_API.SciterSetMediaVars)(self.hwnd.get(), media.as_cptr());
		ok_or!(ok)
	}

	/// Set or append the master style sheet styles (globally).
	pub fn set_master_css(&self, css: &str, append: bool) -> Result<()> {
		let (s,_) = s2u!(css);
		let b = s.as_bytes();
		let n = b.len() as UINT;
		let ok = if append {
			(_API.SciterAppendMasterCSS)(b.as_ptr(), n)
		} else {
			(_API.SciterSetMasterCSS)(b.as_ptr(), n)
		};
		ok_or!(ok)
	}

	/// Set (reset) style sheet of current document.
	pub fn set_window_css(&self, css: &str, base_url: &str, media_type: &str) -> Result<()> {
		let (s,_) = s2u!(css);
		let (url,_) = s2w!(base_url);
		let (media,_) = s2w!(media_type);
		let b = s.as_bytes();
		let n = b.len() as UINT;
		let ok = (_API.SciterSetCSS)(self.hwnd.get(), b.as_ptr(), n, url.as_ptr(), media.as_ptr());
		ok_or!(ok)
	}

}


// Sciter notification handler.
// This comes as free function due to https://github.com/rust-lang/rust/issues/32364
extern "stdcall" fn _on_handle_notification<T: HostHandler>(pnm: *mut SCITER_CALLBACK_NOTIFICATION, param: LPVOID) -> UINT
{
	// reconstruct pointer to Handler
	let mut boxed = NativeHandler::from_mut_ptr3(param);

	// process notification
	let nm: &mut SCITER_CALLBACK_NOTIFICATION = unsafe { &mut *pnm };
	let code: SCITER_NOTIFICATION = unsafe { ::std::mem::transmute(nm.code) };

	let result: UINT = match code {
		SCITER_NOTIFICATION::SC_LOAD_DATA => {
			let me = boxed.as_mut::<T>();
			let scnm = pnm as *mut SCN_LOAD_DATA;
			let re = me.on_data_load(unsafe { &mut *scnm} );
			re as UINT
		},

		SCITER_NOTIFICATION::SC_DATA_LOADED => {
			let me = boxed.as_mut::<T>();
			let scnm = pnm as *mut SCN_DATA_LOADED;
			me.on_data_loaded(unsafe { &mut *scnm} );
			0 as UINT
		},

		SCITER_NOTIFICATION::SC_ATTACH_BEHAVIOR => {
			let me = boxed.as_mut::<T>();
			let scnm = pnm as *mut SCN_ATTACH_BEHAVIOR;
			let re = me.on_attach_behavior(unsafe { &mut *scnm} );
			re as UINT
		},

		SCITER_NOTIFICATION::SC_ENGINE_DESTROYED => {
			let me = boxed.as_mut::<T>();
			me.on_engine_destroyed();
			0 as UINT
		},

		SCITER_NOTIFICATION::SC_GRAPHICS_CRITICAL_FAILURE => {
			let me = boxed.as_mut::<T>();
			me.on_engine_destroyed();
			0 as UINT
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
	let mut boxed = NativeHandler::from_mut_ptr3(param);
	{
		let me = boxed.as_mut::<T>();
		let message = ::utf::w2s(text).replace("\r", "\n");
		me.on_debug_output(subsystem, severity, message.trim_right());
	}
}
