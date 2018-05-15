#![windows_subsystem = "windows"]

#[macro_use]
extern crate sciter;

fn main() {
  let mut frame = sciter::WindowBuilder::main_window().with_size((1152, 768)).create();
  frame.event_handler(CpuSource::default());
  frame.archive_handler(include_bytes!("cpuload.rc")).expect("Can't load archive");
  frame.load_file("this://app/cpuload.htm");
  frame.run_app();
}

#[derive(Default)]
struct CpuSource {
  prev: nt::CpuState,
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
    let layout = match nt::get_cpu_layout() {
      Ok(v) => v,
      Err(e) => return e,
    };
    let load = match nt::get_cpu_load(&mut self.prev) {
      Ok(v) => v,
      Err(e) => return e,
    };

    let result = vmap! {
      "clocks" => speed,
      "topology" => layout,
      "cpu" => load,
      "delta" => sciter::Value::new(),
    };
    result
  }
}

mod nt {
  #![allow(bad_style, dead_code)]
  use sciter::Value;
  use std::mem;
  use std::ptr;

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

  pub fn get_cpu_layout() -> Result<Value> {
    unsafe {
      let mut sockets = 0;
      let mut cores = 0;
      let mut threads = 0;

      let mut cb = 0;
      use self::LOGICAL_PROCESSOR_RELATIONSHIP::*;
      let _ok = GetLogicalProcessorInformationEx(RelationAll, ptr::null_mut(), &mut cb);
      if cb != 0 {
        let mut buf = vec![0u8; cb as usize];
        let ok = GetLogicalProcessorInformationEx(RelationAll, buf.as_mut_ptr() as *mut _, &mut cb);
        if ok != 0 {
          let mut info = buf.as_ptr();
          let end = info.offset(cb as isize);
          while info < end {
            let next;
            {
              let data: *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX = mem::transmute(info);
              let data = &*data;
              next = data.Size;

              match data.Relationship {
                RelationNumaNode => {
                  sockets += 1;
                }
                RelationProcessorPackage => {}
                RelationProcessorCore => {
                  cores += 1;
                  let core = data.Union.Processor;
                  if core.Flags == 0 {
                    threads += 1;
                  } else {
                    for i in 0..core.GroupCount {
                      let bits = core.GroupMask.get_unchecked(i as usize).Mask;
                      threads += bits.count_ones() as i32;
                    }
                  }
                }
                _ => {}
              };
            }
            info = info.offset(next as isize);
          }

          let result = vmap! {
            "sockets" => sockets,
            "cores" => cores,
            "threads" => threads,
          };
          return Ok(result);
        }
      }
      Err(Value::error("Can't get cpu layout"))
    }
  }

  pub fn get_cpu_speed() -> Result<Value> {
    unsafe {
      let threads = get_cpu_cores()?.to_int().unwrap() as usize;
      let mut buf: Vec<PROCESSOR_POWER_INFORMATION> = vec![mem::zeroed(); threads];
      let size = buf.len() * mem::size_of::<PROCESSOR_POWER_INFORMATION>();
      let st = CallNtPowerInformation(
        POWER_INFORMATION_LEVEL::ProcessorInformation,
        ptr::null(),
        0,
        buf.as_mut_ptr() as LPVOID,
        size as u32,
      );
      if st != STATUS::SUCCESS {
        return Err(Value::error("Can't get cpu speed data."));
      }
      let info = &buf[0];
      let result = vmap! {
        "max" => info.MaxMhz as i32,
        "current" => info.CurrentMhz as i32,
        "limit" => info.MhzLimit as i32,
        "idle" => info.CurrentIdleState as i32,
        "maxidle" => info.MaxIdleState as i32,
      };
      Ok(result)
    }
  }

  #[derive(Default)]
  pub struct CpuState(Vec<SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION>);

