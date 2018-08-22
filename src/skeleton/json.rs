use super::data::Data as SkeletonData;
use std::ffi::CString;
use std::ffi::NulError;
use std::error::Error;
use common::AsPtr;
use libspine_sys::{
    spAtlas,
    spSkeletonJson,
    spSkeletonJson_create,
    spSkeletonJson_readSkeletonDataFile,
    spSkeletonJson_dispose
};
use std::ffi::CStr;


pub struct Json {
    raw_ptr: *mut spSkeletonJson
}

#[derive(Debug)]
pub struct JsonError(String);

impl From<NulError> for JsonError {
    fn from(err: NulError) -> Self {
        JsonError(err.description().to_string())
    }
}

impl_as_ptr!(Json, spSkeletonJson);

impl Json {
    pub fn new(atlas: &mut AsPtr<spAtlas>, scale: f32) -> Json {
        let raw_ptr = unsafe {
            let json = spSkeletonJson_create(atlas.as_mut_ptr());
            (*json).scale = scale;
            json
        };

        Json { raw_ptr }
    }

    pub fn error(&self) -> Option<JsonError> {
        let error = unsafe {
            (*self.raw_ptr).error.as_ref().map(|err| {
                CStr::from_ptr(err).to_string_lossy().into_owned()
            })
        };

        error.map(|err| JsonError(err))
    }

    pub fn read_skeleton_file(self, path: &str) -> Result<SkeletonData, JsonError> {
        let c_path = CString::new(path)?;
        let raw = unsafe {
            spSkeletonJson_readSkeletonDataFile(self.raw_ptr, c_path.as_ptr())
        };

        match self.error() {
            Some(err) => Err(err),
            None => Ok(SkeletonData::from(raw))
        }
    }
}

impl Drop for Json {
    fn drop(&mut self) {
        unsafe {
            spSkeletonJson_dispose(self.raw_ptr);
        }
    }
}