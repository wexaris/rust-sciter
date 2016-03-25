//! Sciter's platform independent graphics interface.

#![allow(non_camel_case_types, non_snake_case)]

use capi::sctypes::{LPVOID};

MAKE_HANDLE!(HGFX, _HGFX);
MAKE_HANDLE!(HIMG, _HIMG);
MAKE_HANDLE!(HPATH, _HPATH);
MAKE_HANDLE!(HTEXT, _HTEXT);

#[repr(C)]
pub struct SciterGraphicsAPI
{
	imageCreate: LPVOID,
}
