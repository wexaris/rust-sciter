// This component uses Sciter Engine,
// copyright Terra Informatica Software, Inc.
// (http://terrainformatica.com/).

/*!
# Rust bindings library for Sciter engine.

Sciter is an embeddable [multiplatform](http://sciter.com/sciter/crossplatform/) HTML/CSS/script engine
with GPU accelerated rendering designed to render modern desktop application UI.
It's a compact, single dll/dylib/so file (4-8 mb), engine without any additional dependencies.

Check the [screenshot gallery](https://github.com/oskca/sciter#sciter-desktop-ui-examples) of the desktop UI examples.

Sciter supports all standard elements defined in HTML5 specification [with some additions](http://sciter.com/developers/for-web-programmers/).
CSS extended to better support Desktop UI development, e.g. flow and flex units, vertical and horizontal alignment, OS theming.

[Sciter SDK](http://sciter.com/download/) comes with demo "browser" with builtin DOM inspector, script debugger and documentation browser:

![Sciter tools](http://sciter.com/images/sciter-tools.png)

Check <http://sciter.com> website and its [documentation resources](http://sciter.com/developers/) for engine principles, architecture and more.

## Brief look:

Here is a minimal sciter app:

```no_run
extern crate sciter;

fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("minimal.htm");
    frame.run_app(true);
}
```

It looks similar like this:

![Minimal sciter sample](http://i.imgur.com/ojcM5JJ.png)

Check [rust-sciter/examples](https://github.com/pravic/rust-sciter/tree/master/examples) folder for more complex usage
and module-level sections for the guides about:

* [Window](window/index.html) creation
* [Behaviors](dom/event/index.html) and event handling
* [DOM](dom/index.html) access methods
* Sciter [Value](value/index.html) interface

.
*/

#![doc(html_logo_url = "http://sciter.com/screenshots/slide-sciter-osx.png",
       html_favicon_url = "http://sciter.com/wp-content/themes/sciter/!images/favicon.ico")]

// documentation test:
// #![warn(missing_docs)]

/* Macros */

#[cfg(target_os="macos")]
#[macro_use] extern crate objc;

#[macro_use] extern crate lazy_static;


#[macro_use] mod macros;

mod capi;
pub use capi::scdom::{HELEMENT};

/* Rust interface */
mod platform;
mod eventhandler;

pub mod window;
pub mod host;
pub mod value;
pub mod utf;
pub mod dom;
pub mod types;

pub use dom::Element;
pub use dom::event::EventHandler;
pub use host::{Host, HostHandler};
pub use value::{Value, FromValue};
pub use window::Window;



/* Loader */
pub use capi::scapi::{ISciterAPI};


#[cfg(windows)]
mod ext {
	// Note:
	// Sciter 4.x shipped with universal "sciter.dll" library for different builds:
	// bin/32, bin/64, bin/skia32, bin/skia64
	// However it is quiet unconvenient now (e.g. we can not put x64 and x86 builds in %PATH%)
	//
	use capi::scapi::{ISciterAPI};
	#[link(name="sciter", kind="dylib")]
	extern "system" { pub fn SciterAPI() -> *const ISciterAPI;	}
}

#[cfg(all(target_os="linux", target_arch="x86_64"))]
mod ext {
	// Note:
	// Since 3.3.1.6 library name was changed to "libsciter".
	//
	use capi::scapi::{ISciterAPI};
	#[link(name="libsciter-gtk-64", kind="dylib")]
	extern "system" { pub fn SciterAPI() -> *const ISciterAPI;	}
}

#[cfg(all(target_os="macos", target_arch="x86_64"))]
mod ext {
	use capi::scapi::{ISciterAPI};
	#[link(name="sciter-osx-64", kind = "dylib")]
	extern "system" { pub fn SciterAPI() -> *const ISciterAPI;	}
}

#[allow(non_snake_case)]
#[doc(hidden)]
/// Getting ISciterAPI reference, can be used for manual API calling.
pub fn SciterAPI<'a>() -> &'a ISciterAPI {
	let ap = unsafe {
		let p = ext::SciterAPI();
		&*p
	};
	return ap;
}


lazy_static! {
	static ref _API: &'static ISciterAPI = { SciterAPI() };
}

/// Sciter engine version number (e.g. `0x03030200`).
pub fn version_num() -> u32 {
	let v1 = (_API.SciterVersion)(true);
	let v2 = (_API.SciterVersion)(false);
	let num = ((v1 >> 16) << 24) | ((v1 & 0xFFFF) << 16) | ((v2 >> 16) << 8) | (v2 & 0xFFFF);
	return num;
}

/// Sciter engine version string (e.g. "`3.3.2.0`").
pub fn version() -> String {
	let v1 = (_API.SciterVersion)(true);
	let v2 = (_API.SciterVersion)(false);
	let num = [v1 >> 16, v1 & 0xFFFF, v2 >> 16, v2 & 0xFFFF];
	let version = format!("{}.{}.{}.{}", num[0], num[1], num[2], num[3]);
	return version;
}
