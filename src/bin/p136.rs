//! [Problem 136](https://projecteuler.net/problem=136) solver.
//!
//! Using the same algorithm as p135.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;

use std::iter;

fn num_solutions(limit: uint) -> Vec<u32> {
    let mut cnt = Vec::from_elem(limit, 0);
    for q in range(1, limit) {
        let r = (4 - (q % 4)) % 4;
        if q * r >= limit { continue }
        for p in iter::range_step(r, q * 3, 4) {
            let n = q * p;
            if n >= limit { break }
            cnt[n] += 1;
        }
    }
    cnt
}

fn solve() -> String {
    let limit = 50000000;
    let cnt = 1;
    num_solutions(limit)
        .iter()
        .filter(|&&n| n == cnt)
        .count()
        .to_string()
}

problem!("2544559", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn first_sol() {
        let pos = super::num_solutions(2000)
            .iter()
            .position(|&n| n == 10)
            .unwrap();
        assert_eq!(1155, pos);
    }
}