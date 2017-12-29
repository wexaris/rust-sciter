//! Sciter's platform independent graphics interface.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use capi::sctypes::*;
use capi::scdef::{GFX_LAYER, ELEMENT_BITMAP_RECEIVER};
use capi::scdom::HELEMENT;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum SCITER_X_MSG_CODE
{
  SXM_CREATE  = 0,
  SXM_DESTROY = 1,
  SXM_SIZE    = 2,
  SXM_PAINT   = 3,
}

#[repr(C)]
/// Common header of message structures passed to `SciterProcX`.
pub struct SCITER_X_MSG
{
	pub msg: SCITER_X_MSG_CODE,
}

#[repr(C)]
/// Message to create the specific Sciter backend.
pub struct SCITER_X_MSG_CREATE
{
	pub header: SCITER_X_MSG,
	pub backend: GFX_LAYER,
	pub transparent: BOOL,
}

#[repr(C)]
/// Message to destroy the current Sciter backend.
pub struct SCITER_X_MSG_DESTROY
{
	pub header: SCITER_X_MSG,
}

#[repr(C)]
/// Message to notify Sciter about view resize.
pub struct SCITER_X_MSG_SIZE
{
	pub header: SCITER_X_MSG,
	pub width: UINT,
	pub height: UINT,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug, PartialOrd, PartialEq)]
/// `SCITER_X_MSG_PAINT` target identifier.
pub enum SCITER_PAINT_TARGET_TYPE
{
	/** default rendering target - window surface */
	SPT_DEFAULT   = 0,
	/** target::receiver fields are valid */
	SPT_RECEIVER  = 1,
	/** target::dc is valid */
	SPT_DC        = 2,
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
	pub callback: ELEMENT_BITMAP_RECEIVER,
	// }
}
