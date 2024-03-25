use std::collections::HashSet;

use num_primes::{self, BigUint};
use num_traits::cast::ToPrimitive;

// Diffie-Hellman algorithm
struct DH {
    n: BigUint,
    g: BigUint,
    xy: BigUint,
    XY: BigUint,
    other_XY: Option<BigUint>,
    k: Option<BigUint>,
}

impl DH {
    fn new(n: BigUint, g: BigUint) -> DH {
        let xy = num_primes::Generator::new_prime(16);
        println!("xy: {}", xy);

        let XY = g.modpow(&xy, &n);
        return DH {
            n,
            g,
            xy,
            XY,
            other_XY: None,
            k: None,
        };
    }

    fn create_two_objects(n: BigUint, g: BigUint) -> (DH, DH) {
        let dh1 = DH::new(n.clone(), g.clone());
        let dh2 = DH::new(n, g);
        return (dh1, dh2);
    }

    fn exchange_keys(&mut self, other: &mut DH) {
        self.set_other_XY(&other.XY);
        other.set_other_XY(&self.XY);
    }

    fn set_other_XY(&mut self, other_XY: &BigUint) {
        if self.other_XY.is_some() {
            panic!("other_XY is not None");
        }
        self.other_XY = Some(other_XY.clone());
    }

    fn compute_k(&mut self) {
        if self.k.is_some() {
            panic!("k is not None");
        }
        let other_XY = self.other_XY.as_ref().unwrap();
        self.k = Some(other_XY.modpow(&self.xy, &self.n));
    }

    // https://www.geeksforgeeks.org/primitive-root-of-a-prime-number-n-modulo-n/
    // Given a prime number n, the task is to find its primitive root under modulo n.
    // The primitive root of a prime number n is an integer r between[1, n-1] such that
    // the values of r^x(mod n) where x is in the range[0, n-2] are different.
    fn find_primitive_root(n: &BigUint) -> BigUint {
        let mut hs: HashSet<BigUint> = HashSet::new();
        let mut r: BigUint = BigUint::from(1u8);

        while r < *n {
            hs.clear();
            let mut is_primitive_root = true;
            let mut x = BigUint::from(0u8);
            while x < n - 1u8 {
                hs.insert(r.modpow(&x, &n));
                x += 1u8;
                if hs.len() != x.to_usize().unwrap() {
                    is_primitive_root = false;
                    break;
                }
            }
            if is_primitive_root {
                println!("r: {}", r);
                return r;
            }
            r += 1u8;
        }
        return BigUint::from(0u8);
    }
}

fn main() {
    let some_big_prime = num_primes::Generator::new_prime(16);
    println!("some_big_prime: {}", some_big_prime);
    let primitive_root = DH::find_primitive_root(&some_big_prime);

    let (mut dh1, mut dh2) = DH::create_two_objects(some_big_prime, primitive_root);
    dh1.exchange_keys(&mut dh2);

    dh1.compute_k();
    dh2.compute_k();

    println!("k1: {}", dh1.k.unwrap());
    println!("k2: {}", dh2.k.unwrap());
    // assert_eq!(dh1.k, dh2.k);
}
