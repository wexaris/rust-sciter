use capi::sctypes::*;
use capi::scbehavior::*;
use capi::scdom::{HELEMENT};
use value::Value;
use dom::event::{EventHandler, EventReason};

#[repr(C)]
pub struct WindowHandler<T>
{
	pub hwnd: HWINDOW,
	pub handler: T,
}

pub extern "system" fn _event_handler_window_proc<T: EventHandler>(tag: LPVOID, _he: ::capi::scdom::HELEMENT, evtg: UINT, params: LPVOID) -> BOOL
{
	use capi::scbehavior::*;
	use capi::scdom::HELEMENT;

	let boxed = tag as *mut WindowHandler<T>;
	let mut tuple: &mut WindowHandler<T> = unsafe { &mut *boxed };

	let root = ::dom::Element::from_window(tuple.hwnd);
	let hroot: HELEMENT = if root.is_ok() {
		root.unwrap().as_ptr()
	} else {
		::std::ptr::null_mut()
	};

	// custom initialization
	let evt: EVENT_GROUPS = unsafe { ::std::mem::transmute(evtg) };
	match evt {
		EVENT_GROUPS::HANDLE_INITIALIZATION => {
			let me = &mut tuple.handler;
			let scnm = params as *const INITIALIZATION_EVENTS;
			let cmd = unsafe { *scnm };
			match cmd {
				INITIALIZATION_EVENTS::BEHAVIOR_DETACH => {
					me.detached(hroot);

					// here we dropping our tuple
					let ptr = unsafe { Box::from_raw(boxed) };
					drop(ptr);
					return true as BOOL;
				},

				INITIALIZATION_EVENTS::BEHAVIOR_ATTACH => {
					me.attached(hroot);
				},
			};
			return true as BOOL;
		},

		_ => (),
	};

	return ::eventhandler::_event_handler_proc::<T>(&mut tuple.handler as *mut T as LPVOID, hroot, evtg, params);
}


pub extern "system" fn _event_handler_proc<T: EventHandler>(tag: LPVOID, he: HELEMENT, evtg: UINT, params: LPVOID) -> BOOL
{
	// reconstruct pointer to Handler
	let boxed = tag as *mut T;
	let me = unsafe { &mut *boxed };

	let evtg : EVENT_GROUPS = unsafe { ::std::mem::transmute(evtg) };
	assert!(!he.is_null() || evtg == EVENT_GROUPS::SUBSCRIPTIONS_REQUEST);

	let result = match evtg {

		EVENT_GROUPS::SUBSCRIPTIONS_REQUEST => {
			let scnm = params as *mut EVENT_GROUPS;
			let nm = unsafe {&mut *scnm};
			let handled = me.get_subscription();
			if let Some(needed) = handled {
				*nm = needed;
			}
			handled.is_some()
		},

		EVENT_GROUPS::HANDLE_INITIALIZATION => {
			let scnm = params as *const INITIALIZATION_EVENTS;
			let cmd = unsafe { *scnm };
			match cmd {
				INITIALIZATION_EVENTS::BEHAVIOR_DETACH => {
					me.detached(he);

					// here we dropping our handler
					let ptr = unsafe { Box::from_raw(boxed) };
					drop(ptr);
					return true as BOOL;
				},

				INITIALIZATION_EVENTS::BEHAVIOR_ATTACH => {
					me.attached(he);
				},
			};
			true
		},

		EVENT_GROUPS::HANDLE_BEHAVIOR_EVENT => {
			let scnm = params as *const BEHAVIOR_EVENT_PARAMS;
			let nm = unsafe { &*scnm };

			let code :BEHAVIOR_EVENTS = unsafe{ ::std::mem::transmute(nm.cmd & 0x00FFF) };
			let phase: PHASE_MASK = unsafe { ::std::mem::transmute(nm.cmd & 0xFFFFF000) };
			let reason = match code {
				BEHAVIOR_EVENTS::EDIT_VALUE_CHANGED | BEHAVIOR_EVENTS::EDIT_VALUE_CHANGING => {
					let reason: EDIT_CHANGED_REASON = unsafe{ ::std::mem::transmute(nm.reason as UINT) };
					EventReason::EditChanged(reason)
				},

				BEHAVIOR_EVENTS::VIDEO_BIND_RQ => {
					EventReason::VideoBind(nm.reason as LPVOID)
				}

				_ => {
					let reason: EVENT_REASON = unsafe{ ::std::mem::transmute(nm.reason as UINT) };
					EventReason::General(reason)
				}
			};

			if phase == PHASE_MASK::SINKING {	// catch this only once
				match code {
					BEHAVIOR_EVENTS::DOCUMENT_COMPLETE => {
						me.document_complete(he, nm.heTarget);
					},
					BEHAVIOR_EVENTS::DOCUMENT_CLOSE => {
						me.document_close(he, nm.heTarget);
					},
					_ => ()
				};
			}

			let handled = me.on_event(he, nm.he, nm.heTarget, code, phase, reason);
			handled
		},

		EVENT_GROUPS::HANDLE_SCRIPTING_METHOD_CALL => {
			let scnm = params as *mut SCRIPTING_METHOD_PARAMS;
			let nm = unsafe { &mut *scnm };
			let name = u2s!(nm.name);
			let argv = Value::unpack_from(nm.argv, nm.argc);
			let rv = me.on_script_call(he, &name, &argv);
			let handled = if let Some(v) = rv {
				v.pack_to(&mut nm.result);
				true
			} else {
				false
			};
			handled
		},

		EVENT_GROUPS::HANDLE_TIMER => {
			let scnm = params as *const TIMER_PARAMS;
			let nm = unsafe { & *scnm };
			let handled = me.on_timer(he, nm.timerId as u64);
			handled
		},

		_ => false
	};
	return result as BOOL;
}
