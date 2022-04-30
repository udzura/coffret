extern crate libc;
extern crate rb_sys;

use rb_sys::{rb_cObject, rb_define_class, RubyValue};
use std::ffi::CString;

pub mod class {
    use super::*;
    pub fn object_class() -> RubyValue {
        unsafe { rb_cObject }
    }

    pub fn define_class(
        name: &'static str,
        super_: RubyValue,
    ) -> Result<RubyValue, Box<dyn std::error::Error>> {
        let name = CString::new(name)?;
        let klass = unsafe { rb_define_class(name.as_ptr(), super_) };
        Ok(klass)
    }
}
