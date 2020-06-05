#![allow(non_camel_case_types)]
#![no_std]

//! Binding of rust types to c types for the cortex A9 processor

pub use core::ffi::c_void;

#[cfg(target_arch = "arm")]
mod cortex_types {
    // Prity sure these are right.
    // https://github.com/aDotInTheVoid/pros-rs/blob/master/cortex_a9_types/src/lib.rs

    pub type c_double = f64;
    pub type c_float = f32;
    pub type c_char = u8;
    pub type c_int = i32;
    pub type c_long = i32;
    pub type c_longlong = i64;
    pub type c_schar = i8; 
    pub type c_short = i16; 
    pub type c_uchar = u8; 
    pub type c_ulong = u32; 
    pub type c_ulonglong = u64; 
    pub type c_ushort = u16; 
    pub type c_uint = u32;

}

// Major hack to allow x86 tests to work
#[cfg(target_arch = "arm")]
pub use cortex_types::*;

#[cfg(not(target_arch = "arm"))]
pub use libc::*;