pub unsafe fn from_raw_buf<T: Copy>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);
    dst.set_len(elts);
    ptr.copy_to(dst.as_mut_ptr(), elts);
    dst
}
