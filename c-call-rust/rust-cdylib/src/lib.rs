
extern "C" {
    /// 定义在 C 里的函数
    fn c_fn();
    fn c_foo();
}

#[no_mangle]
extern "C" fn lib_name() -> *const std::ffi::c_char {
   unsafe { c_fn(); }
    c"rust_lib".as_ptr()
}
