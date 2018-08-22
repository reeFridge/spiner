use libspine_sys::spBoundingBoxAttachment;
use common::AsPtr;

pub struct BoundingBox {
    pub raw_ptr: *const spBoundingBoxAttachment
}

impl_as_ptr!(BoundingBox, spBoundingBoxAttachment);