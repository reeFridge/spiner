use super::json::Json;
use animation::Animation;
use common::from_raw_buf;
use libspine_sys::*;
use raw::*;
use std::ffi::CString;
use std::ptr::NonNull;

pub struct Data {
    raw: NonNull<spSkeletonData>,
}

impl_as_raw!(Data, raw, spSkeletonData);
impl_as_raw_mut!(Data, raw);

impl Data {
    pub fn animations(&self) -> Vec<Animation> {
        let raw = self.as_raw();
        let animations_count = raw.animationsCount as usize;
        let animations_raw = unsafe { from_raw_buf(raw.animations, animations_count) };

        animations_raw
            .iter()
            .filter_map(|p| unsafe { p.as_ref().map(|anim_ref| Animation::from(anim_ref)) })
            .collect()
    }

    pub fn find_animation_by_name(&self, name: &str) -> Option<Animation> {
        let raw_ptr = unsafe {
            let c_str = CString::new(name).unwrap();
            spSkeletonData_findAnimation(
                self.as_raw() as *const _ as *mut spSkeletonData,
                c_str.as_ptr(),
            )
        };

        unsafe { raw_ptr.as_ref().map(|anim_ref| Animation::from(anim_ref)) }
    }

    pub fn from_raw(raw: NonNull<spSkeletonData>) -> Self {
        Data { raw }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.raw.as_ptr());
        }
    }
}
