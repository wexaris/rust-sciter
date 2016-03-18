use sctypes::{LPVOID};

pub type HGFX = LPVOID;
pub type HIMG = LPVOID;
pub type HPATH = LPVOID;
pub type HTEXT = LPVOID;

#[repr(C)]
pub struct SciterGraphicsAPI
{
	imageCreate: LPVOID,
}
