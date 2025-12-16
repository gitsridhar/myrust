fn main() {
    println!("This is a fourth main ! {} " , gcd(10,5))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 54321, 5 * 7 * 54321), 54321);

    let mut x;
    x = 42;
    let y = &mut x;
    //x = 43;
    *y = 43;
    assert_eq!(*y, 43);
    assert_eq!(x, 43);
}
