use super::vertex::Vertex;
use atlas::region::Region as AtlasRegion;
use common::from_raw_buf;
use libspine_sys::*;
use raw::*;
use slot::Slot;

pub struct Mesh {
    raw: NonNull<spMeshAttachment>,
}

impl_as_raw!(Mesh, raw, spMeshAttachment);
impl_as_raw_mut!(Mesh, raw);

impl Mesh {
    pub fn from_raw(raw: NonNull<spMeshAttachment>) -> Self {
        Mesh { raw }
    }

    pub fn uvs(&self) -> Vec<f32> {
        unsafe {
            let len = self.world_vertices_len();
            from_raw_buf(self.as_raw().uvs, len)
        }
    }

    pub fn atlas_region(&self) -> Option<AtlasRegion> {
        let ptr = self.as_raw().rendererObject as *mut spAtlasRegion;

        NonNull::new(ptr).map(|raw| AtlasRegion::from_raw(raw))
    }

    pub fn triangles(&self) -> Vec<u16> {
        unsafe {
            let len = self.as_raw().trianglesCount as usize;
            from_raw_buf(self.as_raw().triangles, len)
        }
    }
}

impl Vertex for Mesh {
    fn world_vertices_len(&self) -> usize {
        self.as_raw().super_.worldVerticesLength as usize
    }

    fn compute_world_vertices(
        &self,
        slot: &Slot,
        start: i32,
        count: i32,
        vertices: &mut Vec<f32>,
        offset: i32,
        stride: i32,
    ) {
        unsafe {
            let vertex: *const _ = &self.as_raw().super_;
            let slot: *const _ = slot.as_raw();
            spVertexAttachment_computeWorldVertices(
                vertex as *mut spVertexAttachment,
                slot as *mut spSlot,
                start,
                count,
                vertices.as_mut_ptr(),
                offset,
                stride,
            );
        }
    }
}
