use animation::Animation;
use libspine_sys::*;
use raw::*;
use skeleton::{data::Data as SkeletonData, Skeleton};
use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::ptr::NonNull;
use std::rc::Rc;

pub struct State {
    data: Rc<StateData>,
    raw: NonNull<spAnimationState>,
}

impl_as_raw!(State, raw, spAnimationState);
impl_as_raw_mut!(State, raw);

impl State {
    pub fn from_data(data: Rc<StateData>) -> Result<Self, Error> {
        let ptr = unsafe {
            spAnimationState_create(data.as_raw() as *const _ as *mut spAnimationStateData)
        };

        try_wrap!(ptr, |raw| State{ data, raw })
    }

    pub fn set_animation(&mut self, track_index: i32, animation: &Animation, loop_: bool) {
        unsafe {
            let c_str = CString::new(animation.name.clone()).unwrap();
            let _track = spAnimationState_setAnimationByName(
                self.as_raw_mut(),
                track_index,
                c_str.as_ptr(),
                loop_ as i32,
            );
        }
    }

    pub fn update(&mut self, delta: f32) {
        unsafe {
            spAnimationState_update(self.as_raw_mut(), delta);
        }
    }

    pub fn apply(&mut self, skeleton: &mut Skeleton) {
        let _result = unsafe { spAnimationState_apply(self.as_raw_mut(), skeleton.as_raw_mut()) };
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            spAnimationState_dispose(self.raw.as_ptr());
        }
    }
}

pub struct StateData {
    data: Rc<SkeletonData>,
    raw: NonNull<spAnimationStateData>,
}

impl_as_raw!(StateData, raw, spAnimationStateData);
impl_as_raw_mut!(StateData, raw);

impl StateData {
    pub fn from_skeleton_data(data: Rc<SkeletonData>) -> Result<Self, Error> {
        let ptr = unsafe {
            spAnimationStateData_create(data.as_raw() as *const _ as *mut spSkeletonData)
        };

        try_wrap!(ptr, |raw| StateData { data, raw })
    }

    pub fn set_default_mix(&mut self, val: f32) {
        unsafe {
            self.raw.as_mut().defaultMix = val;
        }
    }
}

impl Drop for StateData {
    fn drop(&mut self) {
        unsafe {
            spAnimationStateData_dispose(self.raw.as_ptr());
        }
    }
}
