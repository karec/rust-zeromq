//! # Rust zeromq binding
//!
//!
extern crate libc;

pub mod ffi;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
