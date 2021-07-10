#![warn(missing_docs)]
//! # Algorithms from *Introduction to Algorithms*
//! Text authored by Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, Clifford Stein.
//!
//! This is based on the third edition.
//!
//! Some chapters do not have code as they discuss topics that don't deal with specific code examples and cover more conceptual concepts.

pub mod chapter2;
pub mod chapter4;
pub mod chapter6;
pub mod chapter7;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
