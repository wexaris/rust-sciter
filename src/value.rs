//! Rust interface to sciter::value.

use ::{_API};
use scvalue::*;
use sctypes::*;


#[derive(Debug)]
pub struct Value
{
	data: VALUE,
}


impl Value {

	/// Return a new sciter value object (undefined).
	pub fn new() -> Value {
		Value { data: VALUE { t: VALUE_TYPE::T_UNDEFINED, u: 0, d: 0 } }
	}

	/// Make explicit json null value.
	pub fn null() -> Value {
		let mut me = Value::new();
		me.data.t = VALUE_TYPE::T_NULL;
		return me;
	}

	/// Make sciter symbol value.
	pub fn symbol(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::UT_STRING_SYMBOL);
		return me;
	}

	/// Make sciter error value.
	pub fn error(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::UT_STRING_ERROR);
		return me;
	}

	/// Parse json string into value.
	pub fn parse(val: &str) -> Result<Value, u32> {
		return Value::parse_as(val, VALUE_STRING_CVT_TYPE::CVT_JSON_LITERAL);
	}

	/// Parse json string into value.
	pub fn parse_as(val: &str, how: VALUE_STRING_CVT_TYPE) -> Result<Value, u32> {
		let mut me = Value::new();
		let (s,n) = s2w!(val);
		let ok: u32 = (_API.ValueFromString)(me.as_ptr(), s.as_ptr(), n, how);
		if ok == 0 {
			Ok(me)
		} else {
			Err(ok)
		}
	}

	/// Parse json string into value.
	/// Note that `Value::from_str()` parses a json string to value object and returns a `Result<Value>`
	/// unlike `Value::from()`, which returns just string object only.
	pub fn from_str(val: &str) -> Result<Self, VALUE_RESULT> {
		match Value::parse(val) {
			Ok(v) => Ok(v),
			Err(_) => Err(VALUE_RESULT::HV_BAD_PARAMETER),
		}
	}

	pub fn as_ptr(&mut self) -> *mut VALUE {
		&mut self.data as *mut VALUE
	}

	pub fn as_cptr(&self) -> *const VALUE {
		&self.data as *const VALUE
	}

	/// Clear the VALUE and deallocates all assosiated structures that are not used anywhere else.
	pub fn clear(&mut self) -> &mut Value {
		(_API.ValueClear)(self.as_ptr());
		self
	}

	/// Return the number of items in the T_ARRAY, T_MAP, T_FUNCTION and T_OBJECT sciter::value.
	pub fn length(&self) -> i32 {
		let mut n: i32 = 0;
		(_API.ValueElementsCount)(self.as_cptr(), &mut n);
		return n;
	}

	// TODO: isolate, copy?
	// TODO: append, insert
	// TODO: get_item, set_item
	// TODO: keys, values, items
	// TODO: call
	// TOOD: get type?
	// TODO: get / set_value

	pub fn is_undefined(&self) -> bool {
		self.data.t == VALUE_TYPE::T_UNDEFINED
	}
	pub fn is_null(&self) -> bool {
		self.data.t == VALUE_TYPE::T_NULL
	}
	pub fn is_bool(&self) -> bool {
		self.data.t == VALUE_TYPE::T_BOOL
	}
	pub fn is_int(&self) -> bool {
		self.data.t == VALUE_TYPE::T_INT
	}
	pub fn is_float(&self) -> bool {
		self.data.t == VALUE_TYPE::T_FLOAT
	}
	pub fn is_bytes(&self) -> bool {
		self.data.t == VALUE_TYPE::T_BYTES
	}
	pub fn is_string(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING
	}
	pub fn is_symbol(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::UT_STRING_SYMBOL as UINT
	}
	pub fn is_error_string(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::UT_STRING_ERROR as UINT
	}
	pub fn is_date(&self) -> bool {
		self.data.t == VALUE_TYPE::T_DATE
	}
	pub fn is_currency(&self) -> bool {
		self.data.t == VALUE_TYPE::T_CURRENCY
	}
	pub fn is_map(&self) -> bool {
		self.data.t == VALUE_TYPE::T_MAP
	}
	pub fn is_array(&self) -> bool {
		self.data.t == VALUE_TYPE::T_ARRAY
	}
	pub fn is_function(&self) -> bool {
		self.data.t == VALUE_TYPE::T_FUNCTION
	}
	pub fn is_native_function(&self) -> bool {
		(_API.ValueIsNativeFunctor)(self.as_cptr()) != 0
	}
	pub fn is_object(&self) -> bool {
		self.data.t == VALUE_TYPE::T_OBJECT
	}
	pub fn is_dom_element(&self) -> bool {
		self.data.t == VALUE_TYPE::T_DOM_OBJECT
	}

	fn assign_str(&mut self, val: &str, unit: VALUE_UNIT_TYPE_STRING) -> VALUE_RESULT {
		let (s,n) = s2w!(val);
		return (_API.ValueStringDataSet)(self.as_ptr(), s.as_ptr(), n, unit as UINT);
	}
}

