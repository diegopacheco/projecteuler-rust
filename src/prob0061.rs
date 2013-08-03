#[link(name = "prob0061", vers = "0.0")];
#[crate_type = "lib"];


use std::vec;
use std::iterator::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "28684";

fn create_map(f: &fn(uint) -> uint) -> ~[~[uint]] {
    let mut result = vec::from_elem(100, ~[]);
    let mut i = 1;
    loop {
        let n = f(i);
        if n >= 10000 { break; }
        result[n / 100].push(n % 100);
        i += 1;
    }
    return result;
}

pub fn solve() -> ~str {
    let map = ~[
        create_map(|n| n * (n + 1) / 2),
        create_map(|n| n * n),
        create_map(|n| n * (3 * n - 1) / 2),
        create_map(|n| n * (2 * n - 1)),
        create_map(|n| n * (5 * n - 3) / 2),
        create_map(|n| n * (3 * n - 2))
    ];

    let mut result = ~[];
    for vec::each_permutation([0u, 1u, 2u, 3u, 4u]) |idx| {
        for map[5].iter().enumerate().advance |(i, v5)| {
            if i < 10 { loop; }
            for v5.iter().advance |&n5| {
                if n5 < 10 { loop; }
                for map[idx[0]][n5].iter().advance |&n0| {
                    if n0 < 10 { loop; }
                    for map[idx[1]][n0].iter().advance |&n1| {
                        if n1 < 10 { loop; }
                        for map[idx[2]][n1].iter().advance |&n2| {
                            if n2 < 10 { loop; }
                            for map[idx[3]][n2].iter().advance |&n3| {
                                if n3 < 10 { loop; }
                                for map[idx[4]][n3].iter().advance |&n4| {
                                    if n4 < 10 { loop; }
                                    if n4 == i {
                                        result.push(~[
                                            n5 * 100 + n0,
                                            n0 * 100 + n1,
                                            n1 * 100 + n2,
                                            n2 * 100 + n3,
                                            n3 * 100 + n4,
                                            n4 * 100 + n5
                                        ]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let sum = result.iter().transform(|vs| vs.iter().transform(|&x| x).sum()).sum();
    return sum.to_str();
}
