use libspine_sys::spBone;
use raw::*;

pub struct Bone {
    raw: NonNull<spBone>
}

impl_as_raw!(Bone, raw, spBone);
impl_as_raw_mut!(Bone, raw);

impl Bone {
    pub fn from_raw(raw: NonNull<spBone>) -> Self {
        Bone {
            raw
        }
    }
}
