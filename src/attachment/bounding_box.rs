use libspine_sys::spBoundingBoxAttachment;
use raw::*;

pub struct BoundingBox {
    raw: NonNull<spBoundingBoxAttachment>,
}

impl BoundingBox {
    pub fn from_raw(raw: NonNull<spBoundingBoxAttachment>) -> Self {
        BoundingBox {
            raw
        }
    }
}

impl_as_raw!(BoundingBox, raw, spBoundingBoxAttachment);
impl_as_raw_mut!(BoundingBox, raw);
