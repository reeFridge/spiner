use libspine_sys::{spRegionAttachment, spRegionAttachment_computeWorldVertices, spBone};
use common::AsPtr;
use bone::Bone;

pub struct Region {
    pub raw_ptr: *const spRegionAttachment
}

impl_as_ptr!(Region, spRegionAttachment);

impl Region {
    pub fn compute_world_vertices(&self, bone: &Bone, vertices: &mut Vec<f32>, offset: i32, stride: i32) {
        unsafe {
            spRegionAttachment_computeWorldVertices(self.raw_ptr as *mut spRegionAttachment, bone.as_ptr() as *mut spBone, vertices.as_mut_ptr(), offset, stride);
        }
    }
}
