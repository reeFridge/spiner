use libspine_sys::spAnimation;
use std::ffi::CStr;
use std::os::raw::c_char;

pub mod state;

#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: f32,
}

impl<'a> From<&'a spAnimation> for Animation {
    fn from(raw_ref: &'a spAnimation) -> Self {
        let name = unsafe {
            CStr::from_ptr(raw_ref.name as *const c_char).to_string_lossy().into_owned()
        };

        let duration = raw_ref.duration;

        Animation {
            name,
            duration,
        }
    }
}
