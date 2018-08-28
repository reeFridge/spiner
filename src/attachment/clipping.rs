use libspine_sys::spClippingAttachment;
use raw::*;

pub struct Clipping {
    raw: NonNull<spClippingAttachment>,
}

impl Clipping {
    pub fn from_raw(raw: NonNull<spClippingAttachment>) -> Self {
        Clipping {
            raw
        }
    }
}

impl_as_raw!(Clipping, raw, spClippingAttachment);
impl_as_raw_mut!(Clipping, raw);
