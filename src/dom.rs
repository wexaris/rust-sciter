//! DOM access methods.
//!
/*!

## DOM access methods.

Let’s assume you already integrated Sciter in your application and so you have Sciter window with loaded content.

From Sciter point of view loaded document is a tree of DOM elements (elements of Document Object Model).
Sciter builds this tree while loading/parsing of input HTML.
As a rule each tag in source HTML gets matching DOM element (there are exceptions, see below).

You can change text, attributes, state flags of DOM elements;
add new or remove existing DOM elements.
You also can attach your own DOM event handlers to DOM elements to receive events and notifications.

Therefore your UI in Sciter is a collection of uniform DOM elements that can be styled by CSS and manipulated by native or script code.

## Basic operations

To access the DOM tree we need to get reference of its root element
(root element is an element representing <html> tag in HTML source).

```
let root = Element::from_window(hwnd);
assert_eq(root.get_tag(), "html");
```

By having root element reference we are able to access any other element in the tree
using various access and search functions like SciterGetNthChild, SciterSelectElements, …
All of them are wrapped into methods of `dom::Element`.
Here is how you would get reference to first <div> element with class "sidebar" using CSS selectors:

```
let sidebar = root.find_first("div.sidebar").unwrap();
```

The same in script:

```ignore
var sidebar = self.select("div.sidebar"); // or
var sidebar = self.$(div.sidebar); // using stringizer select variant
```

## DOM element operations

You can change *text* or HTML of DOM element:

```
if let Some(el) = root.find_first("#cancel") {
	el.set_text("Abort!");
	el.set_html(br##"<img src="http://lorempixel.com/32/32/cats/" alt="some cat"/>"##, None);
}
```

The same but in script:

```ignore
var el = ...;
el.text = "Hello world"; // text
el.html = "Hello <b>wrold</b>!"; // inner html
```

You can get or set DOM *attributes* of any DOM element:

```
let val = el.get_attribute("class").unwrap();
el.set_attribute("class", "new-class");
```

To *remove* existing DOM element (detach it from the DOM) you will do this:

```
el.detach();
```

and when code will live scope where the `el` variable is defined the DOM element will be destroyed.

Creation and population of DOM elements looks like this:

```
let p = Element::with_text("p", "Hello"); // create <p> element
el.append(p); // append it to existing element, or insert() ...
```

And in script:

```ignore
var p = new Element("p", "Hello");
el.append(p);
```

To change runtime state flags of DOM element we do something like this:

```
el.set_state(STATE_VISITED);
```

And in script:

```ignore
el.state.visited = true;
```

(after such call the element will match `:visited` CSS selector)

## Getting and setting values of DOM elements.

By default value of DOM element is its text but some DOM elements may have so called behaviors
attached to them (see below).
`<input>`’s elements for example are plain DOM elements but each input type has its own behavior assigned to the element.
The behavior, among other things, is responsible for providing and setting value of the element.
For example value of `input type=checkbox>` is boolean – _true_ or _false_,
and value of `<form>` element is a collection (name/value map) of all inputs on the form.

In native code values are represented by `sciter::Value` objects.
`sciter::Value` is a structure that can hold different types of values: numbers, strings, arrays, objects, etc
(see [documentation](http://sciter.com/docs/content/script/language/Types.htm)).

Here is how to set numeric value of DOM element in native code:

```
if let Some(num) = root.find_first("input[type=number]") {
	num.set_value( sciter::Value::from(12) );
}
```

In script the same will look like:

```ignore
if (var num = self.select("input[type=number]")) {
	num.value = 12;
}
```

.
*/

use ::{_API};
use scdom::*;
use sctypes::*;
use value::Value;

pub type Result<T> = ::std::result::Result<T, SCDOM_RESULT>;

/// Initialize HELEMENT by nullptr.
macro_rules! HELEMENT {
	() => { ::std::ptr::null_mut() }
}


macro_rules! ok_or {
	($rv:expr, $ok:ident) => {
		if $ok == SCDOM_RESULT::OK {
			Ok($rv)
		} else {
			Err($ok)
		}
	};

	// for DOM access not_handled is ok
	// for calling function operation_failed is also ok
	($rv:expr, $ok:ident, $skip_not_handled:expr) => {
		if $ok == SCDOM_RESULT::OK || ($ok == $skip_not_handled) {
			Ok($rv)
		} else {
			Err($ok)
		}
	};
}


