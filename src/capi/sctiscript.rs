//! TIScript Virtual Machine Runtime.

#![allow(non_camel_case_types, non_snake_case)]
#![allow(dead_code)]

use capi::sctypes::{UINT64, LPCBYTE, LPCSTR, LPCWSTR, LPVOID};

MAKE_HANDLE!(
  #[doc = "TIScript VM native handle."]
  HVM,
  _HVM
);

pub type tiscript_value = UINT64;

#[repr(C)]
pub struct tiscript_stream {
  vtbl: *const tiscript_stream_vtbl,
}

#[repr(C)]
struct tiscript_stream_vtbl {
  input: extern "C" fn(tag: *mut tiscript_stream, pv: *mut i32) -> bool,
  output: extern "C" fn(tag: *mut tiscript_stream, v: i32) -> bool,
  name: extern "C" fn(tag: *mut tiscript_stream) -> LPCWSTR,
  close: extern "C" fn(tag: *mut tiscript_stream),
}

#[repr(C)]
pub struct tiscript_pvalue {
  val: tiscript_value,
  vm: HVM,
  d1: LPVOID,
  d2: LPVOID,
}

#[repr(C)]
pub struct tiscript_method_def {
  dispatch: LPVOID,
  name: LPCSTR,
  handler: extern "C" fn(HVM) -> tiscript_value,
  tag: LPVOID,
  payload: tiscript_value,
}

#[repr(C)]
pub struct tiscript_prop_def {
  dispatch: LPVOID,
  name: LPCSTR,
  getter: extern "C" fn(HVM, obj: tiscript_value) -> tiscript_value,
  setter: extern "C" fn(c: HVM, obj: tiscript_value, tiscript_value: tiscript_value),
  tag: LPVOID,
}

#[repr(C)]
pub union tiscript_const_val {
  i: i32,
  f: f64,
  s: LPCWSTR,
}

#[repr(C)]
pub enum tiscript_const_type {
  Int = 0,
  Float = 1,
  String = 2,
}

#[repr(C)]
pub struct tiscript_const_def {
  name: LPCSTR,
  val: tiscript_const_val,
  vtype: tiscript_const_type,
}

#[repr(C)]
pub struct tiscript_class_def {
  /// having this name
  pub name: LPCSTR,
  /// with these methods
  pub methods: *const tiscript_method_def,
  /// with these properties
  pub props: *const tiscript_prop_def,
  /// with these constants (if any)
  pub consts: *const tiscript_const_def,
  /// `var v = obj[idx]`
  pub get_item: extern "C" fn(c: HVM, obj: tiscript_value, key: tiscript_value) -> tiscript_value,
  /// `obj[idx] = v`
  pub set_item: extern "C" fn(c: HVM, obj: tiscript_value, key: tiscript_value, tiscript_value: tiscript_value),
  /// destructor of native objects
  pub finalizer: extern "C" fn(c: HVM, obj: tiscript_value),
  /// `for(var el in collecton)` handler
  pub iterator: extern "C" fn(c: HVM, index: &mut tiscript_value, obj: tiscript_value) -> tiscript_value,
  /// called by GC to notify that 'self' is moved to a new location
  pub on_gc_copy: extern "C" fn(instance_data: LPVOID, new_self: tiscript_value),
  /// superclass, prototype for the class (or `0`)
  pub prototype: tiscript_value,
}

#[repr(C)]
#[allow(missing_docs)]
pub struct tiscript_native_interface {
  pub create_vm: extern "C" fn(features: u32, heap_size: u32, stack_size: u32) -> HVM,
  // destroy tiscript_VM
  pub destroy_vm: extern "C" fn(pvm: HVM),
  // invoke GC
  pub invoke_gc: extern "C" fn(pvm: HVM),
  // set stdin, stdout and stderr for this tiscript_VM
  pub set_std_streams: extern "C" fn(pvm: HVM, input: *mut tiscript_stream, output: *mut tiscript_stream, error: *mut tiscript_stream),
  // get tiscript_VM attached to the current thread
  pub get_current_vm: extern "C" fn() -> HVM,
  // get global namespace (Object)
  pub get_global_ns: extern "C" fn(HVM) -> tiscript_value,
  // get current namespace (Object)
  pub get_current_ns: extern "C" fn(HVM) -> tiscript_value,

