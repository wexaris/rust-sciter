#![allow(dead_code)]

use sctypes::*;

pub type HELEMENT = LPVOID;
pub type HNODE = LPVOID;

#[repr(C)]
pub enum SCDOM_RESULT {
	SCDOM_OK = 0,
	SCDOM_INVALID_HWND = 1,
	SCDOM_INVALID_HANDLE = 2,
	SCDOM_PASSIVE_HANDLE = 3,
	SCDOM_INVALID_PARAMETER = 4,
	SCDOM_OPERATION_FAILED = 5,
	SCDOM_OK_NOT_HANDLED = -1,
}

pub type SciterElementCallback = extern "stdcall" fn (he: HELEMENT, param: LPVOID) -> BOOL;

pub type ELEMENT_COMPARATOR = extern "stdcall" fn (he1: HELEMENT, he2: HELEMENT, param: LPVOID) -> INT;
