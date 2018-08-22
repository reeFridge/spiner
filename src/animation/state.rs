use libspine_sys::{
    spAnimationState,
    spAnimationState_create,
    spAnimationState_dispose,
    spAnimationState_setAnimationByName,
    spAnimationState_update,
    spAnimationState_apply,
};
use libspine_sys::{spAnimationStateData, spAnimationStateData_create, spAnimationStateData_dispose};
use libspine_sys::spSkeletonData;
use skeleton::data::Data as SkeletonData;
use animation::Animation;
use std::ffi::CString;
use skeleton::Skeleton;

pub struct State {
    raw_ptr: *mut spAnimationState
}

impl State {
    pub fn set_animation(&mut self, track_index: i32, animation: &Animation, loop_: bool) {
        unsafe {
            let c_str = CString::new(animation.name.clone()).unwrap();
            let _track = spAnimationState_setAnimationByName(self.raw_ptr, track_index, c_str.as_ptr(), loop_ as i32);
        }
    }

    pub fn update(&mut self, delta: f32) {
        unsafe {
            spAnimationState_update(self.raw_ptr, delta);
        }
    }

    pub fn apply(&mut self, skeleton: &mut Skeleton) {
        let _result = unsafe {
            spAnimationState_apply(self.raw_ptr, skeleton.raw_ptr)
        };
    }
}

impl<'a> From<&'a StateData<'a>> for State {
    fn from(data: &'a StateData<'a>) -> Self {
        let raw_ptr = unsafe {
            spAnimationState_create(data.raw_ptr as *mut spAnimationStateData)
        };

        State {
            raw_ptr
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            spAnimationState_dispose(self.raw_ptr);
        }
    }
}

pub struct StateData<'a> {
    data: &'a SkeletonData,
    pub raw_ptr: *mut spAnimationStateData,
}

impl<'a> StateData<'a> {
    pub fn set_default_mix(&mut self, val: f32) {
        unsafe {
            (*self.raw_ptr).defaultMix = val;
        }
    }
}

impl<'a> From<&'a SkeletonData> for StateData<'a> {
    fn from(data: &'a SkeletonData) -> Self {
        let raw_ptr = unsafe {
            spAnimationStateData_create(data.raw_ptr as *mut spSkeletonData)
        };

        StateData {
            raw_ptr,
            data,
        }
    }
}

impl<'a> Drop for StateData<'a> {
    fn drop(&mut self) {
        unsafe {
            spAnimationStateData_dispose(self.raw_ptr);
        }
    }
}