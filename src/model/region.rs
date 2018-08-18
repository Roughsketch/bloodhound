use model::address::Address;
use model::value::Value;

use failure::Error;
use kernel32::ReadProcessMemory;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::LPVOID;

use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Fail, Debug)]
#[fail(display = "Error reading memory ({})", _0)]
struct MemoryReadError(u32);

#[derive(Debug, Clone)]
pub struct Region {
    pub base: Address,
    handle: Address,
    size: usize,
}

impl Region {
    pub fn new(base: Address, handle: Address, size: usize) -> Self {
        Self {
            base,
            handle,
            size,
        }
    }

    pub fn get(&self) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::with_capacity(self.size);
        unsafe { buffer.set_len(self.size); }

        let mut read = 0;

        let result = unsafe {
            ReadProcessMemory(self.handle.as_ptr(),
                self.base.as_ptr(),
                buffer.as_mut_ptr() as LPVOID,
                buffer.len() as u64,
                &mut read)
        };

        ensure!(result != 0, MemoryReadError(unsafe { GetLastError() }));

        Ok(buffer)
    }

    pub fn search(&self, value: u32) -> Vec<Value> {
        let data = self.get().unwrap_or(Vec::new());

        data.exact_chunks(size_of::<u32>())
            .enumerate()
            .filter_map(|(index, mut chunk)| {
                let res = chunk.read_u32::<LittleEndian>().unwrap();

                if res == value {
                    let new_base = Address::new(self.base.inner() + index * size_of::<u32>());

                    let new_region = Region::new(new_base, self.handle, size_of::<u32>());

                    Some(Value::new(new_region, value))
                } else {
                    None
                }
            })
            .collect::<Vec<Value>>()
    }
}