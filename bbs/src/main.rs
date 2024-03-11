use gcd;
use num_primes::{BigUint, Verification};
use rand::{thread_rng, Rng};

struct BBS {
    n: u128,
    salt: u128, // x0
}

impl BBS {
    // p and q are large primes such that p ≡ 3 (mod 4) and q ≡ 3 (mod 4)
    fn new(p_and_q: [u128; 2]) -> BBS {
        let p = p_and_q[0];
        let q = p_and_q[1];

        assert!(Verification::is_prime(&BigUint::from(p)));
        assert!(Verification::is_prime(&BigUint::from(q)));

        assert_eq!(p % 4, 3);
        assert_eq!(q % 4, 3);

        let mut rng = thread_rng();
        let n = p * q;
        let salt;

        loop {
            let tmp_salt = rng.gen_range(0..n);
            if gcd::binary_u128(tmp_salt, n) == 1 {
                salt = tmp_salt;
                break;
            }
        }
        BBS { n, salt }
    }

    fn generate_bits(&self, no_bits: u32) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();
        let mut x = self.salt;
        for _ in 0..no_bits {
            x = x.pow(2) % self.n;
            bits.push((x % 2) as u8);
        }
        bits
    }

    // prime numbers in descending order
    fn sieve(n: u32) -> Vec<u128> {
        let mut primes: Vec<u128> = vec![2];
        let mut is_prime = vec![true; n as usize];
        for i in (3..n).step_by(2) {
            if is_prime[i as usize] {
                primes.push(i as u128);
                for j in (i * i..n).step_by(i as usize) {
                    is_prime[j as usize] = false;
                }
            }
        }
        primes.reverse();
        primes
    }

    // generate p and q such that p and q are the largest primes such that p ≡ 3 (mod 4) and q ≡ 3 (mod 4) and p != q
    fn gen_p_q(no_primes: u32) -> [u128; 2] {
        let mut primes = BBS::sieve(no_primes);
        let mut res_primes: [u128; 2] = [0; 2];
        for i in 0..2 {
            let idx = primes.iter().position(|&x| x % 4 == 3).unwrap();
            res_primes[i] = primes.remove(idx);
        }
        return res_primes;
    }
}

fn main() {
    let bbs = BBS::new(BBS::gen_p_q(10000));
    println!("bits: {:?}", bbs.generate_bits(20));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_single_bits() {
        let bbs: BBS = BBS::new(BBS::gen_p_q(20000));
        let bits = bbs.generate_bits(20000);
        // count number of different length sequences of 1s and 0s
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1 - 2, 3);
    }
}

//test - polowa 0 i polowa 1 9725 < '1' < 10275
// test serii - zliczamy '1' obok siebie i '0' obok siebie (osobno)
// seria +26 nie moze sie pojawic
// test pokerowy - 4 bitowe grupy
// wystapienie 0000 jest tak samo prawdopodobne jak 1111
// kazda kombinacja 4 bitowa ma takie samo prawdopodobienstwo i wystapi ~300 razy
