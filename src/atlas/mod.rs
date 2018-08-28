use self::page::Page;
use libspine_sys::*;
use raw::*;
use std::ffi::CString;
use std::ptr;

pub mod page;
pub mod region;

pub struct Atlas {
    raw: NonNull<spAtlas>,
}

impl_as_raw!(Atlas, raw, spAtlas);
impl_as_raw_mut!(Atlas, raw);

impl Atlas {
    pub fn pages(&self) -> Vec<Page> {
        let mut container = Vec::new();
        let page_raw = NonNull::new(self.as_raw().pages);

        if let Some(raw) = page_raw {
            let mut current_page = Some(Page::from_raw(raw));

            while let Some(page) = current_page {
                current_page = page.next();
                container.push(page);
            }
        }

        container
    }

    pub fn from_file(path: &str) -> Result<Atlas, Error> {
        let c_path = CString::new(path)?;
        let ptr = unsafe { spAtlas_createFromFile(c_path.as_ptr(), ptr::null_mut()) };

        try_wrap!(ptr, |raw| Atlas { raw })
    }
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe {
            spAtlas_dispose(self.raw.as_ptr());
        }
    }
}
