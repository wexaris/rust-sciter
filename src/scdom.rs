//! DOM access methods, C interface.

#![allow(non_camel_case_types, non_snake_case)]

use sctypes::*;

MAKE_HANDLE!(HELEMENT, _HELEMENT);
MAKE_HANDLE!(HNODE, _HNODE);

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum SCDOM_RESULT {
	OK = 0,
	INVALID_HWND = 1,
	INVALID_HANDLE = 2,
	PASSIVE_HANDLE = 3,
	INVALID_PARAMETER = 4,
	OPERATION_FAILED = 5,
	OK_NOT_HANDLED = -1,
}

#[repr(C)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum SET_ELEMENT_HTML
{
  SIH_REPLACE_CONTENT     = 0,
  SIH_INSERT_AT_START     = 1,
  SIH_APPEND_AFTER_LAST   = 2,
  SOH_REPLACE             = 3,
  SOH_INSERT_BEFORE       = 4,
  SOH_INSERT_AFTER        = 5,
}


pub type SciterElementCallback = extern "stdcall" fn (he: HELEMENT, param: LPVOID) -> BOOL;

pub type ELEMENT_COMPARATOR = extern "stdcall" fn (he1: HELEMENT, he2: HELEMENT, param: LPVOID) -> INT;