impl Drop for Value {
	/// Destroy pointed value.
	fn drop(&mut self) {
		(_API.ValueClear)(self.as_ptr());
	}
}

/// Value from integer.
impl From<i32> for Value {
	// Note that there is no generic 64-bit integers at Sciter, only Date/Currency types.
	// There is a double (f64) for large numbers as workaround.
	fn from(val: i32) -> Self {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), val as i32, VALUE_TYPE::T_INT as UINT, 0);
		return me;
	}
}

/// Value from float.
impl From<f64> for Value {
	fn from(val: f64) -> Self {
		let mut me = Value::new();
		(_API.ValueFloatDataSet)(me.as_ptr(), val as f64, VALUE_TYPE::T_FLOAT as UINT, 0);
		return me;
	}
}

/// Value from bool.
impl From<bool> for Value {
	fn from(val: bool) -> Self {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), val as INT, VALUE_TYPE::T_BOOL as UINT, 0);
		return me;
	}
}

/// Value from string.
impl<'a> From<&'a str> for Value {
	fn from(val: &'a str) -> Self {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::UT_STRING_STRING);
		return me;
	}
}

/// Value from json string.
impl ::std::str::FromStr for Value {
	type Err = VALUE_RESULT;
	fn from_str(val: &str) -> Result<Self, Self::Err> {
		Value::from_str(val)
	}
}

/// Value from binary array (sequence of bytes).
impl<'a> From<&'a [u8]> for Value {
	fn from(val: &'a [u8]) -> Self {
		let mut me = Value::new();
		(_API.ValueBinaryDataSet)(me.as_ptr(), val.as_ptr(), val.len() as u32, VALUE_TYPE::T_BYTES as UINT, 0);
		return me;
	}
}



mod tests {
	#![allow(unused_imports, unused_variables, unused_mut)]
	
	use super::{Value};
	use ::scvalue::*;
	use std::mem;
	use ::{_API};

	#[test]
	fn test_value_layout() {
		assert_eq!(mem::size_of::<VALUE_TYPE>(), 4);
		assert_eq!(mem::size_of::<VALUE>(), 16);
	}

	#[test]
	fn test_abi() {

		let mut data = VALUE { t: VALUE_TYPE::T_UNDEFINED, u: 0, d: 0 };
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);

		(_API.ValueInit)(&mut data);
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);

		(_API.ValueClear)(&mut data);
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);

		let mut v = Value::new();
		println!("value {:?}", v);

		let p1 = v.as_ptr();
		let p2 = &mut v.data as *mut VALUE;
		println!("p1 {:?} p2 {:?} ", p1, p2);
		assert!(p1 == p2);
	}

	#[test]
	fn test_value_new() {

		let mut v = Value::new();
		println!("value {:?}", v);
		println!("value {:?}", v);

		assert_eq!(v.data.t, VALUE_TYPE::T_UNDEFINED);
		assert!(v.is_undefined());

		v.clear();
		assert_eq!(v.data.t, VALUE_TYPE::T_UNDEFINED);
	}

	#[test]
	fn test_value_clear() {
		let mut v = Value::null();

		println!("value {:?}", v);
		println!("value {:?}", v);

		assert_eq!(v.data.t, VALUE_TYPE::T_NULL);
		assert!(v.is_null());

		v.clear();
		v.clear().clear().clear();

		println!("clear {:?}", v);
		assert!(v.is_undefined());
		assert!(!v.is_null());
		assert_eq!(v.data.t, VALUE_TYPE::T_UNDEFINED);
	}

}
