#![allow(unused_variables)]

#[macro_use]
extern crate sciter;

use sciter::value::*;


#[test]
fn new_works() {
	let v = Value::new();
	assert!(v.is_undefined());
	assert!(!v.is_null());
}


#[test]
fn null_works() {
 	let v = Value::null();
	assert!(!v.is_undefined());
	assert!(v.is_null());
}

#[test]
fn clear_works() {
	let mut v = Value::null();
	assert!(v.is_null());

	v.clear();
	assert!(!v.is_null());
	assert!(v.is_undefined());
}

#[test]
fn symbol_works() {
	let mut v = Value::symbol("hello");
	assert!(v.is_symbol());
	assert!(v.is_string());

	v.clear();
	assert!(v.is_undefined());
}

#[test]
fn from_bool_works() {
	let v = Value::from(true);
	assert!(v.is_bool());
	let v = Value::from(false);
	assert!(v.is_bool());
}

#[test]
fn from_int_works() {
	let v = Value::from(1);
	assert!(v.is_int());
	assert!(!v.is_bool());

	Value::from(1 as i32);
	// Value::from(1 as u32);
}

#[test]
fn from_float_works() {
	let v = Value::from(1.0);
	assert!(v.is_float());
}


#[test]
fn from_str_works() {
	let v = Value::from("hello");
	assert!(v.is_string());

	let s = String::from("hello");
	let v = Value::from(s.as_str());

	let v = Value::from_str("hello");
	let v = Value::from_str(&s);
}

#[test]
fn from_int_seq_works() {
	let v: Value = [1,2,3].iter().cloned().collect();
	assert!(v.is_array());
	assert_eq!(v.len(), 3);
}

#[test]
fn from_str_seq_works() {
	// &str
	let v: Value = ["1","2","3"].iter().cloned().collect();
	assert!(v.is_array());
	assert_eq!(v.len(), 3);

	// String
	let v: Value = ["1","2","3"].iter().map(|x| x.to_string()).collect();
	assert!(v.is_array());
	assert_eq!(v.len(), 3);
	assert_eq!(v[2].as_string(), Some("3".to_string()));
}

#[test]
fn parse_works() {
	let items = ["", "null", "1", "\"2\"", "2.0", "true", "[3, 4]", r##"{"5": 5, "6": 6, seven: "seven"}"##];
	for item in items.iter() {
		let r = Value::parse(item);
		match r {
			Err(num) => panic!("parse({}) failed on character {} of {}", item, num, item.len()),
			Ok(_) => {},
		}
	}

	let v :Value = "4".parse().unwrap();
	assert_eq!(v.to_int(), Some(4));

	let v = "true".parse::<Value>().unwrap();
	assert_eq!(v.to_bool(), Some(true));
}

#[test]	// crashes with 1.7.0 i686-pc-windows-msvc
#[should_panic(expected="failed on character")]
fn parse_fail_works() {
	let item = "{item: "; // invalid json
	let r = Value::parse(item);
	match r {
		Err(num) => panic!("parse({}) failed on character {} of {}", item, num, item.len()),
		Ok(_) => {},
	}
}

#[test]
fn pack_args_works() {
	let args = pack_args!();
	assert_eq!(args.len(), 0);

	let args = pack_args!(777);
	assert_eq!(args.len(), 1);

	let args = pack_args!(1,2,3);
	assert_eq!(args.len(), 3);

	let args = pack_args!(1, "2", 3.0);
	assert_eq!(args.len(), 3);

	let args = pack_args!(1,2,3);
	let unpacked = Value::unpack_from(args.as_ptr(), args.len() as u32);
	assert_eq!(unpacked.len(), 3);
	assert_eq!(unpacked[0], Value::from(1));
}

#[test]
fn make_args_works() {
	let args = make_args!();
	assert_eq!(args.len(), 0);

	let args = make_args!(777);
	assert_eq!(args.len(), 1);

	let args = make_args!(1,2,3);
	assert_eq!(args.len(), 3);

	let args = make_args!(1, "2", 3.0);
	assert_eq!(args.len(), 3);
}

#[test]
fn append_works() {
	let mut v = Value::new();
	v.push(Value::from(1));
	v.push(Value::from("2"));
	v.push(Value::from(3.0));
	v.push(Value::from(false));

	assert!(v.is_array());
	assert_eq!(v.len(), 4);
}

#[test]
fn to_works() {
	// Value has some implicit conversions:
	// bool or int -> int
	// int or float or length -> float
	// function or string -> string

	let vint = Value::from(1);
	assert!(vint.is_int());
	assert!(vint.to_int().is_some());
	assert!(vint.to_float().is_some());
	assert_eq!(vint.to_int().unwrap(), 1);

	let vbool = Value::from(false);
	assert!(vbool.is_bool());
	assert!(!vbool.is_int());
	assert!(vbool.to_bool().is_some());
	assert!(vbool.to_int().is_some());
	assert_eq!(vbool.to_bool().unwrap(), false);

	assert_eq!(Value::from(3.14).to_float().unwrap(), 3.14);

	assert_eq!(Value::from("3.14").as_string().unwrap(), "3.14");
}

#[test]
fn into_works() {
	assert_eq!(Value::from(1).into_string(), "1");
	assert_eq!(Value::from("hello").into_string(), r#""hello""#);
}

#[test]
fn bytes_works() {
	let b = [1,2,3];
	let v = Value::from(&b[..]);
	assert!(v.is_bytes());
	assert_eq!(v.as_bytes().expect("must be bytes"), [1,2,3]);
}

#[test]
fn index_works() {
	let mut v = Value::new();
	v.push(Value::from(1));
	v.push(Value::from(2));
	v.push(Value::from(3));

	println!("v {:?}", v);
	assert_eq!(v.len(), 3);
	assert_eq!(v[0], Value::from(1));

	v.set(1, Value::from(17));
	println!("v {:?}", v);
	assert_eq!(v[1], Value::from(17));

	let mut v: Value = r##"{"5": 5, "6": 6, seven: "seven"}"##.parse().unwrap();
	let key = Value::from("seven");
	v.set_item(key.clone(), Value::from(7.0));
	println!("map {:?}", v);
	assert_eq!(v.get_item(key), Value::from(7.0));

}

#[test]
fn display_works() {
	println!("\nvalue strings: new {}, null {}, bool {}, int {}, float {}, symbol {}, str {}",
		Value::new(), Value::null(), Value::from(true), Value::from(123), Value::from(3.14),
		Value::symbol("symbol"), Value::from("hello"));

	// assert!(false);
}

#[test]
fn debug_works() {
	println!("\nvalue strings: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
		Value::new(), Value::null(), Value::from(true), Value::from(123), Value::from(3.14),
		Value::symbol("symbol"), Value::from("hello"));

	// assert!(false);
}
