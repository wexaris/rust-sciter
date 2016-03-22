//! Rust interface to sciter::value.

use ::{_API};
use scvalue::*;
use sctypes::*;


/// sciter::value wrapper.
pub struct Value
{
	data: VALUE,
	tmp: * mut Value,
}

impl Value {

	/// Return a new sciter value object (undefined).
	pub fn new() -> Value {
		Value { data: VALUE::default(), tmp: ::std::ptr::null_mut() }
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
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::SYMBOL);
		return me;
	}

	/// Make sciter error value.
	pub fn error(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::ERROR);
		return me;
	}

	/// Parse json string into value.
	pub fn parse(val: &str) -> Result<Value, u32> {
		return Value::parse_as(val, VALUE_STRING_CVT_TYPE::JSON_LITERAL);
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
		Value::parse(val).or(Err(VALUE_RESULT::BAD_PARAMETER))
	}

	pub fn as_ptr(&mut self) -> *mut VALUE {
		&mut self.data as *mut VALUE
	}

	pub fn as_cptr(&self) -> *const VALUE {
		&self.data as *const VALUE
	}

	pub fn as_mut_ptr(&self) -> * mut VALUE {
		unsafe { ::std::mem::transmute(self.as_cptr()) }
	}

	/// Get inner value type.
	pub fn get_type(&self) -> VALUE_TYPE {
		return self.data.t;
	}

	pub fn full_type(&self) -> (VALUE_TYPE, UINT) {
		return (self.data.t, self.data.u);
	}

	/// Convert T_OBJECT value type to JSON T_MAP or T_ARRAY.
	pub fn isolate(&mut self) {
		(_API.ValueIsolate)(self.as_ptr());
	}

	/// Clear the VALUE and deallocates all assosiated structures that are not used anywhere else.
	pub fn clear(&mut self) -> &mut Value {
		(_API.ValueClear)(self.as_ptr());
		self
	}

	/// Return the number of items in the T_ARRAY, T_MAP, T_FUNCTION and T_OBJECT sciter::value.
	pub fn len(&self) -> usize {
		let mut n: INT = 0;
		(_API.ValueElementsCount)(self.as_cptr(), &mut n);
		return n as usize;
	}

	/// Append value to the end of T_ARRAY sciter::value.
	pub fn push(&mut self, src: Value) {
		(_API.ValueNthElementValueSet)(self.as_ptr(), self.len() as INT, src.as_cptr());
	}

	/// Insert or set value at given `index` of T_ARRAY, T_MAP, T_FUNCTION and T_OBJECT sciter::value.
	pub fn insert(&mut self, index: usize, src: Value) {
		(_API.ValueNthElementValueSet)(self.as_ptr(), index as INT, src.as_cptr());
	}

	/// Value to integer.
	pub fn to_int(&self) -> Option<i32> {
		let mut val = 0 as i32;
		match (_API.ValueIntData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val),
			_ => None
		}
	}

	/// Value to bool.
	pub fn to_bool(&self) -> Option<bool> {
		let mut val = 0 as i32;
		match (_API.ValueIntData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val != 0),
			_ => None
		}
	}

	/// Value to float.
	pub fn to_float(&self) -> Option<f64> {
		let mut val = 0 as f64;
		match (_API.ValueFloatData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val),
			_ => None
		}
	}

	/// Value as string for T_STRING type.
	pub fn as_string(&self) -> Option<String> {
		let mut s = 0 as LPCWSTR;
		let mut n = 0 as UINT;
		match (_API.ValueStringData)(self.as_cptr(), &mut s, &mut n) {
			VALUE_RESULT::OK => Some(::utf::w2sn(s, n as usize)),
			_ => None
		}
	}

	/// Value to json string (converted in-place).
	pub fn into_string(&mut self) -> String {
		(_API.ValueToString)(self.as_ptr(), VALUE_STRING_CVT_TYPE::JSON_LITERAL);
		return self.as_string().unwrap();
	}

	/// Value as byte slice for T_BYTES type.
	pub fn as_bytes(&self) -> Option<&[u8]> {
		let mut s = 0 as LPCBYTE;
		let mut n = 0 as UINT;
		match (_API.ValueBinaryData)(self.as_cptr(), &mut s, &mut n) {
			VALUE_RESULT::OK => Some(unsafe { ::std::slice::from_raw_parts(s, n as usize) }),
			_ => None
		}
	}

	/// Value to byte vector for T_BYTES type.
	pub fn to_bytes(&self) -> Option<Vec<u8>> {
		match self.as_bytes() {
			Some(r) => Some(r.to_owned()),
			None => None,
		}
	}

	/// Function invokation for T_OBJECT/UT_OBJECT_FUNCTION.
	pub fn call(&self, this: Option<Value>, args: &[Value], name: Option<&str>) -> Result<Value, VALUE_RESULT> {
		let mut rv = Value::new();
		let argv = Value::pack_args(args);
		let (name,_) = s2w!(name.unwrap_or(""));
		let ok = (_API.ValueInvoke)(self.as_cptr(), this.unwrap_or(Value::default()).as_ptr(),
			argv.len() as UINT, argv.as_ptr(), rv.as_ptr(), name.as_ptr());
		match ok {
			VALUE_RESULT::OK => Ok(rv),
			_ => Err(ok)
		}
	}

	pub fn pack_to(&self, dst: &mut VALUE) {
		(_API.ValueCopy)(dst, self.as_cptr());
	}

	pub fn pack_args(args: &[Value]) -> Vec<VALUE> {
		let argc = args.len();
		let mut argv: Vec<VALUE> = Vec::with_capacity(argc);
		argv.resize(argc, VALUE::default());
		for i in 0..argc {
			args[i].pack_to(&mut argv[i]);
		}
		return argv;
	}

	pub fn unpack_from(args: * const VALUE, count: UINT) -> Vec<Value> {
		let argc = count as usize;
		let args = unsafe { ::std::slice::from_raw_parts(args, argc) };
		let mut argv: Vec<Value> = Vec::with_capacity(argc);
		for i in 0..argc {
			let mut v = Value::new();
			(_API.ValueCopy)(v.as_ptr(), &args[i]);
			argv.push(v);
		}
		return argv;
	}

	fn ensure_tmp_mut(&self) -> &mut Value {
		let cp = self as *const Value;
		let mp = cp as *mut Value;
		let me = unsafe { &mut *mp };
		return me.ensure_tmp();
	}

	fn ensure_tmp(&mut self) -> &mut Value {
		if self.tmp.is_null() {
			let tmp = Box::new(Value::new());
			self.tmp = Box::into_raw(tmp);
		}
		return unsafe { &mut *self.tmp };
	}

	// TODO: get_item, set_item
	// TODO: keys, values, items
	// TODO: call

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
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::SYMBOL as UINT
	}
	pub fn is_error_string(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::ERROR as UINT
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

impl ::std::fmt::Display for Value {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		let mut copy = self.clone();
		let re = copy.into_string();
		f.write_str(&re)
	}
}


