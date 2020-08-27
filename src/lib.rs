#[cfg(feature = "default")]
pub use authentication::*;

#[cfg(feature = "management_api")]
pub use management::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
