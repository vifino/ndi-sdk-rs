use crate::sys::NDIlib_source_t;
use crate::NDIResult;
use std::ffi;

#[derive(Clone)]
pub struct Source<'a> {
    pub(crate) inner: &'a NDIlib_source_t,
}
unsafe impl<'a> Send for Source<'a> {}

impl<'a> Source<'a> {
    pub(crate) fn from_c(inner: &'a NDIlib_source_t) -> NDIResult<Self> {
        Ok(Self { inner })
    }

    pub fn get_ndi_name(&self) -> NDIResult<String> {
        Ok(unsafe { ffi::CStr::from_ptr(self.inner.p_ndi_name) }
            .to_str()?
            .to_owned())
    }

    pub fn get_url_address(&self) -> NDIResult<String> {
        Ok(
            unsafe { ffi::CStr::from_ptr(self.inner.__bindgen_anon_1.p_url_address) }
                .to_str()?
                .to_owned(),
        )
    }
}
