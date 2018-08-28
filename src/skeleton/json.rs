use super::data::Data as SkeletonData;
use atlas::Atlas;
use libspine_sys::*;
use raw::*;
use std::error::Error as ErrorTrait;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::NulError;

pub struct Json {
    raw: NonNull<spSkeletonJson>,
}

impl_as_raw!(Json, raw, spSkeletonJson);
impl_as_raw_mut!(Json, raw);

#[derive(Debug)]
pub struct JsonError(String);

impl From<NulError> for JsonError {
    fn from(err: NulError) -> Self {
        JsonError(err.description().to_string())
    }
}

impl From<Error> for JsonError {
    fn from(err: Error) -> Self {
        JsonError(err.description().to_string())
    }
}

impl Json {
    pub fn new(atlas: &mut Atlas, scale: f32) -> Result<Json, JsonError> {
        let ptr = unsafe { spSkeletonJson_create(atlas.as_raw_mut()) };

        let mut raw = try_wrap!(ptr, |raw| raw)?;
        unsafe { raw.as_mut().scale = scale };

        Ok(Json { raw })
    }

    pub fn error(&self) -> Option<JsonError> {
        let error = unsafe {
            self.as_raw()
                .error
                .as_ref()
                .map(|err| CStr::from_ptr(err).to_string_lossy().into_owned())
        };

        error.map(|err| JsonError(err))
    }

    pub fn read_skeleton_file(mut self, path: &str) -> Result<SkeletonData, JsonError> {
        let c_path = CString::new(path)?;
        let ptr =
            unsafe { spSkeletonJson_readSkeletonDataFile(self.as_raw_mut(), c_path.as_ptr()) };

        match self.error() {
            Some(err) => Err(err),
            None => Ok(try_wrap!(ptr, |raw| SkeletonData::from_json(self, raw))?),
        }
    }
}

impl Drop for Json {
    fn drop(&mut self) {
        unsafe {
            spSkeletonJson_dispose(self.raw.as_ptr());
        }
    }
}
