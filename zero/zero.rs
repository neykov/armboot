// Copyright (c) 2006-2009 Graydon Hoare
// Copyright (c) 2009-2013 Mozilla Foundation

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//
// zero.rs
//
// Minimal definitions of the core primitives in Rust. Include this file with
// your project to create a freestanding Rust program that can run on bare
// metal.
//

#[allow(ctypes)];

// Built-in traits

#[lang="copy"]
pub trait Copy {}

#[lang="send"]
pub trait Owned {}

#[lang="sized"]
pub trait Sized {}

#[lang="freeze"]
pub trait Freeze {}


 

#[lang="drop"]
pub trait Drop {
    fn finalize(&self);
}

// Operator overloading

#[lang="eq"]
pub trait Eq {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
}

#[lang="ord"]
pub trait Ord {
    fn lt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
}

#[lang="add"]
pub trait Add<Rhs,Result> {
    fn add(&self, rhs: &Rhs) -> Result;
}

#[lang="sub"]
pub trait Sub<Rhs,Result> {
    fn sub(&self, rhs: &Rhs) -> Result;
}

#[lang="mul"]
pub trait Mul<Rhs,Result> {
    fn mul(&self, rhs: &Rhs) -> Result;
}

#[lang="div"]
pub trait Div<Rhs,Result> {
    fn div(&self, rhs: &Rhs) -> Result;
}

#[lang="rem"]
pub trait Rem<Rhs,Result> {
    fn rem(&self, rhs: &Rhs) -> Result;
}

#[lang="neg"]
pub trait Neg<Rhs,Result> {
    fn neg(&self) -> Result;
}

#[lang="not"]
pub trait Not<Rhs,Result> {
    fn not(&self) -> Result;
}

#[lang="bitand"]
pub trait BitAnd<Rhs,Result> {
    fn bitand(&self, rhs: &Rhs) -> Result;
}

#[lang="bitor"]
pub trait BitOr<Rhs,Result> {
    fn bitor(&self, rhs: &Rhs) -> Result;
}

#[lang="bitxor"]
pub trait BitXor<Rhs,Result> {
    fn bitxor(&self, rhs: &Rhs) -> Result;
}

#[lang="shl"]
pub trait Shl<Rhs,Result> {
    fn shl(&self, rhs: &Rhs) -> Result;
}

#[lang="shr"]
pub trait Shr<Rhs,Result> {
    fn shr(&self, rhs: &Rhs) -> Result;
}

#[lang="index"]
pub trait Index<Index,Result> {
    fn index(&self, rhs: &Index) -> Result;
}

pub type GlueFn = extern "Rust" fn(*i8);

#[lang="ty_desc"]
pub struct TyDesc {
    size: uint,
    align: uint,
    take_glue: GlueFn,
    drop_glue: GlueFn,
    free_glue: GlueFn,
    visit_glue: GlueFn,
}

#[lang="opaque"]
pub enum Opaque { }

#[lang="ty_visitor"]
pub trait TyVisitor {
    fn visit_bot(&self) -> bool;
    fn visit_nil(&self) -> bool;
    fn visit_bool(&self) -> bool;

    fn visit_int(&self) -> bool;
    fn visit_i8(&self) -> bool;
    fn visit_i16(&self) -> bool;
    fn visit_i32(&self) -> bool;
    fn visit_i64(&self) -> bool;

    fn visit_uint(&self) -> bool;
    fn visit_u8(&self) -> bool;
    fn visit_u16(&self) -> bool;
    fn visit_u32(&self) -> bool;
    fn visit_u64(&self) -> bool;

    fn visit_float(&self) -> bool;
    fn visit_f32(&self) -> bool;
    fn visit_f64(&self) -> bool;

    fn visit_char(&self) -> bool;
    fn visit_str(&self) -> bool;

    fn visit_estr_box(&self) -> bool;
    fn visit_estr_uniq(&self) -> bool;
    fn visit_estr_slice(&self) -> bool;
    fn visit_estr_fixed(&self, n: uint, sz: uint, align: uint) -> bool;

