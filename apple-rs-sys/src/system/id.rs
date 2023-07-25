use std::ffi::c_void;

use crate::utils::objective::*;


pub struct ObjcId(pub *mut runtime::Object);

impl ObjcId {
    pub fn new(ptr: *mut runtime::Object) -> Self {
        ObjcId(ptr)
    }
}

impl From<ObjcId> for *mut runtime::Object {
    fn from(obj: ObjcId) -> Self {
        obj.0
    }
}

impl Drop for ObjcId {
    fn drop(&mut self) {
        unsafe {
            let _: *mut c_void = msg_send![self.0, release];
        }
    }
}