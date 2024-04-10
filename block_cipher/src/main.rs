use std::{
    fs::File,
    io::{Read, Write},
};

use openssl::{
    symm::Cipher,
    symm::{decrypt, encrypt},
};
use rand::Rng;

fn generate_benchmark_file(size_mb: usize) -> String {
    let file_name = format!("{}MB.bin", size_mb); // File name
    let mut file = File::create(file_name.as_str()).unwrap(); // Create file
    let mut rng = rand::thread_rng(); // Random number generator
    let mut buffer = [0u8; 1024]; // Buffer
    for _ in 0..size_mb {
        for _ in 0..1024 {
            rng.fill(&mut buffer); // Fill buffer with random data
            file.write_all(&buffer).unwrap(); // Write buffer to file
        }
    }
    file.sync_all().unwrap(); // Sync file
    file_name // Return file name
}

fn encrypt_file(filename: &String, key: &String, iv: &String, cipher: Cipher) -> Vec<u8> {
    let mut file = File::open(filename).unwrap(); // Open file
    let mut data = Vec::new(); // Data
    file.read_to_end(&mut data).unwrap(); // Read file to data
    return encrypt(cipher, key.as_bytes(), Some(iv.as_bytes()), &data).unwrap();
    // Encrypt data and convert to vector
}

fn main() {
    let filename = generate_benchmark_file(1);
    let key = String::from("0123456789ABCDEF"); // key must be known only to sender and receiver
    let iv = String::from("FEDCBA9876543210"); // iv don't need to be secret

    println!("Szyfrowanie AES 128 CBC");
    let start = std::time::Instant::now();
    encrypt_file(&filename, &key, &iv, Cipher::aes_128_cbc());
    println!("Czas szyfrowania: {:?}", start.elapsed());
}
