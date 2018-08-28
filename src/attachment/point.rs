use libspine_sys::spPointAttachment;
use raw::*;

pub struct Point {
    raw: NonNull<spPointAttachment>
}

impl Point {
    pub fn from_raw(raw: NonNull<spPointAttachment>) -> Self {
        Point {
            raw
        }
    }
}

impl_as_raw!(Point, raw, spPointAttachment);
impl_as_raw_mut!(Point, raw);
