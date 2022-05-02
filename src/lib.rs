extern crate libc;
extern crate rb_sys;

pub mod class {
    use rb_sys::{rb_define_class, rb_define_method, RubyValue, VALUE};
    use std::{any::Any, ffi::CString};

    pub fn make_callback(func: &dyn Any) -> unsafe extern "C" fn() -> RubyValue {
        if let Some(func) = func.downcast_ref::<unsafe extern "C" fn() -> VALUE>() {
            return *func;
        }

        if let Some(func) = func.downcast_ref::<unsafe extern "C" fn(VALUE) -> VALUE>() {
            return unsafe {
                std::mem::transmute::<
                    unsafe extern "C" fn(VALUE) -> VALUE,
                    unsafe extern "C" fn() -> VALUE,
                >(*func)
            };
        }

        unreachable!("must pass callback C function")
    }

    pub fn object_class() -> RubyValue {
        unsafe { rb_sys::rb_cObject }
    }

    pub fn define_class(name: &str, super_: RubyValue) -> RubyValue {
        let name = CString::new(name).unwrap();
        let klass = unsafe { rb_define_class(name.as_ptr(), super_) };
        klass
    }

    pub fn define_method(
        klass: RubyValue,
        method: &str,
        callback: unsafe extern "C" fn() -> RubyValue,
        arity: i32,
    ) {
        let method = CString::new(method).unwrap();
        unsafe { rb_define_method(klass, method.as_ptr(), Some(callback), arity) }
    }
}

pub mod exception {
    use rb_sys::{rb_raise, RubyValue};
    use std::ffi::CString;

    pub fn root_class() -> RubyValue {
        unsafe { rb_sys::rb_eException }
    }

    pub fn name_error() -> RubyValue {
        unsafe { rb_sys::rb_eNameError }
    }

    pub fn argument_error() -> RubyValue {
        unsafe { rb_sys::rb_eArgError }
    }

    pub fn runtime_error() -> RubyValue {
        unsafe { rb_sys::rb_eRuntimeError }
    }
    // and so on...

    pub fn rustly_raise(exc: impl std::error::Error) -> ! {
        let s = format!("{}", exc);
        let s = CString::new(s).unwrap();
        unsafe {
            rb_raise(runtime_error(), (&s).as_ptr());
        }
        unreachable!("Calling Kernel#raise failed")
    }

    pub fn raise(exc: RubyValue, fmt: &str) -> ! {
        unsafe {
            let fmt = CString::new(fmt).unwrap();
            rb_raise(exc, fmt.as_ptr());
        }
        unreachable!("Calling Kernel#raise failed")
    }
}
