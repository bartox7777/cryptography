use std::collections::HashMap;

use gcd;
use num_primes::{BigUint, Verification};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

struct BBS {
    n: u128,
    salt: u128, // x0
}

impl BBS {
    // p and q are large primes such that p ≡ 3 (mod 4) and q ≡ 3 (mod 4)
    fn new(p: u128, q: u128) -> BBS {
        assert!(Verification::is_prime(&BigUint::from(p)));
        assert!(Verification::is_prime(&BigUint::from(q)));

        assert_eq!(p % 4, 3);
        assert_eq!(q % 4, 3);

        let mut rng = thread_rng();
        let n = p * q;
        let mut salt;

        loop {
            salt = rng.gen_range(0..n);
            if gcd::binary_u128(salt, n) == 1 {
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
            bits.push((x & 1) as u8);
        }
        bits
    }

    // prime numbers in random order
    fn sieve(n: u128) -> Vec<u128> {
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
        primes.shuffle(&mut thread_rng());
        primes
    }

    // generate p and q such that p ≡ 3 (mod 4) and q ≡ 3 (mod 4) and p != q and p,q > 1000
    fn gen_p_q(no_primes: u128) -> (u128, u128) {
        let mut primes = BBS::sieve(no_primes);
        let mut res_primes: [u128; 2] = [0; 2];
        for i in 0..2 {
            let idx = primes.iter().position(|&x| x % 4 == 3 && x > 1000).unwrap();
            res_primes[i] = primes.remove(idx);
        }
        return (res_primes[0], res_primes[1]);
    }

    fn test_half_ones(bits: &Vec<u8>) -> bool {
        let ones = bits.iter().filter(|&x| *x == 1).count();
        dbg!(ones);
        ones > 9725 && ones < 10275
    }

    fn test_series_and_long_series(bits: &Vec<u8>) -> bool {
        let mut ones = 0;
        let mut zeros = 0;
        let mut max_ones = 0;
        let mut max_zeros = 0;
        let mut ones_series_count: HashMap<u8, u32> = HashMap::new();
        let mut zeros_series_count: HashMap<u8, u32> = HashMap::new();
        for i in 0..bits.len() {
            if bits[i] == 1 {
                ones += 1;
                if zeros != 0 {
                    if zeros >= 6 {
                        // 6+
                        zeros = 6
                    }
                    zeros_series_count
                        .insert(zeros, zeros_series_count.get(&zeros).unwrap_or(&0) + 1);
                    zeros = 0;
                }
                if ones > max_ones {
                    max_ones = ones;
                }
            } else {
                zeros += 1;
                if ones != 0 {
                    if ones >= 6 {
                        // 6+
                        ones = 6
                    }
                    ones_series_count.insert(ones, ones_series_count.get(&ones).unwrap_or(&0) + 1);
                    ones = 0;
                }
                if zeros > max_zeros {
                    max_zeros = zeros;
                }
            }
        }

        let check_count = |count: &HashMap<u8, u32>| {
            count.iter().all(|(k, v)| {
                if *k == 1 {
                    *v > 2315 && *v < 2685
                } else if *k == 2 {
                    *v > 1114 && *v < 1386
                } else if *k == 3 {
                    *v > 527 && *v < 723
                } else if *k == 4 {
                    *v > 240 && *v < 384
                } else {
                    // k == 5 || k == 6
                    *v > 103 && *v < 209
                }
            })
        };

        dbg!(max_ones);
        dbg!(max_zeros);
        dbg!(&ones_series_count);
        dbg!(&zeros_series_count);

        max_ones < 26
            && max_zeros < 26
            && check_count(&zeros_series_count)
            && check_count(&ones_series_count)
    }

    fn test_poker(bits: &Vec<u8>) -> bool {
        let mut poker: HashMap<String, u32> = HashMap::new();
        for i in (0..bits.len() - 3).step_by(4) {
            let key = format!("{}{}{}{}", bits[i], bits[i + 1], bits[i + 2], bits[i + 3]);
            let value = poker.get(&key).unwrap_or(&0) + 1;
            poker.insert(key, value);
        }
        let x = (16.0 / 5000.0) * poker.values().map(|x| x * x).sum::<u32>() as f64 - 5000.0;
        dbg!(x);
        dbg!(poker);
        x > 2.16 && x < 46.17
    }
}

fn main() {
    let (p, q) = BBS::gen_p_q(1000000);
    let bbs = BBS::new(p, q);
    println!("p: {}, q: {}, salt: {}, n: {}", p, q, bbs.salt, bbs.n);
    let bits = bbs.generate_bits(20000);

    println!("-----------------");
    if BBS::test_half_ones(&bits) {
        println!("Test pojedynczych bitow przeszedl");
    } else {
        println!("Test pojedynczych bitow nie przeszedl");
    }
    println!("-----------------");

    if BBS::test_series_and_long_series(&bits) {
        println!("Test serii przeszedl");
    } else {
        println!("Test serii nie przeszedl");
    }
    println!("-----------------");

    if BBS::test_poker(&bits) {
        println!("Test pokerowy przeszedl");
    } else {
        println!("Test pokerowy nie przeszedl");
    }
    println!("-----------------");
}

//test - polowa 0 i polowa 1 9725 < '1' < 10275
// test serii - zliczamy '1' obok siebie i '0' obok siebie (osobno)
// seria +26 nie moze sie pojawic
// test pokerowy - 4 bitowe grupy
// wystapienie 0000 jest tak samo prawdopodobne jak 1111
// kazda kombinacja 4 bitowa ma takie samo prawdopodobienstwo i wystapi ~300 razy
