pub mod de;
pub mod error;
#[cfg(feature = "pyo3")]
mod py_ffi;
pub mod ser;

pub use de::*;
pub use error::*;
pub use ser::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
