use sctypes::{LPVOID, UINT64};

pub type HVM = LPVOID;
pub type tiscript_value = UINT64;

#[repr(C)]
pub struct tiscript_native_interface
{
	create_vm: LPVOID,
}
