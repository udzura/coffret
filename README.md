# Coffret

A fancy wrapper of MRI C API.

Aiming to be [`nix`](https://github.com/nix-rust/nix) for Ruby gem development.

## Example

### Before

```rust
use rb_sys::*;

fn test_show_self() // ...

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_hi_rust() {
    let name = CString::new("Rust").unwrap();
    let object = unsafe { rb_cObject };
    let klass = unsafe { rb_define_class(name.as_ptr(), object) };

    let function_name = CString::new("mostrarme").unwrap();
    let callback = unsafe {
        std::mem::transmute::<unsafe extern "C" fn(VALUE) -> VALUE, unsafe extern "C" fn() -> VALUE>(
            test_show_self,
        )
    };

    unsafe { rb_define_method(klass, function_name.as_ptr(), Some(callback), 0) }
}
```

### After

```rust
use coffret::class;
use coffret::exception;

fn test_show_self() // ...

fn init_hi_rust_internal() -> Result<(), Box<dyn Error>> {
    let object = class::object_class();
    let klass = class::define_class("Rust", object);

    let callback = class::make_callback(&test_show_self);

    class::define_method(klass, "mostrarme", callback, 0);
    Ok(())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_hi_rust() {
    match init_hi_rust_internal() {
        // Rust Error to Ruby's Exception. Isn't it cool?
        Err(e) => exception::rustly_raise(e.as_ref()),
        Ok(_) => {}
    }
}
```

MRI has bunch of APIs, so it is now under construction && any P/Rs are welcomed.

## License

MIT.
