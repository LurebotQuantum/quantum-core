
use log::{info, debug};
use libloading::{Library, Symbol};
use crate::result::{Result, Error};
use std::ffi::{OsStr, CStr};
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Plugin {
    library: Library,
    name: String,
    data: Option<Arc<Mutex<*mut c_void>>>,
}

impl Plugin {
    pub fn version() -> &'static str {
        "1"
    }

    pub fn load<P: AsRef<OsStr>>(path: P) -> Result<Plugin> {
        info!("Loading plugin from `{}`",
            path.as_ref().to_str().or(Some("undisplayable path")).unwrap());
        let library = Library::new(path)?;

        debug!("Reading meta infomation...");

        let fn_api_version: Symbol<unsafe extern "C" fn() -> *const c_char> =
            unsafe { library.get(b"api_version\0")? };
        let api_version = unsafe { CStr::from_ptr(fn_api_version()).to_str()? };
        if api_version != Plugin::version() {
            return Err(Error::from("API version mismatch!"));
        }

        let fn_name: Symbol<unsafe extern "C" fn() -> *const c_char> =
            unsafe { library.get(b"name\0")? };
        let name = unsafe { CStr::from_ptr(fn_name()).to_str()?.to_owned() };

        debug!("Loading plugin data...");

        let fn_on_load: Symbol<unsafe extern "C" fn() -> *mut c_void> =
            unsafe { library.get(b"on_load\0")? };
        let unsafe_data = unsafe { fn_on_load() };
        let data = Some(Arc::new(Mutex::new(unsafe_data)));

        let plugin = Plugin {
            library,
            name,
            data,
        };

        info!("Plugin loaded: {}", plugin.name());

        Ok(plugin)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        let data = self.data.take();
        if let Some(data) = data {
            let unsafe_data = *data.lock().unwrap();
            type FnOnUnload<'lib> =
                Symbol<'lib, unsafe extern "C" fn(*mut c_void)>;
            let fn_on_unload: std::io::Result<FnOnUnload> =
                unsafe { self.library.get(b"on_unload\0") };
            if let Ok(fn_on_unload) = fn_on_unload {
                unsafe { fn_on_unload(unsafe_data) };
            }
        }
    }
}
