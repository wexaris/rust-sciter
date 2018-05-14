#![windows_subsystem = "windows"]

#[macro_use]
extern crate sciter;

fn main() {
  let mut frame = sciter::WindowBuilder::main_window()
    .with_size((1152, 768))
    .create();
  frame.event_handler(CpuSource::default());
  frame.load_html(include_bytes!("cpuload.htm"), Some("example://cpuload.htm"));
  frame.run_app();
}

#[derive(Default)]
struct CpuSource {

}

impl sciter::EventHandler for CpuSource {

  dispatch_script_call! {
    fn get_cpu_cores();
    fn get_cpu_data();
  }
}

impl CpuSource {
  // script
  pub fn get_cpu_cores(&mut self) -> sciter::Value {
    match nt::get_cpu_cores() {
      Ok(v) => v,
      Err(e) => e,
    }
  }

  // script
  pub fn get_cpu_data(&mut self) -> sciter::Value {
    let speed = match nt::get_cpu_speed() {
      Ok(v) => v,
      Err(e) => return e,
    };

    sciter::Value::new()
  }

}

mod nt {
  #![allow(bad_style)]
  use ::std::mem;
  use ::std::ptr;
  use sciter::Value;

  pub fn get_cpu_cores() -> Result<Value> {
    unsafe {
      let mut cb = 0;
      let mut info: SYSTEM_BASIC_INFORMATION = mem::zeroed();
      let st = NtQuerySystemInformation(
        SYSTEM_INFORMATION_CLASS::SystemBasicInformation,
        &mut info as *mut _ as LPVOID,
        mem::size_of_val(&info) as u32,
        Some(&mut cb),
        );
      if st == STATUS::SUCCESS {
        Ok(Value::from(info.NumberOfProcessors as i32))
      } else {
        Err(Value::error("Can't get cpu cores count"))
      }
    }
  }

  pub fn get_cpu_speed() -> Result<Value> {
    unsafe {
      let mut info: PROCESSOR_POWER_INFORMATION = mem::zeroed();
      let st = CallNtPowerInformation(
        POWER_INFORMATION_LEVEL::ProcessorInformation,
        ptr::null(),
        0,
        &mut info as *mut _ as LPVOID,
        mem::size_of_val(&info) as u32,
      );
      if st != STATUS::SUCCESS {
        return Err(Value::error("Can't get cpu speed data."));
      }
      let mut data = Value::map();
      data.set_item("max", info.MaxMhz as i32);
      data.set_item("current", info.CurrentMhz as i32);
      data.set_item("limit", info.MhzLimit as i32);
      data.set_item("idle", info.CurrentIdleState as i32);
      data.set_item("maxidle", info.MaxIdleState as i32);
      Ok(data)
    }
  }

  use sciter::types::*;

  type NTSTATUS = STATUS;
  type ULONG = u32;
  type Result<T> = ::std::result::Result<T, Value>;


  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  enum STATUS {
    SUCCESS = 0,
    INFO_LENGTH_MISMATCH = 0xC000_0004,
  }

  impl ::std::cmp::PartialEq for STATUS {
    fn eq(&self, r: &Self) -> bool {
      *self as i32 == *r as i32
    }
  }


  #[repr(C)]
  enum SYSTEM_INFORMATION_CLASS {
    SystemBasicInformation,
    SystemProcessorInformation,
    SystemPerformanceInformation,
    SystemTimeOfDayInformation,
    SystemPathInformation,
    SystemProcessInformation,
    SystemCallCountInformation,
    SystemDeviceInformation,
    SystemProcessorPerformanceInformation,
  }

  #[repr(C)]
  enum POWER_INFORMATION_LEVEL {
    ProcessorInformation = 11,
  }

  #[repr(C)]
  #[derive(Debug)]
  struct PROCESSOR_POWER_INFORMATION {
    pub Number: ULONG,
    pub MaxMhz: ULONG,
    pub CurrentMhz: ULONG,
    pub MhzLimit: ULONG,
    pub MaxIdleState: ULONG,
    pub CurrentIdleState: ULONG,
  }

  #[repr(C)]
  #[derive(Debug)]
  struct SYSTEM_BASIC_INFORMATION {
    pub Reserved: u32,
    pub TimerResolution: u32,
    pub PageSize: u32,
    pub NumberOfPhysicalPages: u32,
    pub LowestPhysicalPageNumber: u32,
    pub HighestPhysicalPageNumber: u32,
    pub AllocationGranularity: u32,
    pub MinimumUserModeAddress: usize,
    pub MaximumUserModeAddress: usize,
    pub ActiveProcessorsAffinityMask: usize,
    pub NumberOfProcessors: i8,
  }

  #[repr(C)]
  #[derive(Debug)]
  struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub DpcTime: i64,
    pub InterruptTime: i64,
    pub InterruptCount: u32,
  }

  extern "system" {
    fn NtQuerySystemInformation(
      SystemInformationClass: SYSTEM_INFORMATION_CLASS,
      SystemInformation: LPVOID,
      SystemInformationLength: UINT,
      ReturnLength: Option<&mut UINT>,
    ) -> NTSTATUS;
  }

  extern "system" {
    fn CallNtPowerInformation(
      InformationLevel: POWER_INFORMATION_LEVEL,
      lpInputBuffer: LPCVOID,
      nInputBufferSize: ULONG,
      lpOutputBuffer: LPVOID,
      nOutputBufferSize: ULONG,
    ) -> NTSTATUS;
  }
}
