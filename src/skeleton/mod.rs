use animation::Animation;
use common::from_raw_buf;
use libspine_sys::*;
use raw::*;
use skeleton::data::Data;
use slot::Slot;
use std::ptr::NonNull;
use std::rc::Rc;

pub mod data;
pub mod json;

pub struct Skeleton {
    data: Rc<Data>,
    raw: NonNull<spSkeleton>,
}

impl_as_raw!(Skeleton, raw, spSkeleton);
impl_as_raw_mut!(Skeleton, raw);

impl Skeleton {
    pub fn from_data(data: Rc<Data>) -> Result<Self, Error> {
        let ptr = unsafe { spSkeleton_create(data.as_raw() as *const _ as *mut spSkeletonData) };

        try_wrap!(ptr, |raw| Skeleton { data, raw })
    }

    pub fn animations(&self) -> Vec<Animation> {
        self.data.animations()
    }

    pub fn update_world_transform(&mut self) {
        unsafe {
            spSkeleton_updateWorldTransform(self.as_raw_mut());
        }
    }

    pub fn slots(&self) -> Vec<Slot> {
        unsafe { self.collect_slots(NonNull::new_unchecked(self.as_raw().slots)) }
    }

    pub fn slots_ordered(&self) -> Vec<Slot> {
        unsafe { self.collect_slots(NonNull::new_unchecked(self.as_raw().drawOrder)) }
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.set_position_x(position.0);
        self.set_position_y(position.1);
    }

    pub fn set_position_x(&mut self, x: f32) {
        unsafe {
            self.raw.as_mut().x = x;
        }
    }

    pub fn set_position_y(&mut self, y: f32) {
        unsafe {
            self.raw.as_mut().y = y;
        }
    }

    fn collect_slots(&self, slots: NonNull<*mut spSlot>) -> Vec<Slot> {
        let slots_count = self.as_raw().slotsCount as usize;
        let slots_raw = unsafe { from_raw_buf(slots.as_ptr(), slots_count) };

        slots_raw
            .iter()
            .filter_map(|p| NonNull::new(*p).map(|raw| Slot::from_raw(raw)))
            .collect()
    }
}

impl Drop for Skeleton {
    fn drop(&mut self) {
        unsafe {
            spSkeleton_dispose(self.raw.as_ptr());
        }
    }
}
