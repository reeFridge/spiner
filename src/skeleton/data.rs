use libspine_sys::{spSkeletonData, spSkeletonData_dispose};
use common::{AsPtr, from_raw_buf};
use animation::Animation;

pub struct Data {
    pub raw_ptr: *mut spSkeletonData
}

impl_as_ptr!(Data, spSkeletonData);

impl Data {
    pub fn animations(&self) -> Vec<Animation> {
        let animations_count = unsafe { (*self.raw_ptr).animationsCount } as usize;
        let animations_raw = unsafe {
            from_raw_buf((*self.raw_ptr).animations, animations_count)
        };

        animations_raw.iter()
            .filter_map(|p| unsafe {
                p.as_ref().map(|anim_ref| Animation::from(anim_ref))
            })
            .collect()
    }
}

impl From<*mut spSkeletonData> for Data {
    fn from(raw_ptr: *mut spSkeletonData) -> Self {
        Data {
            raw_ptr
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            spSkeletonData_dispose(self.raw_ptr);
        }
    }
}