  pub fn get_cpu_load(state: &mut CpuState) -> Result<Value> {
    unsafe {
      type Info = SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
      let threads = get_cpu_cores()?.to_int().unwrap() as usize;
      let mut buf: Vec<Info> = vec![mem::zeroed(); threads];
      let size = buf.len() * mem::size_of::<Info>();
      let st = NtQuerySystemInformation(
        SYSTEM_INFORMATION_CLASS::SystemProcessorPerformanceInformation,
        buf.as_mut_ptr() as *mut _,
        size as u32,
        None,
      );
      if st == STATUS::SUCCESS {
        // (user, kernel)
        let cores: Vec<_> = buf
          .iter()
          .zip(state.0.iter())
          .map(|(info, prev)| {
            let idle = (info.IdleTime - prev.IdleTime) as f64;
            let kernel = (info.KernelTime - prev.KernelTime) as f64;
            let user = (info.UserTime - prev.UserTime) as f64;
            let total = idle + kernel + user;
            let kernel = kernel * 100. / total;
            let user = user * 100. / total;
            (user, kernel)
          })
          .collect();
        (*state).0 = buf;

        let flen = threads as f64;
        let sum = cores.iter().fold((0., 0.), |p, uk| (uk.0 + p.0, uk.1 + p.1));
        let average = (sum.0 / flen, sum.1 / flen);
        let vtotal = vmap! {
          "user" => average.0,
          "kernel" => average.1,
        };
        let vcores = cores.iter().map(|uk| {
          vmap! {
            "user" => uk.0,
            "kernel" => uk.1
          }
        });
        use std::iter::FromIterator;
        let vcores = Value::from_iter(vcores);
        let result = vmap! {
          "total" => vtotal,
          "cores" => vcores,
        };
        return Ok(result);
      }
    }
    Err(Value::error("Can't get cpu load"))
  }

  use sciter::types::*;

  type NTSTATUS = STATUS;
  type DWORD = u32;
  type ULONG = u32;
  type WORD = u16;
  type Result<T> = ::std::result::Result<T, Value>;

  #[repr(u32)]
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

  #[repr(u32)]
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
  #[derive(Debug, Clone)]
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
  #[derive(Debug, Copy, Clone)]
  struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub DpcTime: i64,
    pub InterruptTime: i64,
    pub InterruptCount: u32,
  }

  #[link(name = "ntdll")]
  extern "system" {
    fn NtQuerySystemInformation(
      SystemInformationClass: SYSTEM_INFORMATION_CLASS,
      SystemInformation: LPVOID,
      SystemInformationLength: UINT,
      ReturnLength: Option<&mut UINT>,
    ) -> NTSTATUS;
  }

  #[link(name = "PowrProf")]
  extern "system" {
    fn CallNtPowerInformation(
      InformationLevel: POWER_INFORMATION_LEVEL,
      lpInputBuffer: LPCVOID,
      nInputBufferSize: ULONG,
      lpOutputBuffer: LPVOID,
      nOutputBufferSize: ULONG,
    ) -> NTSTATUS;
  }

  type KAFFINITY = u32;

  #[repr(C)]
  #[derive(Debug, PartialEq)]
  enum LOGICAL_PROCESSOR_RELATIONSHIP {
    RelationProcessorCore,
    RelationNumaNode,
    RelationCache,
    RelationProcessorPackage,
    RelationGroup,
    RelationAll = 0xffff,
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  struct GROUP_AFFINITY {
    pub Mask: KAFFINITY,
    pub Group: WORD,
    pub Reserved: [WORD; 3],
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  struct PROCESSOR_RELATIONSHIP {
    pub Flags: BYTE,
    pub EfficiencyClass: BYTE,
    pub Reserved: [BYTE; 20],
    pub GroupCount: WORD,
    pub GroupMask: [GROUP_AFFINITY; 1],
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: DWORD,
    pub Reserved: [BYTE; 20],
    pub GroupMask: GROUP_AFFINITY,
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: BYTE,
    pub ActiveProcessorCount: BYTE,
    pub Reserved: [BYTE; 38],
    pub ActiveProcessorMask: KAFFINITY,
  }

  #[repr(C)]
  #[derive(Copy, Clone)]
  struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: WORD,
    pub ActiveGroupCount: WORD,
    pub Reserved: [BYTE; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
  }

  #[repr(C)]
  union SYSTEM_LOGICAL_PROCESSOR_UNION {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    // pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
  }

  #[repr(C)]
  struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: DWORD,
    pub Union: SYSTEM_LOGICAL_PROCESSOR_UNION,
  }

  extern "system" {
    fn GetLogicalProcessorInformationEx(
      RelationshipType: LOGICAL_PROCESSOR_RELATIONSHIP,
      Buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
      ReturnedLength: &mut DWORD,
    ) -> BOOL;
  }
}
