#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate integer;
extern crate iter;

use common::Solver;
use integer::Integer;
use iter::Permutations;

fn compute() -> String {
    let nums = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let radix = 10;

    let mut ans = 0;
    for (ds, rest) in Permutations::new(nums, 4) {
        let n: uint = Integer::from_digits(ds.iter().rev().map(|&x| x), radix);
        let mut ds2 = (n * 2).into_digits(radix).collect::<Vec<_>>();
        ds2.sort_by(|a, b| b.cmp(a));

        if ds2 == rest {
            ans = n;
            break;
        }
    }
    format!("{}{}", ans, ans * 2)
}

fn solve() -> String {
    compute()
}

fn main() { Solver::new("932718654", solve).run(); }
