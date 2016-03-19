//! Sciter's get resource request object - represents requests made by Element/View.request() functions.

#![allow(non_camel_case_types, non_snake_case)]

use sctypes::{LPVOID};

pub type HREQUEST = LPVOID;

#[repr(C)]
pub struct SciterRequestAPI
{
	RequestUse: LPVOID,
}
