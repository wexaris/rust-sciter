//! TIScript Virtual Machine Runtime.
use _TAPI;
use capi::scdef::SCRIPT_RUNTIME_FEATURES;

pub use capi::sctiscript::{HVM, tiscript_value};

pub struct Value(pub tiscript_value);

pub struct Vm(HVM);

impl Drop for Vm {
  fn drop(&mut self) {
    destroy_vm(self.0);
  }
}

impl From<HVM> for Vm {
  fn from(vm: HVM) -> Self {
    assert!(!vm.is_null());
    Vm(vm)
  }
}

impl Vm {
  pub fn new() -> Vm {
    Vm(create_vm(None, None, None).unwrap())
  }

  pub fn with_features(features: SCRIPT_RUNTIME_FEATURES) -> Vm {
    Vm(create_vm(Some(features as u32), None, None).unwrap())
  }

  pub fn current() -> Option<Vm> {
    get_current_vm().map(|h| Vm(h))
  }
}


/// Create a new tiscript_VM (and make it current for the thread).
pub fn create_vm(features: Option<u32>, heap_size: Option<u32>, stack_size: Option<u32>) -> Option<HVM> {
  let vm = (_TAPI.create_vm)(features.unwrap_or(0xFFFF_FFFF), heap_size.unwrap_or(1*1024*1024), stack_size.unwrap_or(64*1024));
  if !vm.is_null() {
    Some(vm)
  } else {
    None
  }
}

/// Destroy tiscript VM.
pub fn destroy_vm(vm: HVM) {
  (_TAPI.destroy_vm)(vm);
}

/// Get tiscript VM attached to the current thread.
pub fn get_current_vm() -> Option<HVM> {
  let vm = (_TAPI.get_current_vm)();
  if !vm.is_null() {
    Some(vm)
  } else {
    None
  }
}

impl Value {
  pub fn new() -> Self {
    Value((_TAPI.undefined_value)())
  }

  pub fn nothing() -> Self {
    Value((_TAPI.nothing_value)())
  }

  pub fn null() -> Self {
    Value((_TAPI.null_value)())
  }

  pub fn int(v: i32) -> Self {
    Value((_TAPI.int_value)(v))
  }

  pub fn is_int(&self) -> bool {
    (_TAPI.is_int)(self.0)
  }

  pub fn to_int(&self) -> Option<i32> {
    let mut v = 0;
    let ok = (_TAPI.get_int_value)(self.0, &mut v);
    if ok {
      Some(v)
    } else {
      None
    }
  }


}
