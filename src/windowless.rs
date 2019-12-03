//! Messages for Windowless Sciter.

use ::{_API};
use capi::scdef::{GFX_LAYER};
use capi::scdom::HELEMENT;
pub use capi::scbehavior::{MOUSE_BUTTONS, MOUSE_EVENTS, KEYBOARD_STATES, KEY_EVENTS};
use capi::sctypes::{HWINDOW, POINT, UINT, BOOL, RECT, LPCBYTE, LPVOID, INT};
use capi::scmsg::*;


/// Application-provided events to notify Sciter.
#[derive(Debug)]
pub enum Message {
	/// Creates an instance of Sciter assotiated with the given handle.
	Create {
		backend: GFX_LAYER,
		transparent: bool,
	},

	/// Destroys the engine instance.
	Destroy,

	/// Window size changes.
	Size {
		width: u32,
		height: u32,
	},

	/// Screen resolution changes.
	Resolution {
		ppi: u32,
	},

	/// Window focus event.
	Focus {
		enter: bool,
	},

	/// Time changes in order to process animations, timers and other timed things.
	Heartbit {
		milliseconds: u32,
	},

	/// Redraw the whole document.
	Redraw,

	/// Redraw the specific layer.
	Paint(PaintEvent),

	/// Render to bitmap.
	RenderTo(RenderEvent),

	/// Mouse input.
	Mouse(MouseEvent),

	/// Keyboard input.
	Keyboard(KeyboardEvent),
}

#[derive(Debug)]
pub struct MouseEvent {
	pub event: MOUSE_EVENTS,
	pub button: MOUSE_BUTTONS,
	pub modifiers: KEYBOARD_STATES,
	pub pos: POINT,
}

#[derive(Debug)]
pub struct KeyboardEvent {
	pub event: KEY_EVENTS,
	pub code: UINT,
	pub modifiers: KEYBOARD_STATES,
}

#[derive(Debug)]
pub struct PaintEvent {
	pub element: HELEMENT,
	pub foreground: bool,
}

pub struct RenderEvent
{
	pub layer: Option<PaintEvent>,
	pub callback: Box<dyn Fn(&RECT, &[u8])>,
}

impl std::fmt::Debug for RenderEvent {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt
			.debug_struct("RenderEvent")
			.field("layer", &self.layer)
			.field("callback", &"Box<dyn Fn>")
			.finish()
	}
}


/// Notify Sciter about UI-specific events.
///
/// `wnd` here is not a window handle but rather a window instance (pointer).
pub fn handle_message(wnd: HWINDOW, event: Message) -> bool
{
	let ok = match event {
		Message::Create { backend, transparent } => {
			let msg = SCITER_X_MSG_CREATE {
				header: SCITER_X_MSG_CODE::SXM_CREATE.into(),
				backend,
				transparent: transparent as BOOL,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Destroy => {
			let msg = SCITER_X_MSG_DESTROY {
				header: SCITER_X_MSG_CODE::SXM_DESTROY.into(),
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Size { width, height} => {
			let msg = SCITER_X_MSG_SIZE {
				header: SCITER_X_MSG_CODE::SXM_SIZE.into(),
				width,
				height,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Resolution { ppi } => {
			let msg = SCITER_X_MSG_RESOLUTION {
				header: SCITER_X_MSG_CODE::SXM_RESOLUTION.into(),
				ppi,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Focus { enter } => {
			let msg = SCITER_X_MSG_FOCUS {
				header: SCITER_X_MSG_CODE::SXM_FOCUS.into(),
				enter: enter as BOOL,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Heartbit { milliseconds } => {
			let msg = SCITER_X_MSG_HEARTBIT {
				header: SCITER_X_MSG_CODE::SXM_HEARTBIT.into(),
				time: milliseconds,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Mouse(params) => {
			let msg = SCITER_X_MSG_MOUSE {
				header: SCITER_X_MSG_CODE::SXM_MOUSE.into(),

				event: params.event,
				button: params.button,
				modifiers: params.modifiers as u32,
				pos: params.pos,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Keyboard(params) => {
			let msg = SCITER_X_MSG_KEY {
				header: SCITER_X_MSG_CODE::SXM_KEY.into(),

				event: params.event,
				code: params.code,
				modifiers: params.modifiers as u32,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Redraw => {
			use std::ptr;
			let msg = SCITER_X_MSG_PAINT {
				header: SCITER_X_MSG_CODE::SXM_PAINT.into(),
				element: ptr::null_mut(),
				isFore: true as BOOL,
				targetType: SCITER_PAINT_TARGET_TYPE::SPT_DEFAULT,
				param: ptr::null_mut(),
				callback: None,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::Paint(paint) => {
			use std::ptr;
			let msg = SCITER_X_MSG_PAINT {
				header: SCITER_X_MSG_CODE::SXM_PAINT.into(),
				element: paint.element,
				isFore: paint.foreground as BOOL,
				targetType: SCITER_PAINT_TARGET_TYPE::SPT_DEFAULT,
				param: ptr::null_mut(),
				callback: None,
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

		Message::RenderTo(paint) => {

			struct Callback {
				callback: Box<dyn Fn(&RECT, &[u8])>,
			}

			extern "system" fn inner(rgba: LPCBYTE, x: INT, y: INT, width: UINT, height: UINT, param: LPVOID)
			{
				if param.is_null() || rgba.is_null() { return; }

				let bitmap_area = RECT {
					left: x,
					top: y,
					right: x + width as INT,
					bottom: y + height as INT,
				};

				let bitmap_size = width * height * 4;
				let bitmap_data = unsafe { std::slice::from_raw_parts(rgba, bitmap_size as usize) };

				let param = param as *const Callback;
				let wrapper = unsafe { &*param };
				(wrapper.callback)(&bitmap_area, bitmap_data);
			}

			// let param = paint.callback.as_ref() as *const _ as LPVOID;
			// let param = Box::into_raw(paint.callback);
			// let param = param as *const _ as LPVOID;
			let wrapper = Callback {
				callback: paint.callback,
			};
			let param = &wrapper as *const _ as LPVOID;

			let element = paint.layer.unwrap_or(PaintEvent {
				element: std::ptr::null_mut(),
				foreground: false,
			});

			let msg = SCITER_X_MSG_PAINT {
				header: SCITER_X_MSG_CODE::SXM_PAINT.into(),
				element: element.element,
				isFore: element.foreground as BOOL,
				targetType: SCITER_PAINT_TARGET_TYPE::SPT_RECEIVER,
				param: param,
				callback: Some(inner),
			};
			(_API.SciterProcX)(wnd, &msg.header as *const _)
		},

	};

	ok != 0
}
