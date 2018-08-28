use extension::Texture;
use libspine_sys::*;
use std::ffi::CStr;
use raw::*;

pub struct Page {
    pub name: String,
    pub width: i32,
    pub height: i32,
    raw: NonNull<spAtlasPage>,
}

impl_as_raw!(Page, raw, spAtlasPage);
impl_as_raw_mut!(Page, raw);

impl Page {
    pub fn from_raw(raw: NonNull<spAtlasPage>) -> Self {
        let raw_ref = unsafe { raw.as_ref() };
        let name = unsafe { CStr::from_ptr(raw_ref.name).to_string_lossy().into_owned() };
        let (width, height) = (raw_ref.width, raw_ref.height);

        Page {
            name,
            width,
            height,
            raw,
        }
    }

    pub fn renderer_object(&self) -> Option<&Texture> {
        let ptr = self.as_raw().rendererObject as *mut Texture;

        unsafe { ptr.as_ref() }
    }

    pub fn next(&self) -> Option<Page> {
        NonNull::new(self.as_raw().next).map(|raw| Page::from_raw(raw))
    }
}
