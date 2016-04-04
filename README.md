# Rust bindings for Sciter

![Build status](https://ci.appveyor.com/api/projects/status/github/pravic/rust-sciter?svg=true)
[![Join the chat at https://gitter.im/pravic/rust-sciter](https://badges.gitter.im/pravic/rust-sciter.svg)](https://gitter.im/pravic/rust-sciter)

_Check [this page](http://sciter.com/developers/sciter-sdk-bindings/) for another language bindings._

----

Sciter is an embeddable [multiplatform](http://sciter.com/sciter/crossplatform/) HTML/CSS/script engine with GPU accelerated rendering designed to render modern desktop application UI. It's a compact, single dll/dylib/so file (4-8 mb), engine without any additional dependencies.

Check the [screenshot gallery](https://github.com/oskca/sciter#sciter-desktop-ui-examples) of the desktop UI examples.

Physically Sciter is a mono library which contains:

* [HTML and CSS](http://sciter.com/developers/for-web-programmers/) rendering engine based on the H-SMILE core used in [HTMLayout](http://www.terrainformatica.com/htmlayout/main.whtm),
* JavaScript alike [Scripting engine](http://sciter.com/developers/sciter-docs/) – core of [TIScript](http://sciter.com/developers/for-web-programmers/tiscript-vs-javascript/) which by itself is based on [c-smile](http://c-smile.sourceforge.net/) engine,
* Persistent [Database](http://sciter.com/docs/content/script/Storage.htm) (a.k.a. [JSON DB](http://terrainformatica.com/2006/10/what-the-hell-is-that-json-db/)) based on excellent DB products of [Konstantin Knizhnik](http://garret.ru/databases.html).
* [Graphics](http://sciter.com/docs/content/sciter/Graphics.htm) module using excellent AGG library of Maxim Shemanarev at [www.antigrain.com](http://antigrain.com).
* Network communication module, it relies on [Libcurl](http://curl.haxx.se/), the Great.


Internally it contains the following modules:

* **CSS** – CSS parser and collection of parsed CSS rules, etc.
* **HTML DOM** – HTML parser and DOM tree implementation.
* **layout managers** – collection of various layout managers – text layout, default block layout, flex layouts. Support of positioned floating elements is also here. This module does layout calculations heavy lifting. This module is also responsible for rendering of layouts.
* **input behaviors** – collection of built-in behaviors – code behind "active" DOM elements: `<input>`, `<select>`, `<textarea>`, etc.
* **script module** – source-to-bytecode compiler and virtual machine (VM) with compacting garbage collector (GC). This module also contains runtime implementation of standard classes and objects: Array, Object, Function and others.
* **script DOM** – runtime classes that expose DOM and DOM view (a.k.a. window) to the script.
* **graphics abstraction layer** – abstract graphics implementation that isolates modules above from particular platform details
    * Direct2D/DirectWrite graphics backend implementation (Windows);
    * GDI+ graphics backend implementation (Windows);
    * CoreGraphics backend implementation (Mac OS X);
    * Cairo backend implementation (GTK on all platforms including Linuxes);
* **core primitives** – set of common primitives: string, arrays, hash maps and so on.


Sciter supports all standard elements defined in HTML5 specification [with some additions](http://sciter.com/developers/for-web-programmers/). CSS extended to better support Desktop UI development, e.g. flow and flex units, vertical and horizontal alignment, OS theming.

[Sciter SDK](http://sciter.com/download/) comes with demo "browser" with builtin DOM inspector, script debugger and documentation browser:

![Sciter tools](http://sciter.com/images/sciter-tools.png)

Check <http://sciter.com> website and its [documentation resources](http://sciter.com/developers/) for engine principles, architecture and more.


## Getting started:

1. Download [Sciter SDK](http://sciter.com/download/) and extract it somewhere.
2. Add target platform binaries to PATH (`bin`, `bin.osx` or `bin.gtk`) and install Sciter shared library to your LIBRARY_PATH.
3. Add this to your Cargo.toml: `sciter = "*"`.
4. Build library and run the minimal sciter sample: `cargo run --example minimal`.

## Brief look:

Here is a minimal sciter app:

```rust
extern crate sciter;

fn main() {
    let mut frame = sciter::Window::new();
    frame.load_file("minimal.htm");
    frame.run_app(true);
}
```

It looks similar like this:

![Minimal sciter sample](http://i.imgur.com/ojcM5JJ.png)

### Interoperability

In respect of [tiscript](http://www.codeproject.com/Articles/33662/TIScript-language-a-gentle-extension-of-JavaScript) functions calling:
```rust
use sciter::{Element, Value};

let root = Element::from_window(hwnd);
let result: Value = root.call_function("namespace.name", &make_args!(1,"2",3));
```

Calling rust from script can be implemented as following:
```rust
struct Handler;

impl Handler {
  fn calc_sum(&self, a: i32, b: i32) -> i32 {
    a + b
  }
}

impl sciter::EventHandler for Handler {
  dispatch_script_call! {
    fn calc_sum(i32, i32);
  }
}
```

And we can access this function from script:
```js
// `view` represents window where script is runnung.
// `stdout` stream is a standard output stream (shell or debugger console, for example)

stdout.printf("2 + 3 = %d\n", view.calc_sum(2, 3));
```

_Check [rust-sciter/examples](https://github.com/pravic/rust-sciter/tree/master/examples) folder for more complex usage_.


## [Library documentation](http://pravic.github.io/rust-sciter/).


## What supported right now:

* [x] [sciter::window](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-window.hpp) which brings together window creation, host and event handlers
* [x] [sciter::host](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-host-callback.h) with basic event handling, needs to be redesigned
* [x] [sciter::event_handler](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-behavior.h) with event handling and auto dispatching script calls to naive code
* [x] [sciter::dom](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-dom.hpp) for HTML DOM access and manipulation methods
* [x] [sciter::value](https://github.com/c-smile/sciter-sdk/blob/master/include/value.hpp) Rust wrapper with sciter::script_error and sciter::native_function support
* [ ] [sciter::behavior_factory](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-behavior.h) - global factory for native behaviors
* [ ] [sciter::graphics](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-graphics.hpp) - platform independent graphics native interface (can be used in native behaviors)
* [ ] [sciter::request](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-request.hpp) - resource request object, used for custom resource downloading and handling
* [ ] [sciter::video](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-video-api.h) - custom video rendering
* [ ] [sciter::archive](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-host-callback.h) - Sciter's compressed archive produced by sdk/bin/packfolder


### Platforms:

* [x] Windows
* [x] OSX
* [x] Linux