    fn visit_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_ptr(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_rptr(&self, mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_unboxed_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_slice(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_fixed(&self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_enter_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_rec_field(&self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;
    fn visit_class_field(&self, i: uint, name: &str,
                         mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;

    fn visit_enter_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_tup_field(&self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_enum(&self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;
    fn visit_enter_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_enum_variant_field(&self, i: uint, offset: uint, inner: *TyDesc) -> bool;
    fn visit_leave_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_leave_enum(&self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;

    fn visit_enter_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;
    fn visit_fn_input(&self, i: uint, mode: uint, inner: *TyDesc) -> bool;
    fn visit_fn_output(&self, retstyle: uint, inner: *TyDesc) -> bool;
    fn visit_leave_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;

    fn visit_trait(&self) -> bool;
    fn visit_var(&self) -> bool;
    fn visit_var_integral(&self) -> bool;
    fn visit_param(&self, i: uint) -> bool;
    fn visit_self(&self) -> bool;
    fn visit_type(&self) -> bool;
    fn visit_opaque_box(&self) -> bool;
    fn visit_constr(&self, inner: *TyDesc) -> bool;
    fn visit_closure_ptr(&self, ck: uint) -> bool;
}



// String utilities

#[lang="str_eq"]
pub fn str_eq(a: &str, b: &str) -> bool {
    unsafe {
        let (aptr, alen): (*u8, uint) = transmute(a);
        let (bptr, blen): (*u8, uint) = transmute(b);
        if alen != blen {
            return false
        }
        memcmp(aptr, bptr, alen - 1) == 0
    }
}

// FIXME(pcwalton): This function is legacy junk.
#[lang="uniq_str_eq"]
pub fn uniq_str_eq(a: &~str, b: &~str) -> bool {
    str_eq(*a, *b)
}

struct StringRepr {
    fill: uint,
    alloc: uint,
}

// FIXME(pcwalton): This function should not be necessary, I don't think.
#[lang="strdup_uniq"]
pub unsafe fn strdup_uniq(ptr: *u8, len: uint) -> ~str {
    let size = size_of::<StringRepr>() + len + 1;
    let string: *mut StringRepr = transmute(exchange_malloc(transmute(0),
                                                            size));
    (*string).fill = len + 1;
    (*string).alloc = len + 1;

    let mut data_ptr: uint = transmute(string);
    data_ptr += size_of::<StringRepr>();
    let data_ptr: *mut u8 = transmute(data_ptr);
    memcpy(data_ptr, ptr, len + 1);

    transmute(string)
}

// Legacy junk

#[lang="log_type"]
pub fn log_type<T>(_: u32, _: &T) {
    // FIXME: This function should not be a lang item.
}

#[lang="annihilate"]
pub unsafe fn annihilate() {}

// Failure

#[lang="fail_"]
pub fn fail(_: *i8, _: *i8, _: uint) -> ! {
    unsafe {
        abort()
    }
}

#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_: *i8, _: uint, _: uint, _: uint) {
    unsafe {
        abort()
    }
}

// Memory allocation

// FIXME: So grotesquely inefficient.
struct Header {
    minus_one: uint,    // Must be -1.
    type_desc: *i8,
    null_0: uint,       // Must be null.
    null_1: uint,       // Must be null.
}

// FIXME: This is horrendously inefficient.
#[lang="exchange_malloc"]
pub unsafe fn exchange_malloc(type_desc: *i8, size: uint) -> *i8 {
    let alloc: *mut Header = transmute(malloc(size_of::<Header>() + size));
    (*alloc).minus_one = -1;
    (*alloc).type_desc = type_desc;
    (*alloc).null_0 = 0;
    (*alloc).null_1 = 0;
    transmute(alloc)
}

#[lang="closure_exchange_malloc"]
pub unsafe fn closure_exchange_malloc(_: *i8, _: uint) -> *i8 {
    abort()
}

#[lang="exchange_free"]
pub unsafe fn exchange_free(alloc: *i8) {
    free(transmute(alloc))
}

// Entry point

// TODO(pcwalton): Stash argc and argv somewhere. Probably needs to wait on
// global variables.
#[lang="start"]
pub fn start(main: *u8, _: int, _: **i8, _: *u8) -> int {
    unsafe {
        let main: extern "Rust" fn() = transmute(main);
        main();
        0
    }
}

// The nonexistent garbage collector

#[lang="malloc"]
pub unsafe fn gc_malloc(_: *i8, _: uint) -> *i8 {
    abort()
}

#[lang="free"]
pub unsafe fn gc_free(_: *i8) {
    abort()
}

#[lang="borrow_as_imm"]
pub unsafe fn borrow_as_imm(_: *u8, _: *i8, _: uint) -> uint {
    abort()
}

#[lang="borrow_as_mut"]
pub unsafe fn borrow_as_mut(_: *u8, _: *i8, _: uint) -> uint {
    abort()
}

#[lang="record_borrow"]
pub unsafe fn record_borrow(_: *u8, _: uint, _: *i8, _: uint) {
    abort()
}

#[lang="unrecord_borrow"]
pub unsafe fn unrecord_borrow(_: *u8, _: uint, _: *i8, _: uint) {
    abort()
}

#[lang="return_to_mut"]
pub unsafe fn return_to_mut(_: *u8, _: uint, _: *i8, _: uint) {
    abort()
}

#[lang="check_not_borrowed"]
pub unsafe fn check_not_borrowed(_: *u8, _: *i8, _: uint) {
    abort()
}

// libc dependencies

extern {
    #[fast_ffi]
    pub fn malloc(size: uint) -> *u8;
    #[fast_ffi]
    pub fn free(ptr: *u8);
    #[fast_ffi]
    pub fn abort() -> !;
    #[fast_ffi]
    pub fn memcpy(dest: *mut u8, src: *u8, size: uint) -> *u8;
    #[fast_ffi]
    pub fn memcmp(a: *u8, b: *u8, size: uint) -> i32;
}

// Rust intrinsic dependencies

extern "rust-intrinsic" {
    pub fn transmute<T,U>(val: T) -> U;
    pub fn size_of<T>() -> uint;
}

