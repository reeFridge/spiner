use libspine_sys::spBone;
use common::AsPtr;

pub struct Bone {
    raw_ptr: *mut spBone
}

impl_as_ptr!(Bone, spBone);

impl From<*const spBone> for Bone {
    fn from(raw_ptr: *const spBone) -> Self {
        Bone {
            raw_ptr: raw_ptr as *mut spBone
        }
    }
}
