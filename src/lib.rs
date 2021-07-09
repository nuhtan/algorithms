#![warn(missing_docs)]
//! # Algorithms from *Introduction to Algorithms*
//! Text authored by Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, Clifford Stein.
//!
//! This is based on the third edition.
//!
//! Chapters one and three are skipped as there are no specific algorithms that are detailed within them.


pub mod chapter2;
pub mod chapter4;
pub mod chapter6;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
