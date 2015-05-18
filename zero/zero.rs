//override system abort to avoid pulling in memory allocation dependencies
extern crate core;

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop{}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {
    loop{}
}

#[no_mangle]
pub extern "C" fn abort() {
    loop{}
}

//#[lang="stack_exhausted"]
//extern fn stack_exhausted() {}

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(_: core::fmt::Arguments,
                                _: &'static str, _: usize) -> ! {
    loop{}
}

