use libspine_sys::spAttachmentType::*;
use libspine_sys::*;
use raw::*;
use std::ffi::CStr;

pub mod bounding_box;
pub mod clipping;
pub mod mesh;
pub mod path;
pub mod point;
pub mod region;
pub mod vertex;

use self::bounding_box::BoundingBox;
use self::clipping::Clipping;
use self::mesh::Mesh;
use self::path::Path;
use self::point::Point;
use self::region::Region;

macro_rules! impl_attachment {
    ($t:ident) => {
        impl Base for $t {
            fn name(&self) -> Option<String> {
                let ptr = self.as_raw() as *const _ as *const spAttachment;

                unsafe {
                    ptr.as_ref().and_then(|attach| {
                        attach
                            .name
                            .as_ref()
                            .map(|name| CStr::from_ptr(name).to_string_lossy().to_owned().into())
                    })
                }
            }

            unsafe fn raw_type(&self) -> AttachmentType {
                let ptr = self.as_raw() as *const _ as *const spAttachment;

                (*ptr).type_
            }
        }
    };
}

macro_rules! create_attachment {
    ($t:ident, $raw:ident, $base:ident) => {
        Attachment::$t($t::from_raw($base.cast::<$raw>()))
    };
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
            Attachment::Clipping(ref clipping) => clipping.$m(),
        }
    };
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

impl Attachment {
    pub fn from_raw(raw: NonNull<spAttachment>) -> Self {
        match unsafe { raw.as_ref().type_ } {
            SP_ATTACHMENT_REGION => create_attachment!(Region, spRegionAttachment, raw),
            SP_ATTACHMENT_BOUNDING_BOX => {
                create_attachment!(BoundingBox, spBoundingBoxAttachment, raw)
            }
            SP_ATTACHMENT_MESH | SP_ATTACHMENT_LINKED_MESH => {
                create_attachment!(Mesh, spMeshAttachment, raw)
            }
            SP_ATTACHMENT_PATH => create_attachment!(Path, spPathAttachment, raw),
            SP_ATTACHMENT_POINT => create_attachment!(Point, spPointAttachment, raw),
            SP_ATTACHMENT_CLIPPING => create_attachment!(Clipping, spClippingAttachment, raw),
        }
    }
}

impl Base for Attachment {
    fn name(&self) -> Option<String> {
        attachment_method!(self, name)
    }
    unsafe fn raw_type(&self) -> AttachmentType {
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
    unsafe fn raw_type(&self) -> AttachmentType;
}
