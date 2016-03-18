extern crate sciter;

use sciter::value::*;

#[allow(unused_variables)]


#[test]
fn test_value_new() {
	let v = Value::new();
	assert!(v.is_undefined());
	assert!(!v.is_null());
}


#[test]
fn test_value_null() {
 	let v = Value::null();
	assert!(!v.is_undefined());
	assert!(v.is_null());
}

#[test]
fn test_value_clear() {
	let mut v = Value::null();
	assert!(v.is_null());

	v.clear();
	assert!(!v.is_null());
	assert!(v.is_undefined());
}

#[test]
fn test_value_symbol() {
	let mut v = Value::symbol("hello");
	assert!(v.is_symbol());
	assert!(v.is_string());

	v.clear();
	assert!(v.is_undefined());
}

