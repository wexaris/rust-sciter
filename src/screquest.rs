use sctypes::{LPVOID};

pub type HREQUEST = LPVOID;

#[repr(C)]
pub struct SciterRequestAPI
{
	RequestUse: LPVOID,
}
