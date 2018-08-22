use libspine_sys::{spSlot, spAttachment};
use attachment::Attachment;
use common::AsPtr;

pub struct Slot {
    raw_ptr: *mut spSlot
}

impl_as_ptr!(Slot, spSlot);

impl From<*mut spSlot> for Slot {
    fn from(raw_ptr: *mut spSlot) -> Self {
        Slot {
            raw_ptr
        }
    }
}

impl Slot {
    pub fn attachment(&self) -> Option<Attachment> {
        let attach_ref = unsafe {
            (*self.raw_ptr).attachment.as_ref()
        };

        attach_ref.map(|attach| Attachment::from(attach as *const spAttachment))
    }
}