#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate common;
extern crate prime;

use common::Solver;
use prime::{PrimeSet, Factorize};

fn compute(n: uint) -> uint {
    let ps = PrimeSet::new();
    n.factorize(&ps)
        .map(|(base, _exp)| base)
        .max()
        .unwrap()
}

fn solve() -> String { compute(600851475143).to_string() }
fn main() { Solver::new("6857", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn factorize_13195() {
        assert_eq!(29, super::compute(13195));
    }
}