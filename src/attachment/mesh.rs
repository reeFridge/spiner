use libspine_sys::spMeshAttachment;
use common::AsPtr;

pub struct Mesh {
    pub raw_ptr: *const spMeshAttachment
}

impl_as_ptr!(Mesh, spMeshAttachment);