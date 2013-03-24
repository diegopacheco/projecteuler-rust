use common::prime::{ Prime };

pub fn solve() -> ~str {
    let mut sum = 0;
    let mut ps = Prime();
    for ps.each |p| {
        if p >= 2000000 {
            break;
        }
        sum += p;
    }
    return sum.to_str();
}