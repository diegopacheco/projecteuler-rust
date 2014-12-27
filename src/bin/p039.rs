//! [Problem 39](https://projecteuler.net/problem=39) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate integer;
extern crate seq;

use std::collections::HashMap;
use integer::Integer;
use seq::PrimitivePythagoreans;

fn num_sum_pythagorean(limit: uint) -> HashMap<uint, uint> {
    let mut map = HashMap::<uint, uint>::new();

    for m in range(1, ((1 + limit).sqrt() - 1) / 2) {
        for (a, b, c) in PrimitivePythagoreans::new(m) {
            let s = a + b + c;
            for k in range(1, limit / s + 1) {
                let new_val = map.get(&(k * s)).map_or(1, |&v| v + 1);
                let _ = map.insert(k * s, new_val);
            }
        }
    }

    map
}

fn compute(limit: uint) -> uint {
    let map = num_sum_pythagorean(limit);
    let (max_key, _max_val) = map.iter().max_by(|&(&_k, &v)| v).unwrap();
    *max_key
}

fn solve() -> String {
    compute(1000).to_string()
}

problem!("840", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn limit_120() {
        let map = super::num_sum_pythagorean(120);
        assert_eq!(3, *map.get(&120).unwrap());
    }
}
