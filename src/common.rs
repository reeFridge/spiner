pub trait AsPtr<T> {
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

macro_rules! impl_as_ptr {
    ($s:ident, $raw_type:ident) => {
        impl AsPtr<$raw_type> for $s {
            fn as_ptr(&self) -> *const $raw_type {
                self.raw_ptr as *const $raw_type
            }

            fn as_mut_ptr(&mut self) -> *mut $raw_type {
                self.raw_ptr as *mut $raw_type
            }
        }
    }
}

pub unsafe fn from_raw_buf<T: Copy>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);
    dst.set_len(elts);
    ptr.copy_to(dst.as_mut_ptr(), elts);
    dst
}