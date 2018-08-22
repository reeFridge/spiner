use libspine_sys::spPointAttachment;
use common::AsPtr;

pub struct Point {
    pub raw_ptr: *const spPointAttachment
}

impl_as_ptr!(Point, spPointAttachment);