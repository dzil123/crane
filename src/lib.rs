use std::ffi::{c_char, CStr, CString};
use std::sync::OnceLock;

mod ffi;

fn to_cstring(s: &str) -> Result<CString, String> {
    CString::new(s).map_err(|_| "Input contained NUL".to_string())
}

unsafe fn to_string(cstr: *mut std::os::raw::c_char) -> String {
    if cstr.is_null() {
        return "".into();
    }

    let cstr = unsafe { CStr::from_ptr(cstr) };
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

pub fn get_buildinfo() -> &'static str {
    static BUILD_INFO: OnceLock<String> = OnceLock::new();

    BUILD_INFO.get_or_init(|| unsafe {
        let cstr = ffi::GetBuildInfo();
        let ret = to_string(cstr);
        ffi::FreeStr(cstr);
        ret
    })
}

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub config: String,
    pub digest: String,
    pub manifest: String,
}

pub fn image_metadata(image: &str, creds: Option<(&str, &str)>) -> Result<ImageMetadata, String> {
    let image = to_cstring(image)?;
    let (user, password) = creds.unwrap_or_default();
    let (user, password) = (to_cstring(user)?, to_cstring(password)?);

    let ret = unsafe {
        ffi::ImageMetadata(
            image.as_ptr() as *mut c_char,
            user.as_ptr() as *mut c_char,
            password.as_ptr() as *mut c_char,
        )
    };

    let error = unsafe { to_string(ret.error) };
    if !error.is_empty() {
        unsafe { ffi::FreeImageMetadataReturn(ret) };

        return Err(error);
    }

    let config = unsafe { to_string(ret.config) };
    let digest = unsafe { to_string(ret.digest) };
    let manifest = unsafe { to_string(ret.manifest) };

    unsafe { ffi::FreeImageMetadataReturn(ret) };

    Ok(ImageMetadata {
        config,
        digest,
        manifest,
    })
}

#[test]
fn test_buildinfo() {
    assert!(!get_buildinfo().is_empty());
}
