#[link(name = "prob0055", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;

use std::str;
use std::from_str::{FromStr};
use extra::bigint::{BigUint};

pub static EXPECTED_ANSWER: &'static str = "249";

fn reverse(n: &BigUint) -> BigUint {
    let s = n.to_str();
    let rev = str::from_chars(s.rev_iter().collect::<~[char]>());
    return FromStr::from_str(rev).get();
}

fn is_lychrel(n: uint) -> bool {
    let n = BigUint::from_uint(n);
    let mut sum = n + reverse(&n);
    foreach _ in range(0, 50) {
        let rev_sum = reverse(&sum);
        if rev_sum == sum { return false; }
        sum = sum + rev_sum;
    }
    return true;
}

pub fn solve() -> ~str {
    let mut cnt = 0u;
    foreach n in range(1u, 10001) {
        if is_lychrel(n) { cnt += 1; }
    }
    return cnt.to_str();
}