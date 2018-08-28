use super::page::Page;
use libspine_sys::*;
use raw::*;

pub struct Region {
    raw: NonNull<spAtlasRegion>,
}

impl_as_raw!(Region, raw, spAtlasRegion);
impl_as_raw_mut!(Region, raw);

impl Region {
    pub fn from_raw(raw: NonNull<spAtlasRegion>) -> Self {
        Region {
            raw
        }
    }

    pub fn page(&self) -> Option<Page> {
        NonNull::new(self.as_raw().page).map(|raw| Page::from_raw(raw))
    }
}
