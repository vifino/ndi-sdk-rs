use crate::sys::{NDIlib_find_create_t, NDIlib_find_instance_t};
use crate::{NDIError, NDIResult, Source, HANDLE};
use std::net::IpAddr;
use std::ptr;

#[derive(Default, Debug, Clone)]
pub struct FindSettings<'a> {
    show_local_sources: bool,
    groups: Vec<&'a str>,
    extra_ips: Vec<&'a IpAddr>,
}

impl<'a> FindSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_local_sources(mut self, show: bool) -> Self {
        self.show_local_sources = show;
        self
    }

    pub fn add_group(mut self, group: &'a str) -> Self {
        self.groups.push(group);
        self
    }

    pub fn add_extra_ip(mut self, addr: &'a IpAddr) -> Self {
        self.extra_ips.push(addr);
        self
    }

    pub fn build(self) -> NDIResult<NDIlib_find_create_t> {
        let groups = self.groups.join(",");
        let ips = self
            .groups
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<_>>()
            .join(",");

        Ok(NDIlib_find_create_t {
            show_local_sources: self.show_local_sources,
            p_groups: if self.groups.is_empty() {
                ptr::null()
            } else {
                groups.as_ptr() as *const i8
            },
            p_extra_ips: if self.extra_ips.is_empty() {
                ptr::null()
            } else {
                ips.as_ptr() as *const i8
            },
        })
    }
}

pub struct FindInstance {
    inner: NDIlib_find_instance_t,
}
unsafe impl Send for FindInstance {}

impl Drop for FindInstance {
    fn drop(&mut self) {
        if let Some(destroy_fn) = unsafe { (*HANDLE.lib).find_destroy } {
            unsafe { destroy_fn(self.inner) };
        }
    }
}

impl FindInstance {
    pub fn create(settings: Option<&NDIlib_find_create_t>) -> NDIResult<FindInstance> {
        let create = match settings {
            Some(settings) => settings,
            None => ptr::null(),
        };
        let Some(create_fn) = (unsafe { (*HANDLE.lib).find_create_v2 }) else {
            return Err(NDIError::MissingSymbolV5("find_create_v2"));
        };

        let instance_ptr = unsafe { create_fn(create) };
        if instance_ptr.is_null() {
            return Err(NDIError::UnexpectedNullPointer("find_create_v2"));
        }
        Ok(FindInstance {
            inner: instance_ptr,
        })
    }

    pub fn get_current_sources(&mut self) -> NDIResult<Vec<Source>> {
        let Some(get_current_fn) = (unsafe { (*HANDLE.lib).find_get_current_sources }) else {
            return Err(NDIError::MissingSymbolV5("find_get_current_sources"));
        };

        let mut num_sources: u32 = 0;
        let sources_ptr = unsafe { get_current_fn(self.inner, &mut num_sources) };
        if num_sources == 0 {
            return Ok(Vec::new());
        }
        if sources_ptr.is_null() {
            return Err(NDIError::UnexpectedNullPointer("find_get_current_sources"));
        }
        let mut sources = Vec::new();
        for idx in 0..num_sources as usize {
            let source = unsafe { &*sources_ptr.add(idx) };
            sources.push(Source::from_c(source)?);
        }
        Ok(sources)
    }

    pub fn wait_for_sources(&mut self, timeout_ms: u32) -> NDIResult<bool> {
        let Some(wait_fn) = (unsafe { (*HANDLE.lib).find_wait_for_sources }) else {
            return Err(NDIError::MissingSymbolV5("find_wait_for_sources"));
        };
        Ok(unsafe { wait_fn(self.inner, timeout_ms) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() -> NDIResult<()> {
        let settings = FindSettings::new().show_local_sources(true).build()?;
        let mut inst = FindInstance::create(Some(&settings))?;
        let mut desired_sources = Vec::new();
        for _ in 0..3 {
            if !inst.wait_for_sources(5000)? {
                println!("No sources found!");
            } else {
                let sources = inst.get_current_sources()?;
                println!("Number of NDI Sources: {}", sources.len());
                for (idx, source) in sources.iter().enumerate() {
                    desired_sources.push(source);
                    println!("\tSource index: {}", idx);
                    println!("\t\tName: {}", source.get_ndi_name()?);
                    println!("\t\tURL:  {}", source.get_url_address()?);
                }
                // We'll just bail out early.
                break;
            }
        }
        Ok(())
    }
}
