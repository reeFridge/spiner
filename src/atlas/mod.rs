use self::page::Page;
use libspine_sys::*;
use raw::*;
use std::ffi::CString;
use std::ptr;
use std::slice::Iter;

pub mod page;
pub mod region;

pub struct Atlas {
    raw: NonNull<spAtlas>,
    pages: Vec<Page>,
}

impl_as_raw!(Atlas, raw, spAtlas);
impl_as_raw_mut!(Atlas, raw);

impl Atlas {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let c_path = CString::new(path)?;
        let ptr = unsafe { spAtlas_createFromFile(c_path.as_ptr(), ptr::null_mut()) };

        let raw = try_wrap!(ptr, |raw| raw)?;
        let pages_ptr = unsafe { raw.as_ref().pages };
        let pages = try_wrap!(pages_ptr, |pages_raw| Atlas::collect_pages(pages_raw))?;

        Ok(Atlas { raw, pages })
    }

    pub fn pages(&self) -> Iter<Page> {
        self.pages.iter()
    }

    fn collect_pages(first: NonNull<spAtlasPage>) -> Vec<Page> {
        let mut container = Vec::new();
        let mut current_page = Some(Page::from_raw(first));

        while let Some(page) = current_page {
            current_page = page.next();
            container.push(page);
        }

        container
    }
}

impl Drop for Atlas {
    fn drop(&mut self) {
        unsafe {
            spAtlas_dispose(self.raw.as_ptr());
        }
    }
}
