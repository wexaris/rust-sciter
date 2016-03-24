/*!
# Rust bindings library for Sciter engine.

Sciter is an embeddable [multiplatform](http://sciter.com/sciter/crossplatform/) HTML/CSS/script engine
with GPU accelerated rendering designed to render modern desktop application UI.
It's a compact, single dll/dylib/so file (4-8 mb), engine without any additional dependencies.

Sciter supports all standard elements defined in HTML5 specification [with some additions](http://sciter.com/developers/for-web-programmers/).
CSS extended to better support Desktop UI development, e.g. flow and flex units, vertical and horizontal alignment, OS theming.

[Sciter SDK](http://sciter.com/download/) comes with demo "browser" with builtin DOM inspector, script debugger and documentation browser:

![Sciter tools](http://sciter.com/images/sciter-tools.png)

Check <http://sciter.com> website and its [documentation resources](http://sciter.com/developers/) for engine principles, architecture and more.
.
*/

/* Macros */

#[macro_use] extern crate lazy_static;

#[macro_use]
mod macros;


/* C interface headers */
mod scapi;
mod scbehavior;
mod scdef;
mod scdom;
mod scgraphics;
mod screquest;
mod sctiscript;
mod sctypes;
mod scvalue;
mod schandler;

pub use scdef::{LOAD_RESULT, SCN_LOAD_DATA, SCN_DATA_LOADED, SCN_ATTACH_BEHAVIOR, OUTPUT_SUBSYTEMS, OUTPUT_SEVERITY};
pub use scdom::{HELEMENT};

/* Rust interface */
mod platform;
mod eventhandler;

pub mod window;
pub mod host;
pub mod value;
pub mod utf;
pub mod dom;

pub use window::Window;
pub use dom::Element;
pub use value::Value;
pub use dom::event::EventHandler;
pub use host::HostHandler;

/* Loader */

pub use self::scapi::{ISciterAPI};

#[cfg(all(windows, target_arch="x86"))]
mod ext {
	#[link(name="sciter32")]
	extern "stdcall" { pub fn SciterAPI() -> *const ::scapi::ISciterAPI;	}
}

#[cfg(all(windows, target_arch="x86_64"))]
mod ext {
	#[link(name="sciter64")]
	extern "stdcall" { pub fn SciterAPI() -> *const ::scapi::ISciterAPI;	}
}

#[allow(non_snake_case)]
pub fn SciterAPI<'a>() -> &'a ::scapi::ISciterAPI {
	let ap = unsafe {
		let p = ext::SciterAPI();
		&*p
	};
	return ap;
}


lazy_static! {
	static ref _API: &'static ISciterAPI = { SciterAPI() };
}