impl ::std::fmt::Debug for Value {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		let mut tname = format!("{:?}", self.data.t);
		if self.is_undefined() || self.is_null() {
			return f.write_str(&tname[2..].to_lowercase());
		}
		if self.is_string() && self.data.u != 0 {
			let units = ["symbol", "string", "error", "secure"];
			tname.push_str(":");
			tname.push_str(units[(self.data.u as i16 + 1) as usize]);
		}
		try!(f.write_str(&tname[2..].to_lowercase()));
		try!(f.write_str(":"));
		write!(f, "{}", &self)
	}
}

impl Drop for Value {
	/// Destroy pointed value.
	fn drop(&mut self) {
		if !self.tmp.is_null() {
			unsafe { Box::from_raw(self.tmp) };
		}
		(_API.ValueClear)(self.as_ptr());
	}
}

impl Default for Value {
	fn default() -> Self {
		return Value::new();
	}
}

impl Clone for Value {
	fn clone(&self) -> Self {
		let mut dst = Value::new();
		(_API.ValueCopy)(dst.as_ptr(), self.as_cptr());
		return dst;
	}
}

impl ::std::cmp::PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (_API.ValueCompare)(self.as_cptr(), other.as_cptr()) {
			VALUE_RESULT::OK_TRUE => true,
			VALUE_RESULT::OK => false,
			_ => false
		}
	}
}

/// Get item by index for array type.
impl ::std::ops::Index<usize> for Value {
	type Output = Value;
	fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueNthElementValue)(self.as_cptr(), index as INT, tmp.as_mut_ptr());
		return tmp;
	}
}

/// Get item by index for array type.
impl ::std::ops::IndexMut<usize> for Value {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Value {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueNthElementValue)(self.as_cptr(), index as INT, tmp.as_ptr());
		return tmp;
	}
}

/// Get item by key for map type.
impl ::std::ops::Index<Value> for Value {
	type Output = Value;
	fn index<'a>(&'a self, index: Value) -> &'a Self::Output {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueGetValueOfKey)(self.as_cptr(), index.as_cptr(), tmp.as_mut_ptr());
		return tmp;
	}
}

/// Get item by key for map type.
impl ::std::ops::IndexMut<Value> for Value {
	fn index_mut<'a>(&'a mut self, index: Value) -> &'a mut Value {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueGetValueOfKey)(self.as_cptr(), index.as_cptr(), tmp.as_mut_ptr());
		return tmp;
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
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::STRING);
		return me;
	}
}

/// Value from string.
impl From<String> for Value {
	fn from(val: String) -> Self {
		let mut me = Value::new();
		me.assign_str(&val, VALUE_UNIT_TYPE_STRING::STRING);
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

/// Value from sequence of i32.
impl ::std::iter::FromIterator<i32> for Value {
	fn from_iter<I: IntoIterator<Item=i32>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of f64.
impl ::std::iter::FromIterator<f64> for Value {
	fn from_iter<I: IntoIterator<Item=f64>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of &str.
impl<'a> ::std::iter::FromIterator<&'a str> for Value {
	fn from_iter<I: IntoIterator<Item=&'a str>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of String.
impl ::std::iter::FromIterator<String> for Value {
	fn from_iter<I: IntoIterator<Item=String>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
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
		// println!("value {:?}", v);

		// let p1 = v.as_ptr();
		// let p2 = &mut v.data as *mut VALUE;
		// println!("p1 {:?} p2 {:?} ", p1, p2);
		// assert!(p1 == p2);
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
