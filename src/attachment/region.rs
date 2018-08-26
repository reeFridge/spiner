use libspine_sys::{spRegionAttachment, spAtlasRegion, spRegionAttachment_computeWorldVertices, spBone};
use common::AsPtr;
use bone::Bone;
use atlas::region::Region as AtlasRegion;

pub struct Region {
    pub raw_ptr: *const spRegionAttachment
}

impl_as_ptr!(Region, spRegionAttachment);

impl Region {
    pub fn uvs(&self) -> [f32; 8] {
        unsafe {
            (*self.raw_ptr).uvs
        }
    }

    pub fn atlas_region(&self) -> AtlasRegion {
        unsafe {
            AtlasRegion::from((*self.raw_ptr).rendererObject as *const spAtlasRegion)
        }
    }
    
    pub fn compute_world_vertices(&self, bone: &Bone, vertices: &mut Vec<f32>, offset: i32, stride: i32) {
        unsafe {
            spRegionAttachment_computeWorldVertices(self.raw_ptr as *mut spRegionAttachment, bone.as_ptr() as *mut spBone, vertices.as_mut_ptr(), offset, stride);
        }
    }
}
