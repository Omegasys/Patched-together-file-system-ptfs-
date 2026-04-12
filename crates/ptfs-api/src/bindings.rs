use crate::Ptfs;

/// Simple C-compatible bindings (expand later)
#[no_mangle]
pub extern "C" fn ptfs_create() -> *mut Ptfs {
    Box::into_raw(Box::new(Ptfs::new()))
}

#[no_mangle]
pub extern "C" fn ptfs_destroy(ptr: *mut Ptfs) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

#[no_mangle]
pub extern "C" fn ptfs_mount(ptr: *mut Ptfs) {
    if let Some(fs) = unsafe { ptr.as_mut() } {
        fs.mount();
    }
}