trait ElementVisitor {
	fn on_element(&mut self, el: Element) -> bool;
	fn result(&self) -> Vec<Element>;
}

#[derive(Default)]
struct FindFirstElement {
	all: Vec<Element>,
}

impl ElementVisitor for FindFirstElement {
	fn on_element(&mut self, el: Element) -> bool {
		self.all.push(el);
		return true;	// stop enumeration
	}
	fn result(&self) -> Vec<Element> {
		self.all.clone()
	}
}

#[derive(Default)]
struct FindAllElements {
	all: Vec<Element>,
}

impl ElementVisitor for FindAllElements {
	fn on_element(&mut self, el: Element) -> bool {
		self.all.push(el);
		return false;	// continue enumeration
	}
	fn result(&self) -> Vec<Element> {
		self.all.clone()
	}
}


/// DOM element wrapper.
#[derive(PartialEq)]
pub struct Element {
	he: HELEMENT,
}

impl Element {

	//\name Creation

	/// Construct Element object from HELEMENT handle.
	pub fn from(he: HELEMENT) -> Element {
		Element { he: Element::use_or(he) }
	}

	/// Create new element, it is disconnected initially from the DOM.
	pub fn create(tag: &str) -> Element {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let text = 0 as LPCWSTR;
		(_API.SciterCreateElement)(tag.as_ptr(), text, &mut e.he);
		return e;
	}

	/// Create new element as child of `parent`.
	pub fn create_at(tag: &str, parent: &mut Element) -> Result<Element> {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let text = 0 as LPCWSTR;
		(_API.SciterCreateElement)(tag.as_ptr(), text, &mut e.he);
		let ok = parent.append(&e);
		ok.map(|_| e)
	}

	/// Create new element with specified `text`, it is disconnected initially from the DOM.
	pub fn with_text(tag: &str, text: &str) -> Element {
		let mut e = Element { he: HELEMENT!() };
		let (tag,_) = s2u!(tag);
		let (text,_) = s2w!(text);
		(_API.SciterCreateElement)(tag.as_ptr(), text.as_ptr(), &mut e.he);
		return e;
	}

	/// Get root DOM element of the Sciter document.
	pub fn from_window(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetRootElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get focus DOM element of the Sciter document.
	pub fn from_focus(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetFocusElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get highlighted element.
	pub fn from_highlighted(hwnd: HWINDOW) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetHighlightedElement)(hwnd, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Find DOM element of the Sciter document by coordinates.
	pub fn from_point(hwnd: HWINDOW, pt: POINT) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterFindElement)(hwnd, pt, &mut p);
		ok_or!(Element::from(p), ok)
	}

