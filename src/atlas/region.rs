use libspine_sys::{spAtlasRegion, spAtlasPage};
use super::page::Page;

pub struct Region {
    raw_ptr: *const spAtlasRegion
}

impl Region {
    pub fn page(&self) -> Page {
        unsafe {
            Page::from((*self.raw_ptr).page as *const spAtlasPage)
        }
    }
}

impl From<*const spAtlasRegion> for Region {
    fn from(raw_ptr: *const spAtlasRegion) -> Self {
        Region {
            raw_ptr
        }
    }
}
