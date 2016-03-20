//! Common Sciter declarations.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use sctypes::*;
use scvalue::{VALUE};
use screquest::{HREQUEST};
use scdom::{HELEMENT};

//////////////////////////////////////////////////////////////////////////////////
pub type ID2D1RenderTarget = VOID;
pub type ID2D1Factory = VOID;
pub type IDWriteFactory = VOID;
pub type IDXGISwapChain = VOID;
pub type IDXGISurface = VOID;


#[repr(C)]
pub enum LOAD_RESULT {
  LOAD_OK,
  LOAD_DISCARD,
  LOAD_DELAYED,
  LOAD_MYSELF,
}

#[repr(C)]
pub enum SCITER_RT_OPTIONS
{
  SCITER_SMOOTH_SCROLL = 1,      // value:TRUE - enable, value:FALSE - disable, enabled by default
  SCITER_CONNECTION_TIMEOUT = 2, // value: milliseconds, connection timeout of http client
  SCITER_HTTPS_ERROR = 3,        // value: 0 - drop connection, 1 - use builtin dialog, 2 - accept connection silently
  SCITER_FONT_SMOOTHING = 4,     // value: 0 - system default, 1 - no smoothing, 2 - std smoothing, 3 - clear type

  SCITER_TRANSPARENT_WINDOW = 6, // Windows Aero support, value:
                                // 0 - normal drawing,
                                // 1 - window has transparent background after calls DwmExtendFrameIntoClientArea() or DwmEnableBlurBehindWindow().
  SCITER_SET_GPU_BLACKLIST  = 7, // hWnd = NULL,
                                // value = LPCBYTE, json - GPU black list, see: gpu-blacklist.json resource.
  SCITER_SET_SCRIPT_RUNTIME_FEATURES = 8, // value - combination of SCRIPT_RUNTIME_FEATURES flags.
  SCITER_SET_GFX_LAYER = 9,      // hWnd = NULL, value - GFX_LAYER
  SCITER_SET_DEBUG_MODE = 10,    // hWnd, value - TRUE/FALSE
  SCITER_SET_UX_THEMING = 11,    // hWnd = NULL, value - BOOL, TRUE - the engine will use "unisex" theme that is common for all platforms. 
                                // That UX theme is not using OS primitives for rendering input elements. Use it if you want exactly
                                // the same (modulo fonts) look-n-feel on all platforms.

  SCITER_ALPHA_WINDOW  = 12,     //  hWnd, value - TRUE/FALSE - window uses per pixel alpha (e.g. WS_EX_LAYERED/UpdateLayeredWindow() window)
}

#[repr(C)]
pub enum SCITER_CREATE_WINDOW_FLAGS {
  SW_CHILD      = (1 << 0), // child window only, if this flag is set all other flags ignored
  SW_TITLEBAR   = (1 << 1), // toplevel window, has titlebar
  SW_RESIZEABLE = (1 << 2), // has resizeable frame
  SW_TOOL       = (1 << 3), // is tool window
  SW_CONTROLS   = (1 << 4), // has minimize / maximize buttons
  SW_GLASSY     = (1 << 5), // glassy window ( DwmExtendFrameIntoClientArea on windows )
  SW_ALPHA      = (1 << 6), // transparent window ( e.g. WS_EX_LAYERED on Windows )
  SW_MAIN       = (1 << 7), // main window of the app, will terminate the app on close
  SW_POPUP      = (1 << 8), // the window is created as topmost window.
  SW_ENABLE_DEBUG = (1 << 9), // make this window inspector ready
  SW_OWNS_VM      = (1 << 10), // it has its own script VM
}

impl ::std::ops::BitOr for SCITER_CREATE_WINDOW_FLAGS {
  type Output = SCITER_CREATE_WINDOW_FLAGS;
  fn bitor(self, rhs: Self::Output) -> Self::Output {
    let rn = (self as UINT) | (rhs as UINT);
    unsafe { ::std::mem::transmute(rn) }
  }
}

#[repr(C)]
#[derive(Debug)]
pub enum SCITER_NOTIFICATION {
  SC_LOAD_DATA = 1,
  SC_DATA_LOADED = 2,
  SC_ATTACH_BEHAVIOR = 4,
  SC_ENGINE_DESTROYED = 5,
  SC_POSTED_NOTIFICATION = 6,
  SC_GRAPHICS_CRITICAL_FAILURE = 7,
}

pub struct SCN_LOAD_DATA
{
  pub code: UINT,
  pub hwnd: HWINDOW,

  pub uri: LPCWSTR,

  pub outData: LPCBYTE,
  pub outDataSize: UINT,
  pub dataType: UINT,

  pub requestId: HREQUEST,

  pub principal: HELEMENT,
  pub initiator: HELEMENT,
}

pub struct SCN_DATA_LOADED
{
  pub code: UINT,
  pub hwnd: HWINDOW,
  pub uri: LPCWSTR,
  pub data: LPCBYTE,
  pub dataSize: UINT,
  pub dataType: UINT,
  pub status: UINT,
}

pub struct SCN_ATTACH_BEHAVIOR
{
  pub code: UINT,
  pub hwnd: HWINDOW,

  pub element: HELEMENT,
  pub behaviorName: LPCSTR,

  pub elementProc: *mut ElementEventProc,
  pub elementTag: LPVOID,
}


#[repr(C)]
pub struct SCITER_CALLBACK_NOTIFICATION
{
	pub code: UINT,
	pub hwnd: HWINDOW,
}
pub type LPSCITER_CALLBACK_NOTIFICATION = *mut SCITER_CALLBACK_NOTIFICATION;

pub type SciterHostCallback = extern "stdcall" fn (pns: LPSCITER_CALLBACK_NOTIFICATION, callbackParam: LPVOID) -> UINT;
pub type LPSciterHostCallback = * const SciterHostCallback;

pub type SciterWindowDelegate = extern "stdcall" fn (hwnd: HWINDOW, msg: UINT, wParam: WPARAM, lParam: LPARAM, pParam: LPVOID, handled: * mut BOOL) -> LRESULT;

pub type ElementEventProc = extern "stdcall" fn (tag: LPVOID, he: HELEMENT, evtg: UINT, prms: LPVOID) -> BOOL;

#[repr(C)]
#[derive(Debug)]
pub enum OUTPUT_SUBSYTEMS
{
  DOM = 0,       // html parser & runtime
  CSSS,          // csss! parser & runtime
  CSS,           // css parser
  TIS,           // TIS parser & runtime
}

#[repr(C)]
#[derive(Debug)]
pub enum OUTPUT_SEVERITY
{
  INFO,
  WARNING,
  ERROR,
}

pub type DEBUG_OUTPUT_PROC = extern "stdcall" fn (param: LPVOID, subsystem: OUTPUT_SUBSYTEMS, severity: OUTPUT_SEVERITY, text: LPCWSTR, text_length: UINT);

pub type LPCWSTR_RECEIVER = extern "stdcall" fn (szstr: LPCWSTR, str_length: UINT, param: LPVOID);
pub type LPCSTR_RECEIVER = extern "stdcall" fn (szstr: LPCWSTR, str_length: UINT, param: LPVOID);
pub type LPCBYTE_RECEIVER = extern "stdcall" fn (szstr: LPCBYTE, str_length: UINT, param: LPVOID);

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

