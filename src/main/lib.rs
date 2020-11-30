/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */

mod cshadow {
    // Inline the bindgen-generated Rust bindings, suppressing warnings.
    #![allow(unused)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("bindings/rust/wrapper.rs");
}

pub mod core;
pub mod host;
pub mod routing;
pub mod shmem;
pub mod utility;
