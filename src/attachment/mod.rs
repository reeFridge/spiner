use libspine_sys::{spAttachment, spAttachmentType::{self, *}};
use libspine_sys::{
    spBoundingBoxAttachment,
    spMeshAttachment,
    spPathAttachment,
    spPointAttachment,
    spRegionAttachment,
    spClippingAttachment,
};
use common::AsPtr;
use std::ffi::CStr;

pub mod bounding_box;
pub mod mesh;
pub mod path;
pub mod point;
pub mod region;
pub mod clipping;
pub mod vertex;

use self::bounding_box::BoundingBox;
use self::region::Region;
use self::mesh::Mesh;
use self::path::Path;
use self::point::Point;
use self::clipping::Clipping;

macro_rules! impl_attachment {
    ($t:ident) => {
        impl Base for $t {
            fn name(&self) -> Option<String> {
                unsafe {
                    (*(self.as_ptr() as *const spAttachment)).name
                        .as_ref()
                        .map(|name| CStr::from_ptr(name).to_string_lossy().to_owned().into())
                }
            }

            fn raw_type(&self) -> AttachmentType {
                unsafe {
                    (*(self.as_ptr() as *const spAttachment)).type_
                }
            }
        }
    }
}

macro_rules! create_attachment {
    ($t:ident, $raw_type:ident, $ptr:ident) => {
        Attachment::$t($t { raw_ptr: $ptr as *const $raw_type })
    }
}

macro_rules! attachment_method {
    ($s:ident, $m:ident) => {
        match *$s {
            Attachment::Region(ref region) => region.$m(),
            Attachment::BoundingBox(ref bounding_box) => bounding_box.$m(),
            Attachment::Mesh(ref mesh) => mesh.$m(),
            Attachment::LinkedMesh(ref mesh) => mesh.$m(),
            Attachment::Path(ref path) => path.$m(),
            Attachment::Point(ref point) => point.$m(),
            Attachment::Clipping(ref clipping) => clipping.$m()
        }
    }
}

pub type AttachmentType = spAttachmentType;

pub enum Attachment {
    Region(Region),
    BoundingBox(BoundingBox),
    Mesh(Mesh),
    LinkedMesh(Mesh),
    Path(Path),
    Point(Point),
    Clipping(Clipping),
}

impl Base for Attachment {
    fn name(&self) -> Option<String> {
        attachment_method!(self, name)
    }
    fn raw_type(&self) -> AttachmentType {
        attachment_method!(self, raw_type)
    }
}

impl_attachment!(BoundingBox);
impl_attachment!(Mesh);
impl_attachment!(Path);
impl_attachment!(Point);
impl_attachment!(Region);
impl_attachment!(Clipping);

pub trait Base {
    fn name(&self) -> Option<String>;
    fn raw_type(&self) -> AttachmentType;
}

impl From<*const spAttachment> for Attachment {
    fn from(raw_ptr: *const spAttachment) -> Self {
        let type_ = unsafe {
            (*raw_ptr).type_
        };

        match type_ {
            SP_ATTACHMENT_REGION => create_attachment!(Region, spRegionAttachment, raw_ptr),
            SP_ATTACHMENT_BOUNDING_BOX => create_attachment!(BoundingBox, spBoundingBoxAttachment, raw_ptr),
            SP_ATTACHMENT_MESH | SP_ATTACHMENT_LINKED_MESH => create_attachment!(Mesh, spMeshAttachment, raw_ptr),
            SP_ATTACHMENT_PATH => create_attachment!(Path, spPathAttachment, raw_ptr),
            SP_ATTACHMENT_POINT => create_attachment!(Point, spPointAttachment, raw_ptr),
            SP_ATTACHMENT_CLIPPING => create_attachment!(Clipping, spClippingAttachment, raw_ptr)
        }
    }
}
