//! Sciter.Lite interface.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use capi::sctypes::*;
use capi::scdef::{GFX_LAYER, ELEMENT_BITMAP_RECEIVER};
use capi::scdom::HELEMENT;
use capi::scbehavior::{MOUSE_BUTTONS, MOUSE_EVENTS, KEY_EVENTS};


#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum SCITER_X_MSG_CODE
{
  SXM_CREATE  = 0,
  SXM_DESTROY = 1,
  SXM_SIZE    = 2,
  SXM_PAINT   = 3,
  SXM_RESOLUTION = 4,
  SXM_HEARTBIT = 5,
  SXM_MOUSE = 6,
  SXM_KEY = 7,
  SXM_FOCUS = 8,
}

#[repr(C)]
#[derive(Debug)]
/// Common header of message structures passed to `SciterProcX`.
pub struct SCITER_X_MSG
{
	pub msg: SCITER_X_MSG_CODE,
}

impl From<SCITER_X_MSG_CODE> for SCITER_X_MSG {
	fn from(code: SCITER_X_MSG_CODE) -> Self {
		Self { msg: code }
	}
}

#[repr(C)]
#[derive(Debug)]
/// Message to create the specific Sciter backend.
pub struct SCITER_X_MSG_CREATE
{
	pub header: SCITER_X_MSG,
	pub backend: GFX_LAYER,
	pub transparent: BOOL,
}

#[repr(C)]
#[derive(Debug)]
/// Message to destroy the current Sciter backend.
pub struct SCITER_X_MSG_DESTROY
{
	pub header: SCITER_X_MSG,
}

#[repr(C)]
#[derive(Debug)]
/// Message to notify Sciter about view resize.
pub struct SCITER_X_MSG_SIZE
{
	pub header: SCITER_X_MSG,
	pub width: UINT,
	pub height: UINT,
}

#[repr(C)]
#[derive(Debug)]
/// Message to notify Sciter about screen resolution change.
pub struct SCITER_X_MSG_RESOLUTION
{
	pub header: SCITER_X_MSG,

	/// Pixels per inch.
	pub ppi: UINT,
}

#[repr(C)]
#[derive(Debug)]
/// Message to notify Sciter about mouse input.
pub struct SCITER_X_MSG_MOUSE
{
	pub header: SCITER_X_MSG,

	pub event: MOUSE_EVENTS,
	pub button: MOUSE_BUTTONS,
	pub modifiers: UINT,
	pub pos: POINT,
}

#[repr(C)]
#[derive(Debug)]
/// Message to notify Sciter about keyboard input.
pub struct SCITER_X_MSG_KEY
{
	pub header: SCITER_X_MSG,

	pub event: KEY_EVENTS,
	pub code: UINT,
	pub modifiers: UINT,
}

#[repr(C)]
#[derive(Debug)]
/// Message to notify Sciter about window focus change.
pub struct SCITER_X_MSG_FOCUS
{
	pub header: SCITER_X_MSG,

	pub enter: BOOL,
}

#[repr(C)]
#[derive(Debug)]
/// Give Sciter a chance to process animations, timers and other timed things.
pub struct SCITER_X_MSG_HEARTBIT
{
	pub header: SCITER_X_MSG,

	/// Absolute time in milliseconds.
	pub time: UINT,
}



#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
/// `SCITER_X_MSG_PAINT` target identifier.
pub enum SCITER_PAINT_TARGET_TYPE
{
	/// default rendering target - OpenGL window surface
	SPT_DEFAULT   = 0,

	/// target::receiver
	SPT_RECEIVER  = 1,
}

/// Message to paint view to the provided target (HDC or callback).
#[repr(C)]
pub struct SCITER_X_MSG_PAINT
{
	pub header: SCITER_X_MSG,
	pub element: HELEMENT,
	pub isFore: BOOL,
	pub targetType: SCITER_PAINT_TARGET_TYPE,

	// union {
	// HDC or LPVOID
	pub param: LPVOID,
	pub callback: Option<ELEMENT_BITMAP_RECEIVER>,
	// }
}

