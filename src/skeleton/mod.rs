use skeleton::data::Data;
use slot::Slot;
use libspine_sys::{spSkeleton, spSlot, spSkeleton_create, spSkeleton_dispose, spSkeletonData, spSkeleton_updateWorldTransform};
use common::{AsPtr, from_raw_buf};
use animation::Animation;

pub mod data;
pub mod json;

pub struct Skeleton<'a> {
    data: &'a Data,
    pub raw_ptr: *mut spSkeleton
}

impl<'a> Skeleton<'a> {
    pub fn animations(&self) -> Vec<Animation> {
        self.data.animations()
    }

    pub fn update_world_transform(&mut self) {
        unsafe {
            spSkeleton_updateWorldTransform(self.raw_ptr);
        }
    }

    pub fn slots(&self) -> Vec<Slot> {
        let slots = unsafe { (*self.raw_ptr).slots.as_ref() };
        self.collect_slots(slots.unwrap())
    }

    pub fn slots_ordered(&self) -> Vec<Slot> {
        let slots = unsafe { (*self.raw_ptr).drawOrder.as_ref() };
        self.collect_slots(slots.unwrap())
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.set_position_x(position.0);
        self.set_position_y(position.1);
    }

    pub fn set_position_x(&mut self, x: f32) {
        unsafe {
            (*self.raw_ptr).x = x;
        }
    }

    pub fn set_position_y(&mut self, y: f32) {
        unsafe {
            (*self.raw_ptr).y = y;
        }
    }

    fn collect_slots(&self, slots: &*mut spSlot) -> Vec<Slot> {
        let slots_count = unsafe { (*self.raw_ptr).slotsCount } as usize;
        let slots_raw = unsafe { from_raw_buf(slots, slots_count) };

        slots_raw.iter().map(|p| Slot::from(*p)).collect()
    }
}

impl<'a> From<&'a Data> for Skeleton<'a> {
    fn from(data: &'a Data) -> Self {
        let raw_ptr = unsafe {
            spSkeleton_create(data.as_ptr() as *mut spSkeletonData)
        };

        Skeleton {
            data,
            raw_ptr
        }
    }
}

impl<'a> Drop for Skeleton<'a> {
    fn drop(&mut self) {
        unsafe {
            spSkeleton_dispose(self.raw_ptr);
        }
    }
}