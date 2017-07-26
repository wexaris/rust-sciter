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

fn is_color_supported() -> bool {
	// e.g. `0x04000100`
	sciter::version_num() > 0x04000100
}

#[test]
fn color_works() {
	if !is_color_supported() { return; }

	// yellow R255, G255, B000
	// RGBA form in memory, ABGR in integer.
	let v = Value::color(0x0000FFFF);
	assert!(v.is_color());
	assert_eq!(v.to_color(), Some(0x0000FFFF));
}

#[test]
fn duration_works() {
	if !is_color_supported() { return; }

	let v = Value::duration(12.5);
	assert!(v.is_duration());
	assert_eq!(v.to_duration(), Some(12.5));
}

#[test]
fn angle_works() {
	if !is_color_supported() { return; }

	let v = Value::angle(1.0);
	assert!(v.is_angle());
	assert_eq!(v.to_angle(), Some(1.0));
}

#[test]
fn array_works() {
	let v = Value::array(0);
	assert!(v.is_array());
	assert!(v.len() == 0);

	let v = Value::array(17);
	assert!(v.is_array());
	assert!(v.len() == 17);
}

#[test]
fn map_works() {
	let v = Value::map();
	assert!(v.is_map());
	assert!(v.len() == 0);
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
fn from_function_works() {
	// create from lambda
	let v = Value::from(|args: &[Value]| Value::from(args.len() as i32));
	assert!(v.is_native_function());

	let args = [Value::from(17), Value::from(42)];
	let r = v.call(None, &args, None);

	assert!(r.is_ok());
	assert_eq!(r.unwrap(), Value::from(args.len() as i32));

	// create from function
	fn inner_fn(args: &[Value]) -> Value {
		Value::array(args.len())
	}

	let v = Value::from(inner_fn);
	assert!(v.is_native_function());
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

	let v = Value::from(1);
	assert!(v.is_int());

	let v: Value = Value::from(1);
	assert!(v.is_int());

	let v: Value = 1.into();
	assert!(v.is_int());

	let mut v = Value::new();
	v.push(false);
	v.push(1);
	v.push(3.0);
	v.push("2");
	assert!(v.is_array());
	assert_eq!(v.len(), 4);

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
	assert_eq!(v[0], 1.into());

	v.set(1, 17);
	assert_eq!(v[1], 17.into());

	let mut v: Value = r##"{"5": 5, "6": 6, seven: "seven"}"##.parse().unwrap();
	let key = Value::from("seven");
	v.set_item(key.clone(), Value::from(7.0));
	println!("map {:?}", v);
	assert_eq!(v.get_item(key), Value::from(7.0));

	// simple syntax:
	let mut v = Value::map();
	v.set_item("seven", 7);
	assert_eq!(v["seven"], 7.into());
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
