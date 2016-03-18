#![allow(unused_variables)]

extern crate sciter;

use sciter::value::*;


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

#[test]
fn test_value_from_bool() {
	let v = Value::from(true);
	assert!(v.is_bool());
	let v = Value::from(false);
	assert!(v.is_bool());
}

#[test]
fn test_value_from_int() {
	let v = Value::from(1);
	assert!(v.is_int());
	assert!(!v.is_bool());

	Value::from(1 as i32);
	// Value::from(1 as u32);
}

#[test]
fn test_value_from_float() {
	let v = Value::from(1.0);
	assert!(v.is_float());
}


#[test]
fn test_value_from_str() {
	let v = Value::from("hello");
	assert!(v.is_string());

	let s = String::from("hello");
	let v = Value::from(s.as_str());

	let v = Value::from_str("hello");
	let v = Value::from_str(&s);
}

#[test]
fn test_value_parse() {
	let items = ["", "null", "1", "\"2\"", "2.0", "true", "[3, 4]", r##"{"5": 5, "6": 6, seven: "seven"}"##];
	for item in items.iter() {
		let r = Value::parse(item);
		match r {
			Err(num) => panic!("parse({}) failed on character {} of {}", item, num, item.len()),
			Ok(_) => {},
		}
	}

	let v :Value = "".parse().unwrap();
	let v = "".parse::<Value>();

}

#[test]
#[should_panic]
fn test_value_parse_fail() {
	let item = "{item: "; // invalid json
	let r = Value::parse(item);
	match r {
		Err(num) => panic!("parse({}) failed on character {} of {}", item, num, item.len()),
		Ok(_) => {},
	}
}
