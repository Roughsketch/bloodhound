use model::address::Address;
use model::value::Value;
use model::region::Region;

use failure::Error;
use rayon::prelude::*;
use winapi::um::errhandlingapi::GetLastError;

use std;
use std::mem::size_of;

#[derive(Fail, Debug)]
pub enum ProcessError {
    #[fail(display = "Could not open process ({})", _0)]
    OpenError(u32),
}

pub struct Process {
    handle: Address,
    regions: Vec<Region>,
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            use kernel32::CloseHandle;
            CloseHandle(self.handle.as_ptr());
        }
    }
}

impl Process {
    pub fn new(pid: u32) -> Result<Self, Error> {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_ALL_ACCESS;

        let handle = unsafe { 
            let h = Address::from_ptr(OpenProcess(PROCESS_ALL_ACCESS, 0, pid));
            let err = GetLastError();
            ensure!(err == 0, ProcessError::OpenError(err));
            h
        };

        Ok(Self {
            handle,
            regions: Vec::new(),
        })
    }
    
    pub fn search(&mut self, value: u32) -> Vec<Value> {
        self.fetch_regions();

        self.regions
            .par_iter()
            .flat_map(|region| {
                region.search(value)
            })
            .collect::<Vec<Value>>()
    }

    fn fetch_regions(&mut self) {
        use winapi::um::winnt::*;
        use winapi::um::memoryapi::VirtualQueryEx;

        let mut info = MEMORY_BASIC_INFORMATION {
            BaseAddress: std::ptr::null_mut(),
            AllocationBase: std::ptr::null_mut(),
            AllocationProtect: 0,
            RegionSize: 0,
            State: 0,
            Protect: 0,
            Type: 0,
        };
        
        let mut addr = Address::null();
        self.regions.clear();

        loop {
            //  Get next memory region
            unsafe {
                VirtualQueryEx(self.handle.as_ptr(), addr.as_ptr(), &mut info, size_of::<MEMORY_BASIC_INFORMATION>());

                if GetLastError() != 0 {
                    break;
                }
            }

            //  If if fulfills the requirements, add it to regions
            if info.State == MEM_COMMIT && info.Protect & PAGE_GUARD == 0 && info.Type == MEM_PRIVATE || info.Type == MEM_MAPPED {
                self.regions.push(Region::new(info.BaseAddress, self.handle, info.RegionSize));
            }

            addr.add(info.RegionSize);
        }
    }
}