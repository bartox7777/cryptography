use gcd;
use mod_exp;
use num_primes::{BigUint, Verification};
use rand::seq::SliceRandom;
use rand::thread_rng;

struct RSA {
    e: u128,
    n: u128,
    d: u128,
    // public key: e + n
    // private key: d + n
}

impl RSA {
    fn new(p: u128, q: u128) -> RSA {
        assert!(Verification::is_prime(&BigUint::from(p)));
        assert!(Verification::is_prime(&BigUint::from(q)));

        let n = p * q;
        let phi = (p - 1) * (q - 1);

        // gcd(e, phi) = 1
        let e = (1000..phi)
            .find(|&e| gcd::binary_u128(e, phi) == 1)
            .unwrap();

        // generujemy d w taki sposób, aby spełniona
        // była zależność: iloczyn e i d przystaje do 1
        // modulo phi.

        let mut d = 1;
        while (e * d) % phi != 1 {
            d += 1;
        }
        return RSA { n, e, d };
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

    fn gen_cipher(&self, message: String) -> Vec<u128> {
        let mut cipher: Vec<u128> = Vec::new();
        for chr in message.chars() {
            let chr = chr as u128;
            let c = mod_exp::mod_exp(chr, self.e, self.n);
            cipher.push(c);
        }
        return cipher;
    }

    fn decipher_message(cipher: Vec<u128>, d: u128, n: u128) -> String {
        let mut message = String::new();
        for c in cipher {
            let m = mod_exp::mod_exp(c, d, n);
            message.push(m as u8 as char);
        }
        return message;
    }

    fn gen_p_q(no_primes: u128) -> (u128, u128) {
        let mut primes = RSA::sieve(no_primes);
        let mut res_primes: [u128; 2] = [0; 2];
        for idx in 0..2 {
            res_primes[idx] = primes.remove(idx);
        }
        return (res_primes[0], res_primes[1]);
    }
}

fn main() {
    let (p, q) = RSA::gen_p_q(100000);
    let rsa = RSA::new(p, q);
    println!(
        "p: {}, q: {}, n: {}, e: {}, d: {}",
        p, q, rsa.n, rsa.e, rsa.d
    );
    // 50 characters message
    let message = String::from("Lorem ipsum dolor sit amet consectetur adipiscing.");
    println!("Message: {}", message);

    let cipher = rsa.gen_cipher(message);
    println!("KLIENT1: Cipher: {:?}", cipher);

    println!(
        "KLIENT2: Message: {}",
        RSA::decipher_message(cipher, rsa.d, rsa.n)
    );
}
