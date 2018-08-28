pub use std::io::{Error, ErrorKind};
pub use std::ptr::NonNull;

pub unsafe trait AsRaw {
    type Raw;
    fn as_raw(&self) -> &Self::Raw;
}

unsafe impl<'a, T> AsRaw for &'a T
where
    T: 'a + AsRaw,
{
    type Raw = <T as AsRaw>::Raw;
    fn as_raw(&self) -> &Self::Raw {
        (*self).as_raw()
    }
}

pub unsafe trait AsRawMut: AsRaw {
    fn as_raw_mut(&mut self) -> *mut <Self as AsRaw>::Raw;
}

macro_rules! try_wrap {
    ($ptr:ident, $cb:expr) => {
        match NonNull::new($ptr) {
            Some(raw) => Ok($cb(raw)),
            None => Err(Error::new(ErrorKind::Other, "Nul"))
        }
    }
}

macro_rules! impl_as_raw {
    ($struct:ident, $field:ident, $raw:ident) => {
        unsafe impl AsRaw for $struct {
            type Raw = $raw;
            fn as_raw(&self) -> &Self::Raw {
                unsafe { self.$field.as_ref() }
            }
        }
    };
}

macro_rules! impl_as_raw_mut {
    ($struct:ident, $field:ident) => {
        unsafe impl AsRawMut for $struct {
            fn as_raw_mut(&mut self) -> *mut <Self as AsRaw>::Raw {
                unsafe { self.$field.as_mut() }
            }
        }
    };
}
