#[link(name = "prob0087", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::hashmap::{HashSet};
use common::prime;

pub static EXPECTED_ANSWER: &'static str = "1097343";

pub fn solve() -> ~str {
    let limit = 50000000;

    let mut cnt = 0u;
    let mut set = HashSet::with_capacity(2000000);

    for prime::each |p| {
        let p4 = p * p * p * p;
        if p4 >= limit { break; }
        for prime::each |q| {
            let q3 = q * q * q;
            if p4 + q3 >= limit { break; }
            for prime::each |r| {
                let r2 = r * r;
                let s = p4 + q3 + r2;
                if s >= limit { break; }
                if set.contains(&s) { loop; }
                set.insert(s);
                cnt += 1;
            }
        }
    }
    
    return cnt.to_str();
}