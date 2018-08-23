use libspine_sys::{spSlot, spAttachment, spBone};
use attachment::Attachment;
use common::AsPtr;
use bone::Bone;

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

    pub fn bone(&self) -> Option<Bone> {
        let bone_ref = unsafe {
            (*self.raw_ptr).bone.as_ref()
        };

        bone_ref.map(|bone| Bone::from(bone as *const spBone))
    }
}
