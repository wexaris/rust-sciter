# Rust bindings for Sciter

Sciter is an embeddable [multiplatform](http://sciter.com/sciter/crossplatform/) HTML/CSS/script engine with GPU accelerated rendering designed to render modern desktop application UI. It's a compact, single dll/dylib/so file (4-8 mb), engine without any additional dependencies.

Check the [screenshot gallery](https://github.com/oskca/sciter#sciter-desktop-ui-examples) of the desktop UI examples.

Physically Sciter is a mono library which contains:

* [HTML and CSS](http://sciter.com/developers/for-web-programmers/) rendering engine based on the H-SMILE core used in [HTMLayout](http://www.terrainformatica.com/htmlayout/main.whtm),
* JavaScript alike [Scripting engine](http://sciter.com/developers/sciter-docs/) – core of [TIScript](http://sciter.com/developers/for-web-programmers/tiscript-vs-javascript/) which by itself is based on [c-smile](http://c-smile.sourceforge.net/) engine,
* Persistent [Database](http://sciter.com/docs/content/script/Storage.htm) (a.k.a. [JSON DB](http://terrainformatica.com/2006/10/what-the-hell-is-that-json-db/)) based on excellent DB products of [Konstantin Knizhnik](http://garret.ru/databases.html).
* [Graphics](http://sciter.com/docs/content/sciter/Graphics.htm) module using excellent AGG library of Maxim Shemanarev at [www.antigrain.com](http://antigrain.com).
* Network communication module, it relies on [Libcurl](http://curl.haxx.se/), the Great.

Sciter supports all standard elements defined in HTML5 specification [with some additions](http://sciter.com/developers/for-web-programmers/). CSS extended to better support Desktop UI development, e.g. flow and flex units, vertical and horizontal alignment, OS theming.

[Sciter SDK](http://sciter.com/download/) comes with demo "browser" with builtin DOM inspector, script debugger and documentation browser:

![Sciter tools](http://sciter.com/images/sciter-tools.png)

Check <http://sciter.com> website and its [documentation resources](http://sciter.com/developers/) for engine principles, architecture and more.


## Getting started:

TBD

## Brief look:

TBD




## What supported right now:

* [ ] [sciter::window](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-window.hpp) which brings together window creation, host and event handlers
* [ ] [sciter::host](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-host-callback.h) extensible implementation with transparent script calls from python code
* [ ] [sciter::event_handler](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-behavior.h) with basic event handling (attached, document_complete, on_script_call), additional handlers will come
* [ ] [sciter::dom](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-dom.hpp) for HTML DOM access and manipulation methods
* [ ] [sciter::value](https://github.com/c-smile/sciter-sdk/blob/master/include/value.hpp) pythonic wrapper with sciter::script_error and sciter::native_function support
* [ ] [sciter::graphics](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-graphics.hpp) - platform independent graphics native interface (can be used in native behaviors)
* [ ] [sciter::request](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-request.hpp) - resource request object, used for custom resource downloading and handling
* [ ] [sciter::video](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-video-api.h) - custom video rendering
* [ ] [sciter::archive](https://github.com/c-smile/sciter-sdk/blob/master/include/sciter-x-host-callback.h) - Sciter's compressed archive produced by sdk/bin/packfolder


### Platforms:

* [ ] Windows (in development)
* [ ] OSX
* [ ] Linux

