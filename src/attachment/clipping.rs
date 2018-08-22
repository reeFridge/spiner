use libspine_sys::spClippingAttachment;
use common::AsPtr;

pub struct Clipping {
    pub raw_ptr: *const spClippingAttachment
}

impl_as_ptr!(Clipping, spClippingAttachment);