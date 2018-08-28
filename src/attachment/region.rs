use atlas::region::Region as AtlasRegion;
use bone::Bone;
use libspine_sys::*;
use raw::*;

pub struct Region {
    raw: NonNull<spRegionAttachment>,
}

impl_as_raw!(Region, raw, spRegionAttachment);
impl_as_raw_mut!(Region, raw);

impl Region {
    pub fn from_raw(raw: NonNull<spRegionAttachment>) -> Region {
        Region { raw }
    }

    pub fn uvs(&self) -> [f32; 8] {
        self.as_raw().uvs
    }

    pub fn atlas_region(&self) -> Option<AtlasRegion> {
        let ptr = self.as_raw().rendererObject as *mut spAtlasRegion;

        NonNull::new(ptr).map(|raw| AtlasRegion::from_raw(raw))
    }

    pub fn compute_world_vertices(
        &self,
        bone: &Bone,
        vertices: &mut Vec<f32>,
        offset: i32,
        stride: i32,
    ) {
        unsafe {
            let self_: *const _ = self.as_raw();
            let bone: *const _ = bone.as_raw();
            spRegionAttachment_computeWorldVertices(
                self_ as *mut spRegionAttachment,
                bone as *mut spBone,
                vertices.as_mut_ptr(),
                offset,
                stride,
            );
        }
    }
}
