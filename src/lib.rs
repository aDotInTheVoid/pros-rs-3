#![no_std]

mod raw {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    use cortex_types;

    include!(concat!(env!("OUT_DIR"), "/pros_raw_gen.rs"));
}
