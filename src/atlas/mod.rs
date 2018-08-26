use libspine_sys::*;
use std::ffi::CString;
use std::ptr;
use std::ffi::NulError;
use common::AsPtr;
use self::page::Page;

pub mod region;
pub mod page;

pub struct Atlas {
    raw_ptr: *mut spAtlas
}

impl_as_ptr!(Atlas, spAtlas);

impl Atlas {
    pub fn pages(&self) -> Vec<Page> {
        let mut container = Vec::new();
        let page_ptr = unsafe {
            (*self.raw_ptr).pages.as_ref()
        };

        if let Some(ptr) = page_ptr {
            let mut current_page = Some(Page::from(ptr as *const spAtlasPage));
            while let Some(page) = current_page {
                current_page = page.next();
                container.push(page);
            }
        }

        container
    }
    
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
