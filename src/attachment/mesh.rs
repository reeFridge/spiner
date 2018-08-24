use super::vertex::Vertex;
use common::{AsPtr, from_raw_buf};
use libspine_sys::{spMeshAttachment, spVertexAttachment_computeWorldVertices, spSlot, spVertexAttachment};
use slot::Slot;

pub struct Mesh {
    pub raw_ptr: *const spMeshAttachment,
}

impl_as_ptr!(Mesh, spMeshAttachment);

impl Mesh {
    pub fn triangles(&self) -> Vec<u16> {
        unsafe {
            let len = (*self.raw_ptr).trianglesCount as usize;
            from_raw_buf((*self.raw_ptr).triangles, len)
        }
    }
}

impl Vertex for Mesh {
    fn world_vertices_len(&self) -> usize {
        unsafe { (*self.raw_ptr).super_.worldVerticesLength as usize }
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
            let super_: *const spVertexAttachment = &(*self.raw_ptr).super_;
            spVertexAttachment_computeWorldVertices(super_ as *mut spVertexAttachment, slot.as_ptr() as *mut spSlot, start, count, vertices.as_mut_ptr(), offset, stride);
        }
    }
}
