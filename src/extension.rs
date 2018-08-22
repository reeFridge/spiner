#[macro_export]
macro_rules! extend_spine {
    ({ _spUtil_readFile -> $read_file:ident, _spAtlasPage_createTexture -> $read_texture:ident}) => {
        pub mod extend_spine {
            use std::ffi::{CString, CStr};
            use std::os::raw::{c_char, c_int, c_void};
            use std::io::{Error, ErrorKind};
            use std::error::Error as StdErr;
            use libc;
            use libspine_sys::spAtlasPage;
            use super::{$read_file, $read_texture};

            impl_read_file!($read_file);
            impl_create_texture!($read_texture);

            #[no_mangle]
            pub extern fn _spAtlasPage_disposeTexture(atlas: *mut spAtlasPage) {
                unsafe {
                    libc::free((*atlas).rendererObject as *mut libc::c_void);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_read_file {
    ($r:ident) => {
        #[no_mangle]
        pub extern fn _spUtil_readFile(path: *const c_char, length: *mut c_int) -> *const c_char {
            let read = || path_str!(path)
                .and_then($r)
                .and_then(|buf| CString::new(buf).map_err(Error::from));
            let c_string = match read() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return 0 as *const c_char;
                }
            };
            let len = c_string.to_bytes().len();

            unsafe {
                *length = len as i32;
                raw_copy!(c_string.as_ptr(), len) as *const c_char
            }
        }
    }
}

#[macro_export]
macro_rules! impl_create_texture {
    ($r:ident) => {
        #[no_mangle]
        pub extern fn _spAtlasPage_createTexture(atlas: *mut spAtlasPage, path: *const c_char) {
            let (buf, (w, h)) = path_str!(path).and_then($r).unwrap();

            unsafe {
                (*atlas).width = w as i32;
                (*atlas).height = h as i32;
                (*atlas).rendererObject = raw_copy!(buf.as_ptr(), buf.len()) as *mut c_void;
            }
        }
    }
}

#[macro_export]
macro_rules! path_str {
    ($n:ident) => {
        (unsafe { CStr::from_ptr($n) })
            .to_str()
            .map_err(|err| Error::new(ErrorKind::Other, err.description()))
    }
}

#[macro_export]
macro_rules! raw_copy {
    ($p:expr, $l:expr) => {
        {
            let out_ptr = libc::malloc($l);
            libc::memcpy(out_ptr, $p as *const libc::c_void, $l);
            out_ptr
        }
    }
}