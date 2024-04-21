#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use bindgen::{Yamm,YammBuffer};


mod bindgen {

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub type YammBuffer = yamm_ns_yamm_buffer;
    pub type Yamm = yamm_ns_yamm;

}