use std::ffi;
use std::sync::{Arc, Mutex};

use crate::Source;
use crate::{
    sys::{NDIlib_routing_create_t, NDIlib_routing_instance_type},
    NDIError, NDIResult, HANDLE,
};

struct RouteInstanceInner(*mut NDIlib_routing_instance_type);
unsafe impl Send for RouteInstanceInner {}

impl Drop for RouteInstanceInner {
    fn drop(&mut self) {
        if let Some(destroy_fn) = unsafe { (*HANDLE.lib).routing_destroy } {
            unsafe { destroy_fn(self.0) };
        }
    }
}

pub struct RouteInstance {
    inner: Arc<Mutex<RouteInstanceInner>>,
}

impl RouteInstance {
    pub fn create(name: &str, groups: &[&str]) -> NDIResult<Self> {
        let ndi_name = ffi::CString::new(name).unwrap();
        let groups = ffi::CString::new(groups.join(",")).unwrap();
        let settings = NDIlib_routing_create_t {
            p_ndi_name: ndi_name.as_ptr(),
            p_groups: groups.as_ptr(),
        };

        let Some(create_fn) = (unsafe { (*HANDLE.lib).routing_create }) else {
            return Err(NDIError::MissingSymbolV5("routing_create"));
        };
        let create_t = unsafe { create_fn(&settings).as_mut() };
        let inner = RouteInstanceInner(
            create_t.ok_or_else(|| NDIError::UnexpectedNullPointer("routing_create"))?,
        );
        Ok(Self {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn change(&self, source: &Source) -> NDIResult<()> {
        let Some(change_fn) = (unsafe { (*HANDLE.lib).routing_change }) else {
            return Err(NDIError::MissingSymbolV5("routing_change"));
        };

        let guard = self.inner.lock().expect("Expected Mutex to work");
        let _undefined = source.with_raw(|src| unsafe { change_fn((*guard).0, src) });
        Ok(())
    }

    pub fn clear(&self) -> NDIResult<()> {
        let Some(clear_fn) = (unsafe { (*HANDLE.lib).routing_clear }) else {
            return Err(NDIError::MissingSymbolV5("routing_clear"));
        };

        let guard = self.inner.lock().expect("Expected Mutex to work");
        let _undefined = unsafe { clear_fn((*guard).0) };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() -> NDIResult<()> {
        let _ri = RouteInstance::create("Router Test 1", &vec!["Public"])?;
        Ok(())
    }
}
