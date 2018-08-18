use model::region::Region;

use byteorder::{LittleEndian, ReadBytesExt};

pub struct Value {
    base: Region,
    value: u32,
}

impl Value {
    pub fn new(base: Region, value: u32) -> Self {
        Self {
            base,
            value,
        }
    }

    pub fn check<F: FnOnce(u32, u32) -> bool>(&mut self, valid: F) -> bool {
        if let Ok(data) = self.base.get() {
            if let Ok(new_value) = data.as_slice().read_u32::<LittleEndian>() {
                if valid(self.value, new_value) {
                    self.value = new_value;
                    return true;
                }
            }
        }

        false
    }

    pub fn address(&self) -> usize {
        self.base.base.inner()
    }
    
    pub fn get(&self) -> Option<u32> {
        if let Ok(data) = self.base.get() {
            if let Ok(value) = data.as_slice().read_u32::<LittleEndian>() {
                return Some(value);
            }
        }

        None
    }
}