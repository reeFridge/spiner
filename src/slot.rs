use attachment::Attachment;
use bone::Bone;
use libspine_sys::*;
use raw::*;

pub struct Slot {
    raw: NonNull<spSlot>,
}

impl_as_raw!(Slot, raw, spSlot);
impl_as_raw_mut!(Slot, raw);

impl Slot {
    pub fn from_raw(raw: NonNull<spSlot>) -> Self {
        Slot { raw }
    }

    pub fn attachment(&self) -> Option<Attachment> {
        NonNull::new(self.as_raw().attachment as *mut spAttachment)
            .map(|raw| Attachment::from_raw(raw))
    }

    pub fn bone(&self) -> Option<Bone> {
        NonNull::new(self.as_raw().bone as *mut spBone).map(|raw| Bone::from_raw(raw))
    }
}