  pub is_int: extern "C" fn(v: tiscript_value) -> bool,
  pub is_float: extern "C" fn(v: tiscript_value) -> bool,
  pub is_symbol: extern "C" fn(v: tiscript_value) -> bool,
  pub is_string: extern "C" fn(v: tiscript_value) -> bool,
  pub is_array: extern "C" fn(v: tiscript_value) -> bool,
  pub is_object: extern "C" fn(v: tiscript_value) -> bool,
  pub is_native_object: extern "C" fn(v: tiscript_value) -> bool,
  pub is_function: extern "C" fn(v: tiscript_value) -> bool,
  pub is_native_function: extern "C" fn(v: tiscript_value) -> bool,
  pub is_instance_of: extern "C" fn(v: tiscript_value, cls: tiscript_value) -> bool,
  pub is_undefined: extern "C" fn(v: tiscript_value) -> bool,
  pub is_nothing: extern "C" fn(v: tiscript_value) -> bool,
  pub is_null: extern "C" fn(v: tiscript_value) -> bool,
  pub is_true: extern "C" fn(v: tiscript_value) -> bool,
  pub is_false: extern "C" fn(v: tiscript_value) -> bool,
  pub is_class: extern "C" fn(HVM, v: tiscript_value) -> bool,
  pub is_error: extern "C" fn(v: tiscript_value) -> bool,
  pub is_bytes: extern "C" fn(v: tiscript_value) -> bool,
  pub is_datetime: extern "C" fn(HVM, v: tiscript_value) -> bool,

  pub get_int_value: extern "C" fn(v: tiscript_value, pi: &mut i32) -> bool,
  pub get_float_value: extern "C" fn(v: tiscript_value, pd: &mut f64) -> bool,
  pub get_bool_value: extern "C" fn(v: tiscript_value, pb: &mut bool) -> bool,
  pub get_symbol_value: extern "C" fn(v: tiscript_value, psz: &mut LPCWSTR) -> bool,
  pub get_string_value: extern "C" fn(v: tiscript_value, pdata: &mut LPCWSTR, plength: &mut u32) -> bool,
  pub get_bytes: extern "C" fn(v: tiscript_value, pb: &mut LPCBYTE, pblen: &mut u32) -> bool,
  pub get_datetime: extern "C" fn(HVM, v: tiscript_value, dt: &mut u64) -> bool,
  // dt - 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC)
  // a.k.a. FILETIME in Windows
  pub nothing_value: extern "C" fn() -> tiscript_value, // special value that designates "does not exist" result -> tiscript_value,
  pub undefined_value: extern "C" fn() -> tiscript_value,
  pub null_value: extern "C" fn() -> tiscript_value,
  pub bool_value: extern "C" fn(v: bool) -> tiscript_value,
  pub int_value: extern "C" fn(v: i32) -> tiscript_value,
  pub float_value: extern "C" fn(v: f64) -> tiscript_value,
  pub string_value: extern "C" fn(HVM, text: LPCWSTR, text_length: u32) -> tiscript_value,
  pub symbol_value: extern "C" fn(zstr: LPCSTR) -> tiscript_value,
  pub bytes_value: extern "C" fn(HVM, data: LPCBYTE, data_length: u32) -> tiscript_value,
  pub datetime_value: extern "C" fn(HVM, dt: u64) -> tiscript_value,

  pub to_string: extern "C" fn(HVM, v: tiscript_value) -> tiscript_value,

  // define native class
  pub define_class: extern "C" fn(vm: HVM, cls: &tiscript_class_def, zns: tiscript_value) -> tiscript_value,

  // object
  pub create_object: extern "C" fn(HVM, of_class: tiscript_value) -> tiscript_value,
  pub set_prop: extern "C" fn(HVM, obj: tiscript_value, key: tiscript_value, tiscript_value: tiscript_value) -> bool,
  pub get_prop: extern "C" fn(HVM, obj: tiscript_value, key: tiscript_value) -> tiscript_value,
  pub for_each_prop: extern "C" fn(HVM, obj: tiscript_value, cb: tiscript_object_enum, tag: LPVOID) -> bool,
  pub get_instance_data: extern "C" fn(obj: tiscript_value) -> LPVOID,
  pub set_instance_data: extern "C" fn(obj: tiscript_value, data: LPVOID),

  // array
  pub create_array: extern "C" fn(HVM, of_size: u32) -> tiscript_value,
  pub set_elem: extern "C" fn(HVM, obj: tiscript_value, idx: u32, tiscript_value: tiscript_value) -> bool,
  pub get_elem: extern "C" fn(HVM, obj: tiscript_value, idx: u32) -> tiscript_value,
  pub set_array_size: extern "C" fn(HVM, obj: tiscript_value, of_size: u32) -> tiscript_value,
  pub get_array_size: extern "C" fn(HVM, obj: tiscript_value) -> u32,

