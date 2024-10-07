type LibName = extern "C" fn() -> *const std::ffi::c_char;


#[no_mangle]
extern "C" fn rust_fn() {
    println!("Now we are running in a rust function");
}

macro_rules! dlerror {
    () => {
        std::ffi::CStr::from_ptr(libc::dlerror()).to_str().unwrap()
    };
}

fn main() {
    // prevent the rust compiler from optimizing `rust_fn`
    let p = &0 as *const i32;
    if p.is_null() {
        rust_fn();
    }
    unsafe {
        let dl = libc::dlopen(c"c_lib.so".as_ptr(), libc::RTLD_NOW|libc::RTLD_GLOBAL);
        if dl.is_null() {
            panic!("cannot open c_lib.so. {}", dlerror!());
        }

        let get_name = libc::dlsym(dl, c"lib_name".as_ptr());
        if get_name.is_null() {
            panic!("cannot find lib_name function. {}", dlerror!());
        }
        let get_name: LibName = std::intrinsics::transmute(get_name);
        let name = std::ffi::CStr::from_ptr(get_name());
        println!("name: {}", name.to_str().unwrap());
    }
}
