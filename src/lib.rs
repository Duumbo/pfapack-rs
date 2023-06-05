extern crate pfapack_sys as ffi;
extern crate libc;
extern crate num_complex as num;

use libc::c_char;

/// A complex number with 32-bit parts
#[allow(non_camel_case_types)]
pub type c32 = num::Complex<f32>;

/// A complex number with 64-bit parts
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;

pub type Select2F32 = Option<extern "C" fn(*const f32, *const f32) -> i32>;
pub type Select3F32 = Option<extern "C" fn(*const f32, *const f32, *const f32) -> i32>;

pub type Select2F64 = Option<extern "C" fn(*const f64, *const f64) -> i32>;
pub type Select3F64 = Option<extern "C" fn(*const f64, *const f64, *const f64) -> i32>;

pub type Select1C32 = Option<extern "C" fn(*const c32) -> i32>;
pub type Select2C32 = Option<extern "C" fn(*const c32, *const c32) -> i32>;

pub type Select1C64 = Option<extern "C" fn(*const c64) -> i32>;
pub type Select2C64 = Option<extern "C" fn(*const c64, *const c64) -> i32>;

include!("pfapack-bind.rs");
