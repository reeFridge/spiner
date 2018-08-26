use extension::Texture;
use libspine_sys::spAtlasPage;
use std::ffi::CStr;

pub struct Page {
    pub name: String,
    pub width: i32,
    pub height: i32,
    raw_ptr: *const spAtlasPage,
}

impl Page {
    pub fn renderer_object(&self) -> *mut Texture {
        unsafe { (*self.raw_ptr).rendererObject as *mut Texture }
    }

    pub fn next(&self) -> Option<Page> {
        let next_ptr = unsafe { (*self.raw_ptr).next.as_ref() };

        next_ptr.map(|ptr| Page::from(ptr as *const spAtlasPage))
    }
}

impl From<*const spAtlasPage> for Page {
    fn from(raw_ptr: *const spAtlasPage) -> Self {
        let name = unsafe {
            CStr::from_ptr((*raw_ptr).name).to_string_lossy().into_owned()
        };
        let (width, height) = unsafe {
            ((*raw_ptr).width, (*raw_ptr).height)
        };
        
        Page { name, width, height, raw_ptr }
    }
}
