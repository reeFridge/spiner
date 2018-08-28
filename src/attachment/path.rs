use libspine_sys::spPathAttachment;
use raw::*;

pub struct Path {
    raw: NonNull<spPathAttachment>,
}

impl Path {
    pub fn from_raw(raw: NonNull<spPathAttachment>) -> Self {
        Path {
            raw
        }
    }
}

impl_as_raw!(Path, raw, spPathAttachment);
impl_as_raw_mut!(Path, raw);
