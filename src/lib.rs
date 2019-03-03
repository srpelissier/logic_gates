#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
//! # Demo example crate for Chapter 3
//! This a logic gates simulation crate built to demonstrate writing unit tests, integration tests, documentation tests.

/// Implements a boolean `and` gate.
/// Input is two bits, output is one bit.
pub fn and(a: u8, b: u8) -> u8 {
   match (a, b) {
      (1, 1) => 1,
      _ => 0,
   }
}
/// Implements a boolean `xor` gate.
/// Input is two bits, output is one bit.
pub fn xor(a: u8, b: u8) -> u8 {
   match (a, b) {
      (0, 1) | (1, 0) => 1,
      _ => 0,
   }
}

#[cfg(test)]
mod tests {
   use crate::{and, xor};
   #[test]
   fn and_test() {
      assert!(and(1, 1) == 1);
      assert!(and(0, 1) == 0);
      assert!(and(1, 0) == 0);
      assert!(and(0, 0) == 0);
   }
   #[test]
   fn xor_test() {
      assert!(xor(1, 1) == 0);
      assert!(xor(0, 1) == 1);
      assert!(xor(1, 0) == 1);
      assert!(xor(0, 0) == 0);
   }
}
