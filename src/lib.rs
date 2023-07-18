mod ffi {
    #![allow(
        dead_code,
        improper_ctypes,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::{c_char, CStr, CString};

fn to_cstring(s: &str) -> Result<CString, String> {
    CString::new(s).map_err(|_| "Input contained NUL".to_string())
}

pub fn image_inspect(image: &str, creds: Option<(&str, &str)>) -> Result<String, String> {
    let image = to_cstring(image)?;
    let (user, password) = creds.unwrap_or_default();
    let (user, password) = (to_cstring(user)?, to_cstring(password)?);

    let res = unsafe {
        ffi::ImageInspect(
            image.as_ptr() as *mut c_char,
            user.as_ptr() as *mut c_char,
            password.as_ptr() as *mut c_char,
        )
    };

    let error = res.r0 != 0;

    let cstr = unsafe { CStr::from_ptr(res.r1) };
    let data = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    unsafe { ffi::GoFree(res.r1) };

    if error {
        Err(data)
    } else {
        Ok(data)
    }
}
