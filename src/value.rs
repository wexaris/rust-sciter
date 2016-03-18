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

	pub fn new() -> Value {
		Value { data: VALUE { t: VALUE_TYPE::T_UNDEFINED, u: 0, d: 0 } }
	}

	pub fn null() -> Value {
		let mut me = Value::new();
		me.data.t = VALUE_TYPE::T_NULL;
		return me;
	}

	pub fn symbol(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::UT_STRING_SYMBOL);
		return me;
	}

	pub fn parse(val: &str) -> Result<Value, VALUE_RESULT> {
		let mut me = Value::new();
		let (s,n) = s2w!(val);
		let ok = (_API.ValueFromString)(me.ptr(), s.as_ptr(), n, VALUE_STRING_CVT_TYPE::CVT_JSON_LITERAL);
		if ok == VALUE_RESULT::HV_OK {
			Ok(me)
		} else {
			Err(ok)
		}
	}

	fn ptr(&mut self) -> *mut VALUE {
		&mut self.data as *mut VALUE
	}

	fn cptr(&self) -> *const VALUE {
		&self.data as *const VALUE
	}

	pub fn clear(&mut self) -> &mut Value {
		(_API.ValueClear)(self.ptr());
		self
	}

	pub fn length(&self) -> i32 {
		let mut n: i32 = 0;
		(_API.ValueElementsCount)(self.cptr(), &mut n);
		return n;
	}

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
	#[cfg(unix)]
	pub fn is_native_function(&self) -> bool {
		(_API.ValueIsNativeFunctor)(self.ptr) != 0
	}
	pub fn is_object(&self) -> bool {
		self.data.t == VALUE_TYPE::T_OBJECT
	}
	pub fn is_dom_element(&self) -> bool {
		self.data.t == VALUE_TYPE::T_DOM_OBJECT
	}

	fn assign_str(&mut self, val: &str, unit: VALUE_UNIT_TYPE_STRING) -> VALUE_RESULT {
		let (s,n) = s2w!(val);
		return (_API.ValueStringDataSet)(self.ptr(), s.as_ptr(), n, unit as UINT);
	}
}

impl Drop for Value {
	fn drop(&mut self) {
		(_API.ValueClear)(self.ptr());
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

		let p1 = v.ptr();
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