  // eval
  pub eval:
    extern "C" fn(HVM, ns: tiscript_value, input: &mut tiscript_stream, template_mode: bool, pretval: Option<&mut tiscript_value>) -> bool,
  pub eval_string:
    extern "C" fn(HVM, ns: tiscript_value, script: LPCWSTR, script_length: u32, pretval: Option<&mut tiscript_value>) -> bool,
  // call function (method)
  pub call: extern "C" fn(
    HVM,
    obj: tiscript_value,
    function: tiscript_value,
    argv: *const tiscript_value,
    argn: u32,
    pretval: Option<&mut tiscript_value>,
  ) -> bool,

  // compiled bytecodes
  pub compile: extern "C" fn(pvm: HVM, input: &mut tiscript_stream, output_bytecodes: &mut tiscript_stream, template_mode: bool) -> bool,
  pub loadbc: extern "C" fn(pvm: HVM, input_bytecodes: &mut tiscript_stream) -> bool,

  // throw error
  pub throw_error: extern "C" fn(HVM, error: LPCWSTR),

  // arguments access
  pub get_arg_count: extern "C" fn(pvm: HVM) -> u32,
  pub get_arg_n: extern "C" fn(pvm: HVM, n: u32) -> tiscript_value,

  // path here is global "path" of the object, something like
  // "one"
  // "one.two", etc.
  pub get_value_by_path: extern "C" fn(pvm: HVM, v: &mut tiscript_value, path: LPCSTR) -> bool,

  // pins
  pub pin: extern "C" fn(HVM, pp: &mut tiscript_pvalue),
  pub unpin: extern "C" fn(pp: &mut tiscript_pvalue),

  // create native_function_value and native_property_value,
  // use this if you want to add native functions/properties in runtime to exisiting classes or namespaces (including global ns)
  pub native_function_value: extern "C" fn(pvm: HVM, p_method_def: &tiscript_method_def) -> tiscript_value,
  pub native_property_value: extern "C" fn(pvm: HVM, p_prop_def: &tiscript_prop_def) -> tiscript_value,

  // Schedule execution of the `pfunc(prm)` in the thread owning this VM.
  // Used when you need to call scripting methods from threads other than main (GUI) thread.
  // It is safe to call tiscript functions inside the `pfunc`.
  // returns `true` if scheduling of the call was accepted, `false` when failure (VM has no dispatcher attached).
  pub post: extern "C" fn(pvm: HVM, pfunc: tiscript_callback, prm: LPVOID) -> bool,

  pub set_remote_std_streams: extern "C" fn(pvm: HVM, input: &tiscript_pvalue, output: &tiscript_pvalue, error: &tiscript_pvalue) -> bool,

  // support of multi-return values from native fucntions, n here is a number 1..64
  // since 4.1.6 (`set_nth_retval` in previous versions here)
  pub make_val_list: extern "C" fn(pvm: HVM, valc: i32, va: *const tiscript_value) -> tiscript_value,

  // returns number of props in object, elements in array, or bytes in byte array.
  pub get_length: extern "C" fn(pvm: HVM, obj: tiscript_value) -> i32,
  // `for( var val in coll ) {...}`
  pub get_next: extern "C" fn(pvm: HVM, obj: &tiscript_value, pos: &mut tiscript_value, val: &mut tiscript_value) -> bool,
  // `for( var (key,val) in coll ) {...}`
  pub get_next_key_value:
    extern "C" fn(pvm: HVM, obj: &tiscript_value, pos: &mut tiscript_value, key: &mut tiscript_value, val: &mut tiscript_value) -> bool,

  // associate extra data pointer with the VM
  pub set_extra_data: extern "C" fn(pvm: HVM, data: LPVOID) -> bool,
  pub get_extra_data: extern "C" fn(pvm: HVM) -> LPVOID,
}

/// callback for `for_each_prop`.
type tiscript_object_enum = extern "C" fn(c: HVM, key: tiscript_value, tiscript_value: tiscript_value, tag: LPVOID) -> bool;

/// callback for `post`.
type tiscript_callback = extern "C" fn(HVM, prm: LPVOID);

/// The entry point of TIScript Extnension Library.
pub type TIScriptLibraryInitFunc = extern "system" fn(vm: HVM, tiscript_api: &tiscript_native_interface);
