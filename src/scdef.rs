#![allow(dead_code)]

use sctypes::*;
use scvalue::{VALUE};


//////////////////////////////////////////////////////////////////////////////////
pub type ID2D1RenderTarget = VOID;
pub type ID2D1Factory = VOID;
pub type IDWriteFactory = VOID;
pub type IDXGISwapChain = VOID;
pub type IDXGISurface = VOID;


#[repr(C)]
pub struct SCITER_CALLBACK_NOTIFICATION
{
	pub code: UINT,
	pub hwnd: HWINDOW,
}
pub type LPSCITER_CALLBACK_NOTIFICATION = * const SCITER_CALLBACK_NOTIFICATION;

pub type SciterHostCallback = extern "stdcall" fn (pns: LPSCITER_CALLBACK_NOTIFICATION, callbackParam: LPVOID) -> UINT;
pub type LPSciterHostCallback = * const SciterHostCallback;

pub type SciterWindowDelegate = extern "stdcall" fn (hwnd: HWINDOW, msg: UINT, wParam: WPARAM, lParam: LPARAM, pParam: LPVOID, handled: * mut BOOL) -> LRESULT;

#[repr(C)]
pub enum OUTPUT_SUBSYTEMS
{
   OT_DOM = 0,       // html parser & runtime
   OT_CSSS,          // csss! parser & runtime
   OT_CSS,           // css parser
   OT_TIS,           // TIS parser & runtime
}

#[repr(C)]
pub enum OUTPUT_SEVERITY
{
  OS_INFO,
  OS_WARNING,
  OS_ERROR,
}

pub type DEBUG_OUTPUT_PROC = extern "stdcall" fn (param: LPVOID, subsystem: OUTPUT_SUBSYTEMS, severity: UINT, text: LPCWSTR, text_length: UINT) -> VOID;

pub type LPCWSTR_RECEIVER = extern "stdcall" fn (szstr: LPCWSTR, str_length: UINT, param: LPVOID) -> VOID;
pub type LPCSTR_RECEIVER = extern "stdcall" fn (szstr: LPCWSTR, str_length: UINT, param: LPVOID) -> VOID;
pub type LPCBYTE_RECEIVER = extern "stdcall" fn (szstr: LPCBYTE, str_length: UINT, param: LPVOID) -> VOID;

#[repr(C)]
pub struct METHOD_PARAMS {
	methodID: UINT,
}

#[repr(C)]
pub struct REQUEST_PARAM { 
  name: LPCWSTR,
  value: LPCWSTR,
}

pub type KeyValueCallback = extern "stdcall" fn (param: LPVOID, pkey: *const VALUE, pval: *const VALUE) -> BOOL;

