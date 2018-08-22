use libspine_sys::spPathAttachment;
use common::AsPtr;

pub struct Path {
    pub raw_ptr: *const spPathAttachment
}

impl_as_ptr!(Path, spPathAttachment);