	/// Get element handle by its UID.
	pub fn from_uid(hwnd: HWINDOW, uid: u32) -> Result<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetElementByUID)(hwnd, uid, &mut p);
		ok_or!(Element::from(p), ok)
	}

	fn use_or(he: HELEMENT) -> HELEMENT {
		let ok = (_API.Sciter_UseElement)(he);
		if ok == SCDOM_RESULT::OK {
			he
		} else {
			HELEMENT!()
		}
	}


	//\name Common methods

	/// Access element pointer.
	pub fn as_ptr(&self) -> HELEMENT {
		self.he
	}

	/// Get element UID - identifier suitable for storage.
	pub fn get_uid(&self) -> u32 {
		let mut n = 0;
		(_API.SciterGetElementUID)(self.he, &mut n);
		return n;
	}

	/// Return element tag as string (e.g. 'div', 'body').
	pub fn get_tag(&self) -> String {
		let mut s = String::new();
		(_API.SciterGetElementTypeCB)(self.he, store_astr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get inner text of the element as string.
	pub fn get_text(&self) -> String {
		let mut s = String::new();
		(_API.SciterGetElementTextCB)(self.he, store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Set inner text of the element.
	pub fn set_text(&mut self, text: &str) {
		let (s,n) = s2w!(text);
		(_API.SciterSetElementText)(self.he, s.as_ptr(), n);
	}

	/// Get html representation of the element as utf-8 bytes.
	pub fn get_html(&self, with_outer_html: bool) -> Vec<u8> {
		let mut s = Vec::new();
		(_API.SciterGetElementHtmlCB)(self.he, with_outer_html as BOOL, store_bstr, &mut s as *mut Vec<u8> as LPVOID);
		return s;
	}

	/// Set inner or outer html of the element.
	pub fn set_html(&mut self, html: &[u8], how: Option<SET_ELEMENT_HTML>) {
		if html.len() == 0 {
			return self.clear();
		}
		(_API.SciterSetElementHtml)(self.he, html.as_ptr(), html.len() as UINT, how.unwrap_or(SET_ELEMENT_HTML::SIH_REPLACE_CONTENT) as UINT);
	}

	/// Get value of the element.
	pub fn get_value(&self) -> Value {
		let mut rv = Value::new();
		(_API.SciterGetValue)(self.he, rv.as_ptr());
		return rv;
	}

	/// Set value of the element.
	pub fn set_value(&mut self, val: Value) {
		(_API.SciterSetValue)(self.he, val.as_cptr());
	}

	/// Get HWINDOW of containing window.
	pub fn get_hwnd(&self, for_root: bool) -> HWINDOW {
		let mut hwnd: HWINDOW = ::std::ptr::null_mut();
		(_API.SciterGetElementHwnd)(self.he, &mut hwnd as *mut HWINDOW, for_root as BOOL);
		return hwnd;
	}

	// TODO: get_location
	// TODO: request_data, request_html
	// TODO: send_request
	// TODO: send_event, post_event, fire_event

	/// Evaluate script in element context.
	pub fn eval_script(&self, script: &str) -> Result<Value> {
		let mut rv = Value::new();
		let (s,n) = s2w!(script);
		let ok = (_API.SciterEvalElementScript)(self.he, s.as_ptr(), n, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}

	/// Call scripting function defined in the namespace of the element (a.k.a. global function).
	pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value> {
		let mut rv = Value::new();
		let (name,_) = s2u!(name);
		let argv = Value::pack_args(args);
		let ok = (_API.SciterCallScriptingFunction)(self.he, name.as_ptr(), argv.as_ptr(), argv.len() as UINT, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}

	/// Call scripting method defined for the element.
	pub fn call_method(&self, name: &str, args: &[Value]) -> Result<Value> {
		let mut rv = Value::new();
		let (name,_) = s2u!(name);
		let argv = Value::pack_args(args);
		let ok = (_API.SciterCallScriptingMethod)(self.he, name.as_ptr(), argv.as_ptr(), argv.len() as UINT, rv.as_ptr());
		return ok_or!(rv, ok, SCDOM_RESULT::OPERATION_FAILED);
	}


	//\name Attributes
	/// Get number of the attributes.
	pub fn attribute_count(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetAttributeCount)(self.he, &mut n);
		return n as usize;
	}

	/// Get attribute name by its index.
	pub fn attribute_name(&self, index: usize) -> String {
		let mut s = String::new();
		(_API.SciterGetNthAttributeNameCB)(self.he, index as UINT, store_astr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get attribute value by its index.
	pub fn attribute(&self, index: usize) -> String {
		let mut s = String::new();
		(_API.SciterGetNthAttributeValueCB)(self.he, index as UINT, store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Get attribute value by its name.
	pub fn get_attribute(&self, name: &str) -> Option<String> {
		let mut s = String::new();
		let (name,_) = s2u!(name);
		let ok = (_API.SciterGetAttributeByNameCB)(self.he, name.as_ptr(), store_wstr, &mut s as *mut String as LPVOID);
		match ok {
			SCDOM_RESULT::OK => Some(s),
			SCDOM_RESULT::OK_NOT_HANDLED => None,
			_ => None,
		}
	}

	/// Add or replace attribute.
	pub fn set_attribute(&mut self, name: &str, value: &str) {
		let (name,_) = s2u!(name);
		let (value,_) = s2w!(value);
		(_API.SciterSetAttributeByName)(self.he, name.as_ptr(), value.as_ptr());
	}

	/// Remove attribute.
	pub fn remove_attribute(&mut self, name: &str) {
		let (name,_) = s2u!(name);
		let value = ::std::ptr::null();
		(_API.SciterSetAttributeByName)(self.he, name.as_ptr(), value);
	}

	/// Toggle attribute.
	pub fn toggle_attribute(&mut self, name: &str, isset: bool, value: Option<&str>) {
		if isset {
			self.set_attribute(name, value.unwrap());
		} else {
			self.remove_attribute(name);
		}
	}

	/// Remove all attributes from the element.
	pub fn clear_attributes(&mut self) {
		(_API.SciterClearAttributes)(self.he);
	}


	//\name Style Attributes

	/// Get [style attribute](http://sciter.com/docs/content/sciter/Style.htm) of the element by its name.
	pub fn get_style_attribute(&self, name: &str) -> String {
		let mut s = String::new();
		let (name,_) = s2u!(name);
		(_API.SciterGetStyleAttributeCB)(self.he, name.as_ptr(), store_wstr, &mut s as *mut String as LPVOID);
		return s;
	}

	/// Set [style attribute](http://sciter.com/docs/content/sciter/Style.htm).
	pub fn set_style_attribute(&mut self, name: &str, value: &str) {
		let (name,_) = s2u!(name);
		let (value,_) = s2w!(value);
		(_API.SciterSetStyleAttribute)(self.he, name.as_ptr(), value.as_ptr());
	}

	//\name State methods


	//\name DOM tree access

	/// Get index of this element in its parent collection.
	pub fn index(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetElementIndex)(self.he, &mut n as *mut UINT);
		return n as usize;
	}

	/// Get root of the element.
	pub fn root(&self) -> Element {
		if let Some(dad) = self.parent() {
			dad.root()
		} else {
			self.clone()
		}
	}

	/// Get parent element.
	pub fn parent(&self) -> Option<Element> {
		let mut p = HELEMENT!();
		(_API.SciterGetParentElement)(self.he, &mut p);
		if p.is_null() {
			None
		} else {
			Some(Element::from(p))
		}
	}

	/// Get first sibling element.
	pub fn first_sibling(&self) -> Option<Element> {
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if count > 0 {
				return dad.child(0);
			}
		}
		None
	}

	/// Get last sibling element.
	pub fn last_sibling(&self) -> Option<Element> {
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if count > 0 {
				return dad.child(count - 1);
			}
		}
		None
	}

	/// Get next sibling element.
	pub fn next_sibling(&self) -> Option<Element> {
		let idx = self.index() + 1;
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if idx < count {
				return dad.child(idx);
			}
		}
		None
	}

	/// Get previous sibling element.
	pub fn prev_sibling(&self) -> Option<Element> {
		let idx = self.index();
		if let Some(dad) = self.parent() {
			let count = dad.len();
			if idx > 0 && (idx - 1) < count {
				return dad.child(idx - 1);
			}
		}
		None
	}

	/// Get first child element.
	pub fn first_child(&self) -> Option<Element> {
		return self.child(0);
	}

	/// Get last child element.
	pub fn last_child(&self) -> Option<Element> {
		let count = self.len();
		if count > 0 {
			return self.child(count - 1);
		}
		None
	}

	/// Get element child at specified index.
	pub fn get(&self, index: usize) -> Option<Element> {
		return self.child(index);
	}

	/// Get element child at specified index.
	pub fn child(&self, index: usize) -> Option<Element> {
		let mut p = HELEMENT!();
		let ok = (_API.SciterGetNthChild)(self.he, index as UINT, &mut p);
		match ok {
			SCDOM_RESULT::OK => Some(Element::from(p)),
			_ => None
		}
	}

	/// Get number of child elements.
	pub fn children_count(&self) -> usize {
		let mut n = 0u32;
		(_API.SciterGetChildrenCount)(self.he, &mut n as *mut UINT);
		return n as usize;
	}

	/// Get number of child elements.
	pub fn len(&self) -> usize {
		return self.children_count();
	}

	/// Clear content of the element.
	pub fn clear(&mut self) {
		(_API.SciterSetElementText)(self.he, ::std::ptr::null(), 0);
	}

	/// Create new element as copy of existing element.
	///
	/// The new element is a full (deep) copy of the element and is initially disconnected from the DOM.
	/// Note that `Element.clone()` does not clone DOM element, just increments its reference count.
	pub fn clone_element(&self) -> Element {
		let mut e = Element { he: HELEMENT!() };
		(_API.SciterCloneElement)(self.he, &mut e.he);
		return e;
	}

	/// Insert element at `index` position of this element.
	///
	/// Note that we cannot follow Rust semantic here
	/// because the newly created `Element` is unusable before it will be inserted at DOM.
	pub fn insert(&mut self, index: usize, child: &Element) -> Result<()> {
		let ok = (_API.SciterInsertElement)(child.he, self.he, index as UINT);
		ok_or!((), ok)
	}

	/// Append element as last child of this element.
	pub fn append(&mut self, child: &Element) -> Result<()> {
		self.insert(0x7FFFFFFF, child)
	}

	/// Append element as last child of this element.
	#[allow(unused_must_use)]
	pub fn push(&mut self, element: Element) {
		self.append(&element);
	}

	/// Remove the last child from this element and returns it, or `None` if this element is empty.
	#[allow(unused_must_use)]
	pub fn pop(&mut self) -> Option<Element> {
		let count = self.len();
		if count > 0 {
			if let Some(mut child) = self.get(count - 1) {
				child.detach();
				return Some(child);
			}
		}
		return None;
	}

	/// Take element out of its container (and DOM tree).
	pub fn detach(&mut self) -> Result<()> {
		let ok = (_API.SciterDetachElement)(self.he);
		ok_or!((), ok)
	}

	/// Take element out of its container (and DOM tree) and force destruction of all behaviors.
	pub fn destroy(&mut self) -> Result<()> {
		let mut p = HELEMENT!();
		::std::mem::swap(&mut self.he, &mut p);
		let ok = (_API.SciterDeleteElement)(p);
		ok_or!((), ok)
	}

	/// Swap element positions.
	pub fn swap(&mut self, other: &mut Element) -> Result<()> {
		let ok = (_API.SciterSwapElements)(self.he, other.he);
		ok_or!((), ok)
	}

	//\name Selectors

	/// Test this element against CSS selector(s).
	pub fn test(&self, selector: &str) -> bool {
		let mut p = HELEMENT!();
		let (s,_) = s2u!(selector);
		(_API.SciterSelectParent)(self.he, s.as_ptr(), 1, &mut p);
		return !p.is_null();
	}

	/// Call specified function for every element in a DOM that meets specified CSS selectors.
	fn select_elements<T: ElementVisitor>(&self, selector: &str, callback: T) -> Result<Vec<Element>> {
		extern "stdcall" fn inner<T: ElementVisitor>(he: HELEMENT, param: LPVOID) -> BOOL {
			let handler = ::schandler::NativeHandler::from_mut_ptr3(param);
			let mut obj = handler.as_mut::<T>();
			let e = Element::from(he);
			let stop = obj.on_element(e);
			return stop as BOOL;
		}
		let (s,_) = s2u!(selector);
		let handler = ::schandler::NativeHandler::from(callback);
		let ok = (_API.SciterSelectElements)(self.he, s.as_ptr(), inner::<T>, handler.as_mut_ptr());
		if ok != SCDOM_RESULT::OK {
			return Err(ok);
		}
		let obj = handler.as_ref::<T>();
		return Ok(obj.result());
	}

	/// Will find first parent element starting from this satisfying given css selector(s).
	pub fn find_nearest_parent(&self, selector: &str) -> Option<Element> {
		let mut p = HELEMENT!();
		let (s,_) = s2u!(selector);
		(_API.SciterSelectParent)(self.he, s.as_ptr(), 0, &mut p);
		if p.is_null() { None } else { Some(Element::from(p)) }
	}

	/// Will find first element starting from this satisfying given css selector(s).
	pub fn find_first(&self, selector: &str) -> Option<Element> {
		let cb = FindFirstElement::default();
		let all = self.select_elements(selector, cb);
		if let Ok(mut all) = all {
			all.pop()
		} else {
			None
		}
	}

	/// Will find all elements starting from this satisfying given css selector(s).
	pub fn find_all(&self, selector: &str) -> Option<Vec<Element>> {
		let cb = FindFirstElement::default();
		let all = self.select_elements(selector, cb);
		all.ok()
	}

	//\name Scroll methods:

	//\name Other methods:

	/// Apply changes and refresh element area in its window.
	pub fn update(&self, render_now: bool) -> Result<()> {
		let ok = (_API.SciterUpdateElement)(self.he, render_now as BOOL);
		ok_or!((), ok)
	}

	/// Start Timer for the element. Element will receive on_timer event.
	pub fn start_timer(&self, period_ms: u32, timer_id: usize) -> Result<()> {
		let ok = (_API.SciterSetTimer)(self.he, period_ms as UINT, timer_id as ::sctypes::UINT_PTR);
		ok_or!((), ok)
	}

	/// Stop Timer for the element.
	pub fn stop_timer(&self, timer_id: usize) -> Result<()> {
		if !self.he.is_null() {
			let ok = (_API.SciterSetTimer)(self.he, 0 as UINT, timer_id as ::sctypes::UINT_PTR);
			ok_or!((), ok)
		} else {
			Ok(())
		}
	}
}

/// Release element pointer.
impl Drop for Element {
	fn drop(&mut self) {
		(_API.Sciter_UnuseElement)(self.he);
		self.he = HELEMENT!();
	}
}

/// Increment reference count of the dom element.
impl Clone for Element {
	fn clone(&self) -> Self {
		Element::from(self.he)
	}
}

/// Human element representation.
impl ::std::fmt::Display for Element {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		if self.he.is_null() {
			return f.write_str("None");
		}
		// "tag#id.class|type(name)"
		// "tag#id.class"
		let (t,n,i,c) = (self.get_attribute("type"), self.get_attribute("name"), self.get_attribute("id"), self.get_attribute("class"));
		let tag = self.get_tag();
		try!(f.write_str(&tag));
		if i.is_some() {
			try!(write!(f, "#{}", i.unwrap()));
		}
		if c.is_some() {
			try!(write!(f, ".{}", c.unwrap()));
		}
		if t.is_some() {
			try!(write!(f, "|{}", t.unwrap()));
		}
		if n.is_some() {
			try!(write!(f, "({})", n.unwrap()));
		}
		return Ok(());
	}
}

/// Machine-like element visualization.
impl ::std::fmt::Debug for Element {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		// "tag#id.class(name):0xdfdfdf"
		write!(f, "{{{}:{:?}}}", self, self.he)
	}
}



use ::utf;

/// Convert an incoming UTF-16 to `String`.
extern "stdcall" fn store_wstr(szstr: LPCWSTR, str_length: UINT, param: LPVOID) {
	let s = utf::w2sn(szstr, str_length as usize);
	let out = param as *mut String;
	unsafe { *out = s };
}

/// Convert an incoming UTF-8 to `String`.
extern "stdcall" fn store_astr(szstr: LPCSTR,  str_length: UINT, param: LPVOID) {
	let s = utf::u2sn(szstr, str_length as usize);
	let out = param as *mut String;
	unsafe { *out = s };
}

/// Convert an incoming html string (UTF-8 in fact) to `String`.
extern "stdcall" fn store_bstr(szstr: LPCBYTE, str_length: UINT, param: LPVOID) {
	let s = unsafe { ::std::slice::from_raw_parts(szstr, str_length as usize) };
	let pout = param as *mut Vec<u8>;
	let out = unsafe {&mut *pout};
	out.extend_from_slice(s);
}

/* Not implemented yet or not used APIs:

SciterAttachEventHandler
SciterAttachHwndToElement

SciterCallBehaviorMethod
SciterCombineURL
SciterControlGetType
SciterDetachEventHandler
SciterFireEvent
SciterGetElementIntrinsicHeight
SciterGetElementIntrinsicWidths
SciterGetElementLocation
SciterGetElementNamespace
SciterGetElementState
SciterGetElementType
SciterGetExpando
SciterGetObject
SciterGetScrollInfo
SciterHidePopup
SciterHttpRequest
SciterIsElementEnabled
SciterIsElementVisible
SciterPostEvent
SciterRefreshElementArea
SciterReleaseCapture
SciterRequestElementData
SciterScrollToView
SciterSendEvent
SciterSetCapture
SciterSetElementState
SciterSetHighlightedElement
SciterSetScrollPos
SciterShowPopup
SciterShowPopupAt
SciterSortElements
SciterTraverseUIEvent

SciterCreateCommentNode
SciterCreateTextNode
SciterNodeAddRef
SciterNodeCastFromElement
SciterNodeCastToElement
SciterNodeChildrenCount
SciterNodeFirstChild
SciterNodeGetText
SciterNodeInsert
SciterNodeLastChild
SciterNodeNextSibling
SciterNodeNthChild
SciterNodeParent
SciterNodePrevSibling
SciterNodeRelease
SciterNodeRemove
SciterNodeSetText
SciterNodeType

*/
