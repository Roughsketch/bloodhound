use winapi::ctypes::c_void;
use std::mem::transmute;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Address(usize);

impl Address {
    pub fn new(addr: usize) -> Self {
        Address(addr)
    }

    pub fn from_ptr(ptr: *mut c_void) -> Self {
        let addr = unsafe { transmute::<*mut c_void, usize>(ptr) };
        Address(addr)
    }

    pub fn null() -> Self {
        Address(0)
    }

    pub fn as_ptr(&self) -> *mut c_void {
        unsafe { transmute::<usize, *mut c_void>(self.0) }
    }

    pub fn inner(&self) -> usize {
        self.0
    }

    pub fn add(&mut self, value: usize) {
        self.0 += value;
    }

    pub fn sub(&mut self, value: usize) {
        self.0 += value;
    }
}

impl From<*mut c_void> for Address {
    fn from(ptr: *mut c_void) -> Address {
        Address::from_ptr(ptr)
    }
}

impl From<Address> for *mut c_void {
    fn from(addr: Address) -> *mut c_void {
        addr.as_ptr()
    }
}