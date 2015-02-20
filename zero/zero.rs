#[lang="fail_bounds_check"]
fn fail_bounds_check(_: &(&'static str, uint),
                     _: uint, _: uint) -> ! {
    loop{}
}

//override system abort to avoid pulling in memory allocation dependencies
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop{}
}


#[cold] #[inline(never)]
#[lang="panic_bounds_check"]
fn panic_bounds_check(_: &(&'static str, uint),
                     _: uint, _: uint) -> ! {
    loop{}
}

#[lang="phantom_fn"]
pub trait PhantomFn<A:?Sized,R:?Sized=()> { }
pub trait MarkerTrait : PhantomFn<Self> { }

#[lang = "sized"]
pub trait Sized : MarkerTrait {}

#[lang = "copy"]
pub trait Copy : MarkerTrait {}

#[lang="sync"]
pub trait Sync : MarkerTrait {}

