use libspine_sys::spRegionAttachment;
use common::AsPtr;

pub struct Region {
    pub raw_ptr: *const spRegionAttachment
}

impl_as_ptr!(Region, spRegionAttachment);