use crate::sys::{NDIlib_source_t, NDIlib_source_t__bindgen_ty_1};
use crate::{NDIError, NDIResult};
use std::ffi;

#[derive(Clone)]
pub struct Source {
    pub ndi_name: String,
    pub url_address: String,
}
unsafe impl Send for Source {}

impl From<&NDIlib_source_t> for Source {
    fn from(inner: &NDIlib_source_t) -> Self {
        Self {
            ndi_name: unsafe { ffi::CStr::from_ptr(inner.p_ndi_name) }
                .to_string_lossy()
                .into_owned(),
            url_address: unsafe { ffi::CStr::from_ptr(inner.__bindgen_anon_1.p_url_address) }
                .to_string_lossy()
                .into_owned(),
        }
    }
}

impl Source {
    pub fn with_raw<T>(&self, closure: impl FnOnce(&NDIlib_source_t) -> T) -> NDIResult<T> {
        let p_ndi_name =
            ffi::CString::new(self.ndi_name.clone()).map_err(|_| NDIError::InvalidCString)?;
        let p_url_address =
            ffi::CString::new(self.url_address.clone()).map_err(|_| NDIError::InvalidCString)?;

        let source = NDIlib_source_t {
            p_ndi_name: p_ndi_name.as_ptr(),
            __bindgen_anon_1: NDIlib_source_t__bindgen_ty_1 {
                p_url_address: p_url_address.as_ptr(),
            },
        };
        Ok(closure(&source))
    }
}
