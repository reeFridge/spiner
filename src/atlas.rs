use libspine_sys::{spAtlas, spAtlas_createFromFile, spAtlas_dispose};
use std::ffi::CString;
use std::ptr;
use std::ffi::NulError;
use common::AsPtr;

pub struct Atlas {
    raw_ptr: *mut spAtlas
}

impl_as_ptr!(Atlas, spAtlas);

impl Atlas {
    pub fn from_file(path: &str) -> Result<Atlas, NulError> {
        let c_path = CString::new(path)?;
        let raw_ptr = unsafe {
            spAtlas_createFromFile(c_path.as_ptr(), ptr::null_mut())
        };

        Ok(Atlas { raw_ptr })
    }
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe {
            spAtlas_dispose(self.raw_ptr);
        }
    }